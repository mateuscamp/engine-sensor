use std::fmt;

use serde::{Deserialize, Serialize};

use crate::config::Profile;

pub const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Godot,
    Defold,
}

impl fmt::Display for Engine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Godot => write!(formatter, "godot"),
            Self::Defold => write!(formatter, "defold"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Proven,
    Ambiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    AnimationProperty,
    InputEffect,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Span {
    pub path: String,
    pub line: u32,
    pub column: u32,
}

impl Span {
    pub fn label(&self) -> String {
        format!("{}:{}:{}", self.path, self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceKey {
    pub engine: Engine,
    pub kind: ResourceKind,
    pub scope: String,
    pub target: String,
    pub property: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
}

impl ResourceKey {
    pub fn id(&self) -> String {
        let kind = match self.kind {
            ResourceKind::AnimationProperty => "animation",
            ResourceKind::InputEffect => "input",
        };
        let profile = self
            .profile
            .map(|value| format!(":{value}"))
            .unwrap_or_default();
        format!(
            "{kind}:{}:{}:{}:{}{profile}",
            self.engine, self.scope, self.target, self.property
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipClaim {
    pub resource: ResourceKey,
    pub owner: String,
    pub span: Span,
    pub confidence: Confidence,
    pub operation: String,
    pub controller: String,
    pub flow: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub rule: String,
    pub severity: Severity,
    pub resource: String,
    pub primary: Span,
    pub related: Vec<Span>,
    pub owners: Vec<String>,
    pub explanation: String,
    pub remediation: String,
}

impl Diagnostic {
    pub fn sort_key(&self) -> (&str, &str, &str, u32, u32) {
        (
            self.rule.as_str(),
            self.resource.as_str(),
            self.primary.path.as_str(),
            self.primary.line,
            self.primary.column,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub schema_version: u32,
    pub tool_version: String,
    pub project: String,
    pub engine: Engine,
    pub profiles: Vec<Profile>,
    pub files_scanned: usize,
    pub claims: Vec<OwnershipClaim>,
    pub diagnostics: Vec<Diagnostic>,
}

impl AnalysisReport {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|item| item.severity == Severity::Error)
    }
}
