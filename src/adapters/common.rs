use std::collections::BTreeSet;

use regex::Regex;

use crate::{
    model::{Confidence, Diagnostic, OwnershipClaim, Severity, Span},
    parser::{CallSite, FunctionSite},
};

#[derive(Debug, Clone)]
pub struct Branch {
    pub actions: Vec<String>,
    pub handlers: Vec<(String, Span)>,
}

pub fn normalized_expression(source: &str) -> (String, Confidence) {
    let compact = source.split_whitespace().collect::<String>();
    if compact.is_empty() {
        return ("<vazio>".to_owned(), Confidence::Ambiguous);
    }
    if let Some(literal) = literal_inside(&compact, "hash(") {
        return (literal, Confidence::Proven);
    }
    if (compact.starts_with('"') && compact.ends_with('"'))
        || (compact.starts_with('\'') && compact.ends_with('\''))
    {
        return (
            compact[1..compact.len().saturating_sub(1)].to_owned(),
            Confidence::Proven,
        );
    }
    let stable =
        Regex::new(r"^[%$]?[A-Za-z_][A-Za-z0-9_./:$]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*$").unwrap();
    if stable.is_match(&compact) {
        (compact, Confidence::Proven)
    } else {
        (compact, Confidence::Ambiguous)
    }
}

pub fn normalized_property(source: &str) -> (String, Confidence) {
    let compact = source.split_whitespace().collect::<String>();
    let known = [
        ("gui.PROP_POSITION", "position"),
        ("gui.PROP_SCALE", "scale"),
        ("gui.PROP_COLOR", "color"),
        ("gui.PROP_ROTATION", "rotation"),
        ("go.PROP_POSITION", "position"),
        ("go.PROP_SCALE", "scale"),
        ("go.PROP_EULER", "euler"),
    ];
    if let Some((_, normalized)) = known.iter().find(|(raw, _)| compact == *raw) {
        return ((*normalized).to_owned(), Confidence::Proven);
    }
    let (property, confidence) = normalized_expression(&compact);
    let family = property
        .split(['.', ':'])
        .next()
        .unwrap_or(property.as_str())
        .to_ascii_lowercase();
    (family, confidence)
}

pub fn symbolic_scope(path: &str, owner: &str, target: &str, control_path: &str) -> String {
    let stable_across_functions = target == "."
        || target == "self"
        || target.starts_with("self.")
        || target.starts_with('#')
        || target.starts_with('$')
        || target.starts_with('%')
        || target.contains(":/");
    if stable_across_functions {
        path.to_owned()
    } else {
        let loops = control_path
            .split('/')
            .filter(|part| {
                part.starts_with("for_statement@")
                    || part.starts_with("while_statement@")
                    || part.starts_with("repeat_statement@")
            })
            .collect::<Vec<_>>()
            .join("/");
        if loops.is_empty() {
            format!("{path}::{owner}")
        } else {
            format!("{path}::{owner}::{loops}")
        }
    }
}

fn literal_inside(source: &str, prefix: &str) -> Option<String> {
    let inner = source.strip_prefix(prefix)?.strip_suffix(')')?;
    if (inner.starts_with('"') && inner.ends_with('"'))
        || (inner.starts_with('\'') && inner.ends_with('\''))
    {
        Some(inner[1..inner.len().saturating_sub(1)].to_owned())
    } else {
        None
    }
}

pub fn unresolved_diagnostic(claim: &OwnershipClaim, what: &str) -> Diagnostic {
    Diagnostic {
        rule: "SAR-PARSE-001".to_owned(),
        severity: Severity::Warning,
        resource: claim.resource.id(),
        primary: claim.span.clone(),
        related: Vec::new(),
        owners: vec![claim.owner.clone()],
        explanation: format!(
            "{what} é dinâmico; o Sara registrou a declaração, mas não consegue provar identidade ou sobreposição"
        ),
        remediation: "use um alvo/propriedade textual estável ou mantenha o aviso visível"
            .to_owned(),
    }
}

pub fn conflict_diagnostic(
    rule: &str,
    severity: Severity,
    first: &OwnershipClaim,
    second: &OwnershipClaim,
    explanation: &str,
    remediation: &str,
) -> Diagnostic {
    let mut owners = vec![first.owner.clone(), second.owner.clone()];
    owners.sort();
    owners.dedup();
    Diagnostic {
        rule: rule.to_owned(),
        severity,
        resource: first.resource.id(),
        primary: second.span.clone(),
        related: vec![first.span.clone()],
        owners,
        explanation: explanation.to_owned(),
        remediation: remediation.to_owned(),
    }
}

/// Sintaxe de bloco condicional da linguagem analisada.
///
/// O núcleo compartilhado não sabe qual engine está sendo analisada: o adapter
/// entrega a sintaxe como dado. Isso mantém a semântica de cada engine dentro do
/// seu adapter, que é a fronteira exigida pelo achado A1 de
/// `docs/AUDITORIA-ARQUITETURAL.md` e verificada por `tests/governanca.rs`.
#[derive(Debug, Clone, Copy)]
pub struct BlockSyntax {
    /// Prefixos de linha que abrem um ramo condicional.
    pub opens_branch: &'static [&'static str],
    /// Como reconhecer que a condição terminou e o corpo começou.
    pub condition_end: ConditionEnd,
    /// Prefixos de linha que encerram o corpo de um ramo.
    pub closes_body_prefix: &'static [&'static str],
    /// Linhas exatas que encerram o corpo de um ramo.
    pub closes_body_exact: &'static [&'static str],
}

/// Marca que separa a condição do corpo do ramo.
#[derive(Debug, Clone, Copy)]
pub enum ConditionEnd {
    /// A condição termina na primeira linha que termina com este caractere.
    LineEndsWith(char),
    /// A condição termina na primeira linha que contém esta palavra.
    LineContains(&'static str),
}

impl BlockSyntax {
    fn opens(&self, trimmed: &str) -> bool {
        self.opens_branch
            .iter()
            .any(|prefix| trimmed.starts_with(prefix))
    }

    fn condition_complete(&self, line: &str) -> bool {
        match self.condition_end {
            ConditionEnd::LineEndsWith(marker) => line.trim_end().ends_with(marker),
            ConditionEnd::LineContains(word) => line.contains(word),
        }
    }

    fn closes(&self, trimmed: &str) -> bool {
        self.closes_body_prefix
            .iter()
            .any(|prefix| trimmed.starts_with(prefix))
            || self.closes_body_exact.contains(&trimmed)
    }
}

pub fn action_branches(
    function: &FunctionSite,
    calls: &[CallSite],
    action_regex: &Regex,
    syntax: BlockSyntax,
) -> Vec<Branch> {
    let lines = function.text.lines().collect::<Vec<_>>();
    let mut branches = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let trimmed = lines[index].trim_start();
        if !syntax.opens(trimmed) {
            index += 1;
            continue;
        }

        let indent = indentation(lines[index]);
        let condition_start = index;
        let mut condition_end = index;
        while condition_end + 1 < lines.len() && !syntax.condition_complete(lines[condition_end]) {
            condition_end += 1;
        }
        let condition = lines[condition_start..=condition_end].join("\n");
        let actions = action_regex
            .captures_iter(&condition)
            .map(|captures| captures[1].to_owned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        if actions.is_empty() {
            index = condition_end + 1;
            continue;
        }

        let body_start = condition_end + 1;
        let mut body_end = lines.len();
        for (candidate, line) in lines.iter().enumerate().skip(body_start) {
            if syntax.closes(line.trim_start()) && indentation(line) <= indent {
                body_end = candidate;
                break;
            }
        }
        let absolute_start = function.start_line + body_start as u32;
        let absolute_end = function.start_line + body_end as u32;
        let handlers = calls
            .iter()
            .filter(|call| {
                call.owner == function.name
                    && call.span.line >= absolute_start
                    && call.span.line < absolute_end
                    && effect_handler(&call.callee)
            })
            .map(|call| (call.callee.clone(), call.span.clone()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        branches.push(Branch { actions, handlers });
        index = condition_end + 1;
    }
    branches
}

fn effect_handler(callee: &str) -> bool {
    let simple = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap();
    simple.is_match(callee)
        && !matches!(
            callee,
            "hash"
                | "ipairs"
                | "pairs"
                | "print"
                | "assert"
                | "len"
                | "range"
                | "min"
                | "max"
                | "abs"
        )
}

fn indentation(line: &str) -> usize {
    line.chars()
        .take_while(|character| matches!(character, ' ' | '\t'))
        .map(|character| if character == '\t' { 4 } else { 1 })
        .sum()
}
