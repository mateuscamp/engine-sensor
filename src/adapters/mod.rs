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

/// Eixo de posse que uma construção de API afeta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Animation,
    Input,
    Depth,
}

impl Axis {
    /// Nome do eixo como o contrato publicado o escreve.
    pub fn label(self) -> &'static str {
        match self {
            Axis::Animation => "animação",
            Axis::Input => "entrada",
            Axis::Depth => "profundidade",
        }
    }
}

/// Construção de API que um adapter reconhece.
///
/// A lista é pública porque ela **é** o contrato publicado em
/// `docs/COMPATIBILIDADE.md`. `tests/governanca.rs` compara os dois nos dois
/// sentidos: o documento não promete o que o código não faz, e não cala o que ele
/// faz. Achado A7 de `docs/AUDITORIA-ARQUITETURAL.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Construct {
    pub engine: Engine,
    pub axis: Axis,
    /// Token que o adapter procura no fonte analisado, escrito como aparece no código.
    pub token: &'static str,
}

/// Todas as construções reconhecidas, na ordem em que cada adapter as declara.
pub fn recognized_constructs() -> Vec<Construct> {
    godot::CONSTRUCTS
        .iter()
        .chain(defold::CONSTRUCTS)
        .copied()
        .collect()
}

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
