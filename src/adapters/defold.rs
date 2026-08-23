use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use regex::Regex;

use crate::{
    adapters::{AdapterOutput, common},
    config::Profile,
    model::{Confidence, Diagnostic, Engine, OwnershipClaim, ResourceKey, ResourceKind, Severity},
    parser::ParsedSource,
};

#[derive(Debug, Clone)]
struct CancelSite {
    resource: ResourceKey,
    owner: String,
    line: u32,
    control_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Channel {
    Mouse,
    Touch,
    Other,
}

pub fn analyze(
    project: &Path,
    sources: &[ParsedSource],
    profiles: &[Profile],
) -> Result<AdapterOutput> {
    let mut output = AdapterOutput::default();
    let mut cancels = Vec::new();
    for source in sources {
        animation_claims(source, &mut output, &mut cancels);
    }
    diagnose_animations(&output.claims, &cancels, sources, &mut output.diagnostics);
    input_claims(project, sources, profiles, &mut output)?;
    Ok(output)
}

fn animation_claims(
    source: &ParsedSource,
    output: &mut AdapterOutput,
    cancels: &mut Vec<CancelSite>,
) {
    for call in &source.calls {
        let animate = matches!(call.callee.as_str(), "go.animate" | "gui.animate");
        let cancel = matches!(
            call.callee.as_str(),
            "go.cancel_animations" | "gui.cancel_animations"
        );
        if (!animate && !cancel) || call.args.len() < 2 {
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
        let resource = ResourceKey {
            engine: Engine::Defold,
            kind: ResourceKind::AnimationProperty,
            scope: common::symbolic_scope(&source.path, &call.owner, &target, &call.control_path),
            target,
            property,
            profile: None,
        };
        if cancel {
            cancels.push(CancelSite {
                resource,
                owner: format!("{}::{}", source.path, call.owner),
                line: call.span.line,
                control_path: call.control_path.clone(),
            });
            continue;
        }
        let claim = OwnershipClaim {
            resource,
            owner: format!("{}::{}", source.path, call.owner),
            span: call.span.clone(),
            confidence,
            operation: call.callee.clone(),
            controller: if call.inside_loop {
                format!("loop:{}", call.control_path)
            } else {
                call.control_path.clone()
            },
            flow: completion_parent(call, &source.calls).map_or_else(
                || call.control_path.clone(),
                |line| format!("{}|completion_of:{line}", call.control_path),
            ),
        };
        if confidence == Confidence::Ambiguous {
            output.diagnostics.push(common::unresolved_diagnostic(
                &claim,
                "o alvo ou a propriedade da animação",
            ));
        }
        output.claims.push(claim);
    }
}

fn diagnose_animations(
    all_claims: &[OwnershipClaim],
    cancels: &[CancelSite],
    sources: &[ParsedSource],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut groups: BTreeMap<ResourceKey, Vec<&OwnershipClaim>> = BTreeMap::new();
    for claim in all_claims.iter().filter(|item| {
        item.resource.engine == Engine::Defold
            && item.resource.kind == ResourceKind::AnimationProperty
            && item.confidence == Confidence::Proven
    }) {
        groups
            .entry(claim.resource.clone())
            .or_default()
            .push(claim);
    }
    for (resource, claims) in groups.iter_mut() {
        claims.sort_by_key(|item| item.span.line);
        let mut warned_between_owners = false;
        for pair in claims.windows(2) {
            let first = pair[0];
            let second = pair[1];
            if is_completion_of(second, first) {
                continue;
            }
            if mutually_exclusive_init(first, second, sources) {
                continue;
            }
            if cancelled_pair(resource, first, second, cancels) {
                continue;
            }
            if first.owner == second.owner && first.controller == second.controller {
                if first.controller.starts_with("loop:") {
                    if !warned_between_owners {
                        diagnostics.push(common::conflict_diagnostic(
                            "SAR-OWN-001",
                            Severity::Warning,
                            first,
                            second,
                            "a mesma região de repetição anima simbolicamente o mesmo alvo; a análise não prova que duas iterações alcançam a mesma instância",
                            "use uma identidade de alvo estável ou documente por que as iterações são disjuntas",
                        ));
                        warned_between_owners = true;
                    }
                    continue;
                }
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-001",
                    Severity::Error,
                    first,
                    second,
                    "duas animações da mesma função começam sobre a mesma propriedade sem cancelamento entre elas",
                    "mantenha uma animação proprietária, componha os valores em uma chamada ou cancele explicitamente antes da segunda",
                ));
            } else if !warned_between_owners {
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-001",
                    Severity::Warning,
                    first,
                    second,
                    "duas trajetórias de execução podem animar a mesma propriedade, mas a análise estática não prova que seus ciclos se sobrepõem",
                    "centralize a animação ou documente uma exceção exata se os ciclos forem mutuamente exclusivos",
                ));
                warned_between_owners = true;
            }
        }
    }
}

fn completion_parent(
    call: &crate::parser::CallSite,
    calls: &[crate::parser::CallSite],
) -> Option<u32> {
    calls
        .iter()
        .filter(|parent| {
            matches!(parent.callee.as_str(), "go.animate" | "gui.animate")
                && parent.text.contains("function")
                && parent.span.line < call.span.line
                && parent.end_line >= call.end_line
        })
        .min_by_key(|parent| parent.end_line.saturating_sub(parent.span.line))
        .map(|parent| parent.span.line)
}

fn is_completion_of(candidate: &OwnershipClaim, parent: &OwnershipClaim) -> bool {
    candidate
        .flow
        .split('|')
        .any(|part| part == format!("completion_of:{}", parent.span.line))
}

fn mutually_exclusive_init(
    first: &OwnershipClaim,
    second: &OwnershipClaim,
    sources: &[ParsedSource],
) -> bool {
    if first.span.path != second.span.path {
        return false;
    }
    let Some(source) = sources.iter().find(|source| source.path == first.span.path) else {
        return false;
    };
    let first_line = completion_root_line(first).unwrap_or(first.span.line);
    let second_line = completion_root_line(second).unwrap_or(second.span.line);
    let (early, late) = if first_line <= second_line {
        (first_line, second_line)
    } else {
        (second_line, first_line)
    };
    let in_init = source.functions.iter().any(|function| {
        function.name == "init" && early >= function.start_line && late <= function.end_line
    });
    if !in_init {
        return false;
    }
    branch_returns_before(&source.source, early, late)
}

fn completion_root_line(claim: &OwnershipClaim) -> Option<u32> {
    claim
        .flow
        .split('|')
        .find_map(|part| part.strip_prefix("completion_of:"))?
        .parse()
        .ok()
}

fn branch_returns_before(source: &str, early: u32, late: u32) -> bool {
    let lines = source.lines().collect::<Vec<_>>();
    let early_index = early.saturating_sub(1) as usize;
    let late_index = late.saturating_sub(1) as usize;
    let Some(early_line) = lines.get(early_index) else {
        return false;
    };
    let early_indent = indentation(early_line);
    let Some((branch_index, branch_indent)) = (0..early_index).rev().find_map(|index| {
        let line = lines[index];
        let indent = indentation(line);
        (line.trim_start().starts_with("if ") && indent < early_indent).then_some((index, indent))
    }) else {
        return false;
    };

    let mut returned = false;
    for line in lines.iter().take(late_index).skip(branch_index + 1) {
        let trimmed = line.trim();
        let indent = indentation(line);
        if trimmed.starts_with("return") && indent == early_indent {
            returned = true;
        }
        if trimmed == "end" && indent == branch_indent {
            return returned;
        }
    }
    false
}

fn indentation(line: &str) -> usize {
    line.chars()
        .take_while(|character| matches!(character, ' ' | '\t'))
        .map(|character| if character == '\t' { 4 } else { 1 })
        .sum()
}

fn cancelled_pair(
    resource: &ResourceKey,
    first: &OwnershipClaim,
    second: &OwnershipClaim,
    cancels: &[CancelSite],
) -> bool {
    cancels.iter().any(|cancel| {
        if cancel.resource != *resource {
            return false;
        }
        let between = cancel.owner == second.owner
            && cancel.line > first.span.line
            && cancel.line < second.span.line
            && path_dominates(&cancel.control_path, &second.flow);
        let before_branches = first.owner == second.owner
            && cancel.owner == first.owner
            && first.flow != second.flow
            && cancel.line < first.span.line
            && path_dominates(&cancel.control_path, &first.flow)
            && path_dominates(&cancel.control_path, &second.flow);
        between || before_branches
    })
}

fn path_dominates(prefix: &str, path: &str) -> bool {
    prefix.is_empty()
        || prefix == path
        || path
            .strip_prefix(prefix)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn input_claims(
    project: &Path,
    sources: &[ParsedSource],
    profiles: &[Profile],
    output: &mut AdapterOutput,
) -> Result<()> {
    if !profiles.contains(&Profile::Android) {
        return Ok(());
    }
    let Some(binding_path) = binding_path(project)? else {
        return Ok(());
    };
    let bindings = parse_bindings(&binding_path)?;
    let relative_binding = binding_path
        .strip_prefix(project)
        .unwrap_or(&binding_path)
        .to_string_lossy()
        .replace('\\', "/");
    let action_regex = Regex::new(r#"action_id\s*==\s*hash\(\s*["']([^"']+)["']\s*\)"#).unwrap();
    let mut effects: BTreeMap<String, Vec<OwnershipClaim>> = BTreeMap::new();
    for source in sources {
        for function in source
            .functions
            .iter()
            .filter(|item| item.name == "on_input")
        {
            for branch in common::action_branches(function, &source.calls, &action_regex, false) {
                for action in branch.actions {
                    for (handler, span) in &branch.handlers {
                        let resource = ResourceKey {
                            engine: Engine::Defold,
                            kind: ResourceKind::InputEffect,
                            scope: relative_binding.clone(),
                            target: handler.clone(),
                            property: "normalized_action".to_owned(),
                            profile: Some(Profile::Android),
                        };
                        let claim = OwnershipClaim {
                            resource,
                            owner: format!("{}::on_input[{action}]", source.path),
                            span: span.clone(),
                            confidence: Confidence::Proven,
                            operation: action.clone(),
                            controller: handler.clone(),
                            flow: String::new(),
                        };
                        effects
                            .entry(handler.clone())
                            .or_default()
                            .push(claim.clone());
                        output.claims.push(claim);
                    }
                }
            }
        }
    }

    for claims in effects.values_mut() {
        claims.sort_by(|left, right| left.owner.cmp(&right.owner));
        for left_index in 0..claims.len() {
            for right_index in left_index + 1..claims.len() {
                let first = &claims[left_index];
                let second = &claims[right_index];
                if first.operation == second.operation {
                    continue;
                }
                let first_channels = bindings.get(&first.operation).cloned().unwrap_or_default();
                let second_channels = bindings.get(&second.operation).cloned().unwrap_or_default();
                if physical_duplicate(&first_channels, &second_channels) {
                    output.diagnostics.push(common::conflict_diagnostic(
                        "SAR-OWN-002",
                        Severity::Error,
                        first,
                        second,
                        "mouse e multitoque usam ações diferentes que alcançam diretamente o mesmo efeito no perfil Android",
                        "normalize os dois canais em uma única ação antes de chamar o efeito ou deduplique explicitamente no adaptador de entrada",
                    ));
                }
            }
        }
    }
    Ok(())
}

fn binding_path(project: &Path) -> Result<Option<PathBuf>> {
    let game_project = project.join("game.project");
    let source = fs::read_to_string(&game_project)
        .with_context(|| format!("não consegui ler {}", game_project.display()))?;
    let expression = Regex::new(r"(?m)^game_binding\s*=\s*/?([^\s]+)$").unwrap();
    let Some(captures) = expression.captures(&source) else {
        return Ok(None);
    };
    let mut relative = captures[1].to_owned();
    if relative.ends_with(".input_bindingc") {
        relative.pop();
    }
    let path = project.join(relative);
    Ok(path.is_file().then_some(path))
}

fn parse_bindings(path: &Path) -> Result<BTreeMap<String, BTreeSet<Channel>>> {
    let source =
        fs::read_to_string(path).with_context(|| format!("não consegui ler {}", path.display()))?;
    let block = Regex::new(r"(?s)(mouse_trigger|touch_trigger|key_trigger)\s*\{(.*?)\}").unwrap();
    let action = Regex::new(r#"action\s*:\s*["']([^"']+)["']"#).unwrap();
    let mut bindings: BTreeMap<String, BTreeSet<Channel>> = BTreeMap::new();
    for captures in block.captures_iter(&source) {
        let Some(action_capture) = action.captures(&captures[2]) else {
            continue;
        };
        let channel = match &captures[1] {
            "mouse_trigger" => Channel::Mouse,
            "touch_trigger" => Channel::Touch,
            _ => Channel::Other,
        };
        bindings
            .entry(action_capture[1].to_owned())
            .or_default()
            .insert(channel);
    }
    Ok(bindings)
}

fn physical_duplicate(first: &BTreeSet<Channel>, second: &BTreeSet<Channel>) -> bool {
    (first.contains(&Channel::Mouse) && second.contains(&Channel::Touch))
        || (first.contains(&Channel::Touch) && second.contains(&Channel::Mouse))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{Channel, physical_duplicate};

    #[test]
    fn mouse_and_touch_are_duplicate_on_android() {
        assert!(physical_duplicate(
            &BTreeSet::from([Channel::Mouse]),
            &BTreeSet::from([Channel::Touch])
        ));
        assert!(!physical_duplicate(
            &BTreeSet::from([Channel::Other]),
            &BTreeSet::from([Channel::Touch])
        ));
    }
}
