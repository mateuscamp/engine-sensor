use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
    adapters,
    config::{AllowRule, EngineChoice, Profile},
    model::{AnalysisReport, Diagnostic, REPORT_SCHEMA_VERSION},
    scanner::{detect_engine, scan_sources, validate_compatibility},
};

#[derive(Debug, Clone)]
pub struct CheckRequest {
    pub project: PathBuf,
    pub engine: EngineChoice,
    pub profiles: Vec<Profile>,
    pub allow: Vec<AllowRule>,
}

pub fn check_project(request: &CheckRequest) -> Result<AnalysisReport> {
    let engine = detect_engine(&request.project, request.engine)?;
    validate_compatibility(&request.project, engine)?;
    let sources = scan_sources(&request.project, engine)?;
    let mut output = adapters::analyze(engine, &request.project, &sources, &request.profiles)?;
    output.claims.sort_by(|left, right| {
        (
            left.resource.id(),
            left.owner.as_str(),
            left.span.path.as_str(),
            left.span.line,
            left.span.column,
        )
            .cmp(&(
                right.resource.id(),
                right.owner.as_str(),
                right.span.path.as_str(),
                right.span.line,
                right.span.column,
            ))
    });
    output.claims.dedup_by(|left, right| {
        left.resource == right.resource
            && left.owner == right.owner
            && left.span == right.span
            && left.operation == right.operation
    });
    for diagnostic in &mut output.diagnostics {
        diagnostic.related.sort();
        diagnostic.related.dedup();
        diagnostic.owners.sort();
        diagnostic.owners.dedup();
    }
    output
        .diagnostics
        .retain(|diagnostic| !allowed(diagnostic, &request.allow));
    output
        .diagnostics
        .sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
    output.diagnostics.dedup_by(|left, right| {
        left.rule == right.rule
            && left.resource == right.resource
            && left.primary == right.primary
            && left.owners == right.owners
    });
    let mut profiles = request.profiles.clone();
    profiles.sort();
    profiles.dedup();
    Ok(AnalysisReport {
        schema_version: REPORT_SCHEMA_VERSION,
        tool_version: env!("CARGO_PKG_VERSION").to_owned(),
        project: ".".to_owned(),
        engine,
        profiles,
        files_scanned: sources.len(),
        claims: output.claims,
        diagnostics: output.diagnostics,
    })
}

fn allowed(diagnostic: &Diagnostic, allow: &[AllowRule]) -> bool {
    allow.iter().any(|item| {
        let mut owners = item.owners.clone();
        owners.sort();
        owners.dedup();
        item.rule == diagnostic.rule
            && item.resource == diagnostic.resource
            && owners == diagnostic.owners
    })
}

pub fn relative_label(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(".")
        .to_owned()
}
