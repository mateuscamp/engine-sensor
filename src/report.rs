use anyhow::Result;

use crate::model::{AnalysisReport, Severity};

pub fn json(report: &AnalysisReport) -> Result<String> {
    Ok(serde_json::to_string_pretty(report)?)
}

pub fn text(report: &AnalysisReport) -> String {
    let errors = report
        .diagnostics
        .iter()
        .filter(|item| item.severity == Severity::Error)
        .count();
    let warnings = report.diagnostics.len() - errors;
    let mut lines = vec![format!(
        "Sara {} - {} arquivo(s), {} declaração(ões), {} erro(s), {} aviso(s)",
        report.tool_version,
        report.files_scanned,
        report.claims.len(),
        errors,
        warnings
    )];
    for diagnostic in &report.diagnostics {
        let severity = match diagnostic.severity {
            Severity::Error => "ERRO",
            Severity::Warning => "AVISO",
        };
        lines.push(format!(
            "\n{severity} {} {}\n  {}\n  recurso: {}\n  correção: {}",
            diagnostic.rule,
            diagnostic.primary.label(),
            diagnostic.explanation,
            diagnostic.resource,
            diagnostic.remediation
        ));
        for related in &diagnostic.related {
            lines.push(format!("  relacionado: {}", related.label()));
        }
    }
    lines.join("\n")
}
