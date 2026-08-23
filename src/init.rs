use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::{config::EngineChoice, scanner::detect_engine};

const CONTRACT: &str = include_str!("../kit/CONTRATO.md");
const AGENTS_FRAGMENT: &str = include_str!("../kit/AGENTS.fragment.md");
const CLAUDE_FRAGMENT: &str = include_str!("../kit/CLAUDE.fragment.md");
const EVIDENCE: &str = include_str!("../kit/EVIDENCIA.md");
const PATTERNS: &str = include_str!("../kit/PADROES.md");
const USES: &str = include_str!("../kit/USOS.md");
const GODOT_GATE: &str = include_str!("../kit/godot/portao_ai_first.gd");
const GODOT_PATTERNS: &str = include_str!("../kit/godot/padroes_ai_first.gd");
const DEFOLD_GATE: &str = include_str!("../kit/defold/portao_ai_first.lua");
const DEFOLD_PATTERNS: &str = include_str!("../kit/defold/padroes_ai_first.lua");

pub fn initialize_project(project: &Path, choice: EngineChoice) -> Result<Vec<String>> {
    let engine = detect_engine(project, choice)?;
    let sara = project.join(".sara");
    fs::create_dir_all(&sara).with_context(|| format!("não consegui criar {}", sara.display()))?;
    let engine_dir = sara.join(engine.to_string());
    fs::create_dir_all(&engine_dir)
        .with_context(|| format!("não consegui criar {}", engine_dir.display()))?;

    let mut created = Vec::new();
    write_new(&sara.join("CONTRATO.md"), CONTRACT, project, &mut created)?;
    write_new(
        &sara.join("AGENTS.fragment.md"),
        AGENTS_FRAGMENT,
        project,
        &mut created,
    )?;
    write_new(
        &sara.join("CLAUDE.fragment.md"),
        CLAUDE_FRAGMENT,
        project,
        &mut created,
    )?;
    write_new(&sara.join("EVIDENCIA.md"), EVIDENCE, project, &mut created)?;
    write_new(&sara.join("PADROES.md"), PATTERNS, project, &mut created)?;
    write_new(&sara.join("USOS.md"), USES, project, &mut created)?;
    match engine {
        crate::model::Engine::Godot => {
            write_new(
                &engine_dir.join("portao_ai_first.gd"),
                GODOT_GATE,
                project,
                &mut created,
            )?;
            write_new(
                &engine_dir.join("padroes_ai_first.gd"),
                GODOT_PATTERNS,
                project,
                &mut created,
            )?;
        }
        crate::model::Engine::Defold => {
            write_new(
                &engine_dir.join("portao_ai_first.lua"),
                DEFOLD_GATE,
                project,
                &mut created,
            )?;
            write_new(
                &engine_dir.join("padroes_ai_first.lua"),
                DEFOLD_PATTERNS,
                project,
                &mut created,
            )?;
        }
    }
    let config = format!(
        "schema_version = 1\nengine = \"{}\"\nprofiles = [\"desktop\", \"android\"]\n\n# Exceção exata, nunca baseline global:\n# [[allow]]\n# rule = \"SAR-OWN-001\"\n# resource = \"animation:...\"\n# owners = [\"arquivo::funcao_a\", \"arquivo::funcao_b\"]\n# reason = \"por que a composição não sobrepõe em runtime\"\n",
        engine
    );
    write_new(&project.join("sara.toml"), &config, project, &mut created)?;
    Ok(created)
}

fn write_new(path: &Path, content: &str, root: &Path, created: &mut Vec<String>) -> Result<()> {
    if path.exists() {
        return Ok(());
    }
    fs::write(path, content)
        .with_context(|| format!("não consegui escrever {}", path.display()))?;
    created.push(
        path.strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/"),
    );
    Ok(())
}
