use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

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
    let submodules = declared_submodules(project);
    let mut paths = Vec::new();
    for entry in WalkDir::new(project)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| included_entry(project, &submodules, entry))
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

/// Nome com que o Git assinala a raiz de um checkout. Como **diretório** num
/// repositório comum; como **arquivo** (`gitdir: ...`) numa worktree vinculada e num
/// submódulo. Os dois formatos vivem na raiz e em nenhum outro lugar da árvore.
const GIT_MARKER: &str = ".git";

fn included_entry(project: &Path, submodules: &BTreeSet<PathBuf>, entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return true;
    }
    if !entry.file_type().is_dir() {
        return true;
    }
    if matches!(
        entry.file_name().to_string_lossy().as_ref(),
        GIT_MARKER
            | ".godot"
            | ".engine-sensor"
            | ".aurora"
            | "build"
            | "dist"
            | "target"
            | "node_modules"
            | "vendor"
    ) {
        return false;
    }
    !foreign_checkout(project, submodules, entry.path())
}

/// Raiz de outro checkout, que este projeto não declara como conteúdo seu.
///
/// A raiz pedida nunca chega aqui: `included_entry` devolve cedo na profundidade 0, e
/// é isso que mantém `check .` funcionando **dentro** de uma worktree, onde o agente
/// trabalha. Abaixo dela a marca do Git diz onde outro checkout começa, e o que começa
/// ali não é fonte do projeto pedido — é outra árvore, com outra história, que ninguém
/// está editando por este caminho.
///
/// O critério é a **presença** da marca, não o que ela aponta. Cópia velha cujo
/// `gitdir` não existe mais continua sendo outro checkout, e era essa a metade maior do
/// caso medido no BomberBoom em 09/09/2026: três das quatro cópias apontavam para um
/// repositório que tinha sido renomeado. Resolver o alvo, ou perguntar ao binário do
/// `git`, deixaria justamente essas três de fora — e traria dependência de processo
/// externo a uma ferramenta que é offline por decisão.
fn foreign_checkout(project: &Path, submodules: &BTreeSet<PathBuf>, path: &Path) -> bool {
    if fs::symlink_metadata(path.join(GIT_MARKER)).is_err() {
        return false;
    }
    path.strip_prefix(project)
        .is_ok_and(|relative| !submodules.contains(relative))
}

/// Caminhos que o `.gitmodules` da raiz declara como submódulos.
///
/// Submódulo também é checkout aninhado, e a diferença não está na marca: está em o
/// projeto **declarar** aquele caminho como conteúdo seu. Worktree vinculada e clone
/// solto não têm declaração nenhuma, e é ela — e não o gitdir, que numa cópia velha já
/// não existe — que separa os dois casos.
///
/// Sem isso a correção trocaria um defeito por outro: pararia de contar cinco vezes o
/// mesmo achado e passaria a não contar nenhuma vez o código que a engine carrega
/// junto com o projeto. Ausência do arquivo é o caso comum e não é erro.
fn declared_submodules(project: &Path) -> BTreeSet<PathBuf> {
    let Ok(source) = fs::read_to_string(project.join(".gitmodules")) else {
        return BTreeSet::new();
    };
    source
        .lines()
        .filter_map(|line| line.split_once('='))
        .filter(|(key, _)| key.trim() == "path")
        .map(|(_, value)| PathBuf::from(value.trim()))
        .collect()
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
