mod common;
mod defold;
mod godot;

use std::path::Path;

use anyhow::Result;

use crate::{
    config::Profile,
    model::{Diagnostic, Engine, OwnershipClaim},
    parser::ParsedSource,
};

#[derive(Debug, Default)]
pub struct AdapterOutput {
    pub claims: Vec<OwnershipClaim>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn analyze(
    engine: Engine,
    project: &Path,
    sources: &[ParsedSource],
    profiles: &[Profile],
) -> Result<AdapterOutput> {
    match engine {
        Engine::Godot => godot::analyze(project, sources, profiles),
        Engine::Defold => defold::analyze(project, sources, profiles),
    }
}
