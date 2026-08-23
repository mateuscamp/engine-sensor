use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result};
use regex::Regex;

use crate::{
    adapters::{AdapterOutput, common},
    config::Profile,
    model::{Confidence, Diagnostic, Engine, OwnershipClaim, ResourceKey, ResourceKind, Severity},
    parser::ParsedSource,
};

pub fn analyze(
    project: &Path,
    sources: &[ParsedSource],
    _profiles: &[Profile],
) -> Result<AdapterOutput> {
    let declared_actions = declared_actions(project)?;
    let mut output = AdapterOutput::default();
    for source in sources {
        animation_claims(source, &mut output);
        input_claims(source, &declared_actions, &mut output);
    }
    diagnose_animations(&output.claims, sources, &mut output.diagnostics);
    diagnose_inputs(&output.claims, sources, &mut output.diagnostics);
    Ok(output)
}

/// Sintaxe de bloco do GDScript. Fica aqui, e não no núcleo compartilhado: o
/// `common` não deve saber qual engine está analisando (achado A1).
const GDSCRIPT_BLOCKS: common::BlockSyntax = common::BlockSyntax {
    opens_branch: &["if ", "elif "],
    condition_end: common::ConditionEnd::LineEndsWith(':'),
    closes_body_prefix: &["elif "],
    closes_body_exact: &["else:"],
};

fn animation_claims(source: &ParsedSource, output: &mut AdapterOutput) {
    let assignments = tween_assignments(source);
    let naked_tween =
        Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)\s*\.\s*tween_property\s*\(").expect("regex válida");
    for call in &source.calls {
        let callee = if call.callee.ends_with(".tween_property") {
            call.callee.clone()
        } else if call.callee == "tween_property" {
            let line = source
                .source
                .lines()
                .nth(call.span.line.saturating_sub(1) as usize)
                .unwrap_or_default();
            let column = call.span.column.saturating_sub(1) as usize;
            let Some(captures) = naked_tween.captures_iter(line).find(|captures| {
                captures
                    .get(0)
                    .is_some_and(|matched| matched.start() <= column && column < matched.end())
            }) else {
                continue;
            };
            format!("{}.tween_property", &captures[1])
        } else {
            continue;
        };
        if call.args.len() < 2 {
            continue;
        }
        let (target, target_confidence) = common::normalized_expression(&call.args[0]);
        let (property, property_confidence) = common::normalized_property(&call.args[1]);
        let confidence = if target_confidence == Confidence::Proven
            && property_confidence == Confidence::Proven
        {
            Confidence::Proven
        } else {
            Confidence::Ambiguous
        };
        let prefix = callee.strip_suffix(".tween_property").unwrap_or(&callee);
        let controller = assignments
            .get(&(call.owner.clone(), prefix.to_owned()))
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "{}::{}::{}@{}",
                    source.path, call.owner, prefix, call.span.line
                )
            });
        let claim = OwnershipClaim {
            resource: ResourceKey {
                engine: Engine::Godot,
                kind: ResourceKind::AnimationProperty,
                scope: common::symbolic_scope(
                    &source.path,
                    &call.owner,
                    &target,
                    &call.control_path,
                ),
                target,
                property,
                profile: None,
            },
            owner: format!("{}::{}", source.path, call.owner),
            span: call.span.clone(),
            confidence,
            operation: "Tween.tween_property".to_owned(),
            controller,
            flow: call.control_path.clone(),
        };
        if confidence == Confidence::Ambiguous {
            output.diagnostics.push(common::unresolved_diagnostic(
                &claim,
                "o alvo ou a propriedade do Tween",
            ));
        }
        output.claims.push(claim);
    }
}

fn tween_assignments(source: &ParsedSource) -> BTreeMap<(String, String), String> {
    let expression = Regex::new(
        r"(?m)^\s*(?:var\s+)?([A-Za-z_][A-Za-z0-9_]*)(?:\s*:\s*[A-Za-z_][A-Za-z0-9_.]*)?\s*(?::=|=)\s*(?:[A-Za-z_][A-Za-z0-9_.]*\.)?create_tween\(\)(?:\.[A-Za-z_][A-Za-z0-9_]*\([^\n)]*\))*",
    )
    .unwrap();
    let mut assignments = BTreeMap::new();
    for captures in expression.captures_iter(&source.source) {
        let line = source.source[..captures.get(0).unwrap().start()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count() as u32
            + 1;
        let owner = source
            .functions
            .iter()
            .find(|function| line >= function.start_line && line <= function.end_line)
            .map(|function| function.name.clone())
            .unwrap_or_else(|| "<arquivo>".to_owned());
        let variable = captures[1].to_owned();
        assignments.insert(
            (owner.clone(), variable.clone()),
            format!("{}::{owner}::{variable}@{line}", source.path),
        );
    }
    assignments
}

fn diagnose_animations(
    all_claims: &[OwnershipClaim],
    sources: &[ParsedSource],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut groups: BTreeMap<ResourceKey, Vec<&OwnershipClaim>> = BTreeMap::new();
    for claim in all_claims.iter().filter(|item| {
        item.resource.engine == Engine::Godot
            && item.resource.kind == ResourceKind::AnimationProperty
            && item.confidence == Confidence::Proven
    }) {
        groups
            .entry(claim.resource.clone())
            .or_default()
            .push(claim);
    }
    for claims in groups.values_mut() {
        claims.sort_by_key(|item| item.span.line);
        let mut warned_between_owners = false;
        for pair in claims.windows(2) {
            let first = pair[0];
            let second = pair[1];
            if first.owner == second.owner
                && first.flow == second.flow
                && first.controller != second.controller
                && !has_ordering_barrier(first, second, sources)
            {
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-001",
                    Severity::Error,
                    first,
                    second,
                    "dois Tweens distintos começam na mesma função sobre a mesma propriedade",
                    "use um único Tween sequencial, centralize o proprietário ou encerre o Tween anterior antes de criar o próximo",
                ));
            } else if (first.owner != second.owner || first.flow != second.flow)
                && !warned_between_owners
            {
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-001",
                    Severity::Warning,
                    first,
                    second,
                    "duas trajetórias podem animar a mesma propriedade, mas seus ciclos de vida não são prováveis apenas pelo texto",
                    "centralize o proprietário ou registre uma exceção exata se os ciclos forem mutuamente exclusivos",
                ));
                warned_between_owners = true;
            }
        }
    }
}

fn has_ordering_barrier(
    first: &OwnershipClaim,
    second: &OwnershipClaim,
    sources: &[ParsedSource],
) -> bool {
    let Some(source) = sources.iter().find(|item| item.path == first.span.path) else {
        return false;
    };
    let variable = first
        .controller
        .split("::")
        .last()
        .unwrap_or_default()
        .split('@')
        .next()
        .unwrap_or_default();
    let start = first.span.line.saturating_sub(1) as usize;
    let end = second.span.line as usize;
    let between = source
        .source
        .lines()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect::<Vec<_>>()
        .join("\n");
    between.contains(&format!("await {variable}.finished"))
        || between.contains(&format!("{variable}.kill()"))
}

fn input_claims(
    source: &ParsedSource,
    declared_actions: &BTreeSet<String>,
    output: &mut AdapterOutput,
) {
    let action_regex = Regex::new(
        r#"(?:is_action|is_action_pressed|is_action_released)\s*\(\s*&?["']([^"']+)["']"#,
    )
    .unwrap();
    for function in source.functions.iter().filter(|item| {
        matches!(
            item.name.as_str(),
            "_input" | "_unhandled_input" | "_gui_input"
        )
    }) {
        for branch in
            common::action_branches(function, &source.calls, &action_regex, GDSCRIPT_BLOCKS)
        {
            for action in branch.actions {
                for (handler, span) in &branch.handlers {
                    let declared = declared_actions.contains(&action);
                    let claim = OwnershipClaim {
                        resource: ResourceKey {
                            engine: Engine::Godot,
                            kind: ResourceKind::InputEffect,
                            scope: source.path.clone(),
                            target: handler.clone(),
                            property: action.clone(),
                            profile: None,
                        },
                        owner: format!("{}::{}[{action}]", source.path, function.name),
                        span: span.clone(),
                        confidence: if declared {
                            Confidence::Proven
                        } else {
                            Confidence::Ambiguous
                        },
                        operation: function.name.clone(),
                        controller: handler.clone(),
                        flow: String::new(),
                    };
                    if !declared {
                        output.diagnostics.push(Diagnostic {
                            rule: "SAR-PARSE-001".to_owned(),
                            severity: Severity::Warning,
                            resource: claim.resource.id(),
                            primary: span.clone(),
                            related: Vec::new(),
                            owners: vec![claim.owner.clone()],
                            explanation: format!(
                                "a ação `{action}` é consultada no código, mas não está definida na seção [input] de project.godot"
                            ),
                            remediation: "declare a ação em project.godot ou remova a consulta obsoleta"
                                .to_owned(),
                        });
                    }
                    output.claims.push(claim);
                }
            }
        }
    }
}

fn diagnose_inputs(
    all_claims: &[OwnershipClaim],
    sources: &[ParsedSource],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut groups: BTreeMap<ResourceKey, Vec<&OwnershipClaim>> = BTreeMap::new();
    for claim in all_claims.iter().filter(|item| {
        item.resource.engine == Engine::Godot
            && item.resource.kind == ResourceKind::InputEffect
            && item.confidence == Confidence::Proven
    }) {
        groups
            .entry(claim.resource.clone())
            .or_default()
            .push(claim);
    }
    for claims in groups.values_mut() {
        claims.sort_by(|left, right| left.owner.cmp(&right.owner));
        for pair in claims.windows(2) {
            let first = pair[0];
            let second = pair[1];
            if first.operation == second.operation {
                continue;
            }
            let operations = BTreeMap::from([
                (first.operation.as_str(), first),
                (second.operation.as_str(), second),
            ]);
            if let (Some(input), Some(unhandled)) =
                (operations.get("_input"), operations.get("_unhandled_input"))
            {
                if input_marks_handled(input, sources) {
                    continue;
                }
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-002",
                    Severity::Error,
                    input,
                    unhandled,
                    "_input e _unhandled_input encaminham a mesma ação ao mesmo efeito sem marcar o evento como tratado",
                    "escolha um único proprietário ou marque o evento como tratado antes que ele alcance o segundo callback",
                ));
            } else {
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-002",
                    Severity::Warning,
                    first,
                    second,
                    "dois callbacks de entrada encaminham a mesma ação ao mesmo efeito, mas a propagação depende da árvore de cena",
                    "centralize a entrada ou prove e documente onde o evento é consumido",
                ));
            }
        }
    }
}

fn input_marks_handled(claim: &OwnershipClaim, sources: &[ParsedSource]) -> bool {
    let Some(source) = sources.iter().find(|item| item.path == claim.span.path) else {
        return false;
    };
    source
        .functions
        .iter()
        .find(|function| function.name == "_input")
        .is_some_and(|function| function.text.contains("set_input_as_handled"))
}

fn declared_actions(project: &Path) -> Result<BTreeSet<String>> {
    let path = project.join("project.godot");
    let source = fs::read_to_string(&path)
        .with_context(|| format!("não consegui ler {}", path.display()))?;
    let section = source
        .split("\n[input]\n")
        .nth(1)
        .unwrap_or_default()
        .split("\n[")
        .next()
        .unwrap_or_default();
    let expression = Regex::new(r"(?m)^([^=\r\n]+?)\s*=\s*\{").unwrap();
    Ok(expression
        .captures_iter(section)
        .map(|captures| captures[1].trim().trim_matches('"').to_owned())
        .collect())
}
