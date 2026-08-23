use std::path::PathBuf;

use sara_ai_first::{
    CheckRequest, check_project,
    config::{EngineChoice, Profile},
};

#[test]
#[ignore = "depende do corpus pessoal local"]
fn five_personal_projects_have_no_blocking_false_positive() {
    let projects = [
        "/home/mateus/defold/bomberboom",
        "/home/mateus/Godot/port-gd-bomberboom",
        "/home/mateus/Godot/boomlitude",
        "/home/mateus/Godot/mineboom",
        "/home/mateus/Godot/gods",
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
