use std::path::PathBuf;

use sara_ai_first::{
    CheckRequest, check_project,
    config::{EngineChoice, Profile},
};

#[test]
#[ignore = "depende do corpus pessoal local"]
fn five_personal_projects_have_no_blocking_false_positive() {
    // Caminhos corrigidos em 28/08/2026: os cinco tinham migrado de `~/Godot` para
    // `~/godot`, e o porte mudou de nome. O teste é `#[ignore]`, então a defasagem
    // ficou invisível -- e com ela o confronto com o corpus que a ADR 0012 §3 exige.
    let projects = [
        "/home/mateus/defold/bomberboom-df",
        "/home/mateus/godot/bomberboom-gd",
        "/home/mateus/godot/boomlitude",
        "/home/mateus/godot/mineboom",
        "/home/mateus/godot/gods",
    ];
    for project in projects {
        let path = PathBuf::from(project);
        assert!(path.is_dir(), "corpus ausente: {project}");
        let report = check_project(&CheckRequest {
            project: path,
            engine: EngineChoice::Auto,
            profiles: vec![Profile::Desktop, Profile::Android],
            allow: Vec::new(),
        })
        .unwrap_or_else(|error| panic!("{project}: {error:#}"));
        assert!(
            !report.has_errors(),
            "{project} produziu erro(s): {:?}",
            report.diagnostics
        );
    }
}
