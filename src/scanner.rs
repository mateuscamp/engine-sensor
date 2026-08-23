use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use crate::{
    config::EngineChoice,
    model::Engine,
    parser::{ParsedSource, parse_source},
};

pub fn detect_engine(project: &Path, choice: EngineChoice) -> Result<Engine> {
    match choice {
        EngineChoice::Godot => return Ok(Engine::Godot),
        EngineChoice::Defold => return Ok(Engine::Defold),
        EngineChoice::Auto => {}
    }
    let godot = project.join("project.godot").is_file();
    let defold = project.join("game.project").is_file();
    match (godot, defold) {
        (true, false) => Ok(Engine::Godot),
        (false, true) => Ok(Engine::Defold),
        (true, true) => {
            bail!("o caminho contém project.godot e game.project; informe --engine explicitamente")
        }
        (false, false) => bail!(
            "não encontrei project.godot nem game.project em {}",
            project.display()
        ),
    }
}

pub fn scan_sources(project: &Path, engine: Engine) -> Result<Vec<ParsedSource>> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(project)
        .follow_links(false)
        .into_iter()
        .filter_entry(included_entry)
    {
        let entry = entry.with_context(|| format!("falha ao percorrer {}", project.display()))?;
        if entry.file_type().is_file() && supported(entry.path(), engine) {
            paths.push(entry.into_path());
        }
    }
    paths.sort();

    let mut parsed = Vec::with_capacity(paths.len());
    for path in paths {
        let relative = path
            .strip_prefix(project)
            .with_context(|| format!("{} saiu da raiz do projeto", path.display()))?
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(&path)
            .with_context(|| format!("não consegui ler {}", path.display()))?;
        parsed.push(parse_source(engine, relative, source)?);
    }
    Ok(parsed)
}

pub fn validate_compatibility(project: &Path, engine: Engine) -> Result<()> {
    if engine != Engine::Godot {
        return Ok(());
    }
    let path = project.join("project.godot");
    let source = fs::read_to_string(&path)
        .with_context(|| format!("não consegui ler {}", path.display()))?;
    let expression =
        Regex::new(r#"config/features\s*=\s*PackedStringArray\(\s*"([0-9]+\.[0-9]+)""#).unwrap();
    if let Some(captures) = expression.captures(&source)
        && &captures[1] != "4.7"
    {
        bail!(
            "Godot {} está fora do contrato 0.1.0; esperado Godot 4.7",
            &captures[1]
        );
    }
    Ok(())
}

fn included_entry(entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return true;
    }
    if !entry.file_type().is_dir() {
        return true;
    }
    !matches!(
        entry.file_name().to_string_lossy().as_ref(),
        ".git"
            | ".godot"
            | ".sara"
            | ".aurora"
            | "build"
            | "dist"
            | "target"
            | "node_modules"
            | "vendor"
    )
}

fn supported(path: &Path, engine: Engine) -> bool {
    let extension = path.extension().and_then(|value| value.to_str());
    match engine {
        Engine::Godot => extension == Some("gd"),
        Engine::Defold => matches!(
            extension,
            Some("lua" | "script" | "gui_script" | "render_script")
        ),
    }
}
