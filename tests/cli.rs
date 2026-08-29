use std::{fs, path::PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::TempDir;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn json_report(name: &str, extra: &[&str]) -> (i32, Value, String) {
    let mut command = Command::cargo_bin("sara").expect("binary");
    command
        .arg("check")
        .arg(fixture(name))
        .arg("--format")
        .arg("json")
        .args(extra);
    let output = command.output().expect("run");
    let code = output.status.code().expect("exit code");
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    let json = serde_json::from_str(&stdout).unwrap_or_else(|error| {
        panic!(
            "invalid json for {name}: {error}\nstdout={stdout}\nstderr={}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    (code, json, stdout)
}

#[test]
fn defold_animation_red_fails_and_green_passes() {
    let (red_code, red, _) = json_report("defold_animation_red", &[]);
    assert_eq!(red_code, 1);
    assert!(
        red["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| { item["rule"] == "SAR-OWN-001" && item["severity"] == "error" })
    );

    let (green_code, green, _) = json_report("defold_animation_green", &[]);
    assert_eq!(green_code, 0);
    assert!(green["diagnostics"].as_array().unwrap().is_empty());

    let (dominating_code, dominating, _) =
        json_report("defold_animation_dominating_cancel_green", &[]);
    assert_eq!(dominating_code, 0);
    assert!(dominating["diagnostics"].as_array().unwrap().is_empty());

    let (separate_loops_code, separate_loops, _) =
        json_report("defold_animation_separate_loops_green", &[]);
    assert_eq!(separate_loops_code, 0);
    assert!(separate_loops["diagnostics"].as_array().unwrap().is_empty());

    let (callback_code, callback, _) = json_report("defold_animation_callback_green", &[]);
    assert_eq!(callback_code, 0);
    assert!(callback["diagnostics"].as_array().unwrap().is_empty());

    let (init_exit_code, init_exit, _) = json_report("defold_animation_init_exit_green", &[]);
    assert_eq!(init_exit_code, 0);
    assert!(init_exit["diagnostics"].as_array().unwrap().is_empty());
}

#[test]
fn defold_input_is_profile_aware() {
    let (android_code, android, _) = json_report("defold_input_red", &["--profile", "android"]);
    assert_eq!(android_code, 1);
    assert!(
        android["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| { item["rule"] == "SAR-OWN-002" && item["severity"] == "error" })
    );

    let (desktop_code, desktop, _) = json_report("defold_input_red", &["--profile", "desktop"]);
    assert_eq!(desktop_code, 0);
    assert!(desktop["diagnostics"].as_array().unwrap().is_empty());

    let (green_code, _, _) = json_report("defold_input_green", &["--profile", "android"]);
    assert_eq!(green_code, 0);
}

#[test]
fn godot_distinguishes_sequential_and_competing_tweens() {
    let (red_code, red, _) = json_report("godot_animation_red", &[]);
    assert_eq!(red_code, 1);
    assert!(
        red["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| { item["rule"] == "SAR-OWN-001" && item["severity"] == "error" })
    );

    let (green_code, green, _) = json_report("godot_animation_green", &[]);
    assert_eq!(green_code, 0);
    assert!(green["diagnostics"].as_array().unwrap().is_empty());
}

/// ADR 0009. O padrão de dono centralizado — cada escritor encerra o Tween guardado
/// antes de criar o seu — é a própria remediação que o `SAR-OWN-001` recomenda, e
/// virava aviso falso quando o cancelamento passava por método auxiliar.
///
/// A segunda metade do teste é a que importa: uma regra boa demais silenciaria também
/// o caso em que só um dos lados cancela, que não serializa nada.
/// ADR 0010. O eixo de entrada só enxergava projeto com mapa de ações declarado, e o
/// porte do BomberBoom não usa mapa: despacha `InputEvent` cru. Metade da ferramenta
/// era cega no único projeto em desenvolvimento.
///
/// As duas metades verdes é que dão sentido à vermelha: desligar a emulação separa os
/// canais de verdade, e tratar os dois canais não é conflito quando cada um cai num
/// efeito diferente.
#[test]
fn godot_detects_touch_and_mouse_reaching_the_same_effect() {
    let (android_code, android, _) =
        json_report("godot_input_channel_red", &["--profile", "android"]);
    assert_eq!(android_code, 1);
    assert!(
        android["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| { item["rule"] == "SAR-OWN-002" && item["severity"] == "error" }),
        "{android:#}"
    );

    let (desktop_code, desktop, _) =
        json_report("godot_input_channel_red", &["--profile", "desktop"]);
    assert_eq!(
        desktop_code, 0,
        "sem toque não há canal duplicado: {desktop:#}"
    );
    assert!(desktop["diagnostics"].as_array().unwrap().is_empty());

    let (green_code, green, _) =
        json_report("godot_input_channel_green", &["--profile", "android"]);
    assert_eq!(
        green_code, 0,
        "emulate_mouse_from_touch=false separa os canais: {green:#}"
    );

    let (separate_code, separate, _) = json_report(
        "godot_input_channel_separate_green",
        &["--profile", "android"],
    );
    assert_eq!(
        separate_code, 0,
        "canais distintos em efeitos distintos não são conflito: {separate:#}"
    );
}

#[test]
fn godot_recognizes_centralized_owner_cancellation() {
    let (code, report, _) = json_report("godot_animation_centralized_owner_green", &[]);
    assert_eq!(code, 0);
    assert_eq!(report["claims"].as_array().unwrap().len(), 2, "{report:#}");
    assert!(
        report["diagnostics"].as_array().unwrap().is_empty(),
        "dono centralizado por método auxiliar não pode gerar diagnóstico: {report:#}"
    );

    let (warn_code, warn, _) = json_report("godot_animation_uncancelled_owners_warn", &[]);
    assert_eq!(warn_code, 0);
    assert!(
        warn["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| { item["rule"] == "SAR-OWN-001" && item["severity"] == "warning" }),
        "cancelar de um lado só não serializa: o aviso precisa continuar de pé: {warn:#}"
    );
}

#[test]
fn godot_inventories_tweens_with_fluent_configuration() {
    let (code, report, _) = json_report("godot_animation_fluent_green", &[]);
    assert_eq!(code, 0);
    let claims = report["claims"].as_array().unwrap();
    assert_eq!(claims.len(), 2, "report={report:#}");
    assert!(
        !report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["severity"] == "error")
    );
}

/// O relógio do Tween, nascido do caso da aranha (28/08/2026).
///
/// A Sara declarava alvo, propriedade e dono, e nenhuma das três muda quando alguém
/// pausa ou desacelera a trajetória -- enquanto o que acontece na tela muda inteiro.
/// A capacidade entra **declarando**, sem diagnóstico novo, como a ADR 0010 entrou.
#[test]
fn godot_inventories_who_controls_the_tween_clock() {
    let (code, report, _) = json_report("godot_animation_clock_control_green", &[]);
    assert_eq!(code, 0);
    let claims = report["claims"].as_array().unwrap();
    let relogio = |operacao: &str| -> Vec<&serde_json::Value> {
        claims
            .iter()
            .filter(|item| item["operation"] == operacao)
            .collect()
    };

    for operacao in ["Tween.pause", "Tween.play", "Tween.set_speed_scale"] {
        assert!(
            !relogio(operacao).is_empty(),
            "{operacao} não virou declaração: {report:#}"
        );
    }

    // A forma em que o caso de origem estava escrita: `for animacao in [a, b]:`.
    // Uma capacidade que não vê o próprio caso de origem não foi construída.
    let pelo_laco = relogio("Tween.set_speed_scale")
        .into_iter()
        .filter(|item| {
            item["owner"]
                .as_str()
                .unwrap()
                .ends_with("lentificar_o_pavio")
        })
        .count();
    assert_eq!(
        pelo_laco, 2,
        "a variável de laço precisa valer pelos dois Tweens da lista: {report:#}"
    );

    // O limite, e é ele que separa esta regra de uma que inventa: `play()` num
    // AudioStreamPlayer tem o mesmo nome e não é Tween nenhum. Uma regra "boa demais"
    // reprova aqui.
    assert!(
        !claims.iter().any(|item| {
            item["operation"] == "Tween.play"
                && item["owner"].as_str().unwrap().contains("soltar")
                && item["span"]["line"].as_u64() == Some(33)
        }),
        "declarou relógio para um objeto que não é Tween: {report:#}"
    );

    assert!(
        report["diagnostics"].as_array().unwrap().is_empty(),
        "a capacidade entra sem diagnóstico novo: {report:#}"
    );
}

#[test]
fn godot_input_needs_one_owner_or_explicit_consumption() {
    let (red_code, red, _) = json_report("godot_input_red", &[]);
    assert_eq!(red_code, 1);
    assert!(
        red["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| { item["rule"] == "SAR-OWN-002" && item["severity"] == "error" })
    );

    let (green_code, green, _) = json_report("godot_input_green", &[]);
    assert_eq!(green_code, 0);
    assert!(green["diagnostics"].as_array().unwrap().is_empty());
}

#[test]
fn godot_undeclared_action_warns_without_inventing_a_conflict() {
    let (code, report, _) = json_report("godot_input_undeclared", &[]);
    assert_eq!(code, 0);
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert!(diagnostics.iter().any(|item| {
        item["rule"] == "SAR-PARSE-001"
            && item["severity"] == "warning"
            && item["explanation"]
                .as_str()
                .is_some_and(|text| text.contains("não está definida"))
    }));
    assert!(!diagnostics.iter().any(|item| item["severity"] == "error"));
}

#[test]
fn syntax_error_exits_two_and_names_the_rule() {
    Command::cargo_bin("sara")
        .expect("binary")
        .arg("check")
        .arg(fixture("godot_invalid"))
        .assert()
        .code(2)
        .stderr(predicate::str::contains("SAR-PARSE-001"));
}

#[test]
fn incompatible_engine_version_exits_two() {
    Command::cargo_bin("sara")
        .expect("binary")
        .arg("check")
        .arg(fixture("godot_incompatible"))
        .assert()
        .code(2)
        .stderr(predicate::str::contains("fora do contrato"));
}

#[test]
fn dynamic_target_warns_without_blocking() {
    let (code, report, _) = json_report("defold_dynamic", &[]);
    assert_eq!(code, 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| { item["rule"] == "SAR-PARSE-001" && item["severity"] == "warning" })
    );
}

#[test]
fn mutation_that_adds_a_second_owner_is_rejected() {
    let temporary = TempDir::new().expect("tempdir");
    copy_tree(&fixture("defold_animation_green"), temporary.path());
    fs::write(
        temporary.path().join("main/hud.gui_script"),
        "function init(self)\n    gui.animate(\"bomb\", gui.PROP_SCALE, vmath.vector3(1, 1, 1), gui.EASING_LINEAR, 0.2)\n    gui.animate(\"bomb\", gui.PROP_SCALE, vmath.vector3(2, 2, 1), gui.EASING_LINEAR, 0.2)\nend\n",
    )
    .expect("mutate");
    Command::cargo_bin("sara")
        .expect("binary")
        .arg("check")
        .arg(temporary.path())
        .assert()
        .code(1)
        .stdout(predicate::str::contains("SAR-OWN-001"));
}

#[test]
fn json_is_byte_for_byte_deterministic() {
    let (_, _, first) = json_report("defold_animation_red", &[]);
    let (_, _, second) = json_report("defold_animation_red", &[]);
    assert_eq!(first, second);
}

#[test]
fn init_creates_both_agent_fragments_without_overwriting_roots() {
    let temporary = TempDir::new().expect("tempdir");
    fs::write(temporary.path().join("project.godot"), "config_version=5\n").expect("manifest");
    fs::write(temporary.path().join("AGENTS.md"), "meu agents\n").expect("agents");
    fs::write(temporary.path().join("CLAUDE.md"), "meu claude\n").expect("claude");

    Command::cargo_bin("sara")
        .expect("binary")
        .arg("init")
        .arg(temporary.path())
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(temporary.path().join("AGENTS.md")).unwrap(),
        "meu agents\n"
    );
    assert_eq!(
        fs::read_to_string(temporary.path().join("CLAUDE.md")).unwrap(),
        "meu claude\n"
    );
    assert!(temporary.path().join(".sara/CONTRATO.md").is_file());
    assert!(temporary.path().join(".sara/PADROES.md").is_file());
    assert!(temporary.path().join(".sara/USOS.md").is_file());
    assert!(temporary.path().join(".sara/AGENTS.fragment.md").is_file());
    assert!(temporary.path().join(".sara/CLAUDE.fragment.md").is_file());
    assert!(
        temporary
            .path()
            .join(".sara/godot/portao_ai_first.gd")
            .is_file()
    );
    assert!(
        temporary
            .path()
            .join(".sara/godot/padroes_ai_first.gd")
            .is_file()
    );
}

#[test]
fn init_creates_defold_gate_and_patterns() {
    let temporary = TempDir::new().expect("tempdir");
    fs::write(
        temporary.path().join("game.project"),
        "[project]\ntitle = Sara\n",
    )
    .expect("manifest");

    Command::cargo_bin("sara")
        .expect("binary")
        .arg("init")
        .arg(temporary.path())
        .assert()
        .success();

    assert!(
        temporary
            .path()
            .join(".sara/defold/portao_ai_first.lua")
            .is_file()
    );
    assert!(
        temporary
            .path()
            .join(".sara/defold/padroes_ai_first.lua")
            .is_file()
    );
}

#[test]
fn exact_exception_suppresses_only_the_named_conflict() {
    let temporary = TempDir::new().expect("tempdir");
    copy_tree(&fixture("defold_animation_red"), temporary.path());
    let request = sara_ai_first::CheckRequest {
        project: temporary.path().to_path_buf(),
        engine: sara_ai_first::config::EngineChoice::Auto,
        profiles: vec![sara_ai_first::config::Profile::Desktop],
        allow: Vec::new(),
    };
    let report = sara_ai_first::check_project(&request).expect("report");
    let diagnostic = report.diagnostics.first().expect("diagnostic");
    let owners = diagnostic
        .owners
        .iter()
        .map(|owner| format!("\"{}\"", owner))
        .collect::<Vec<_>>()
        .join(", ");
    fs::write(
        temporary.path().join("sara.toml"),
        format!(
            "schema_version = 1\nengine = \"defold\"\nprofiles = [\"desktop\"]\n\n[[allow]]\nrule = \"{}\"\nresource = \"{}\"\nowners = [{}]\nreason = \"ciclos mutuamente exclusivos provados pelo projeto\"\n",
            diagnostic.rule, diagnostic.resource, owners
        ),
    )
    .expect("config");
    Command::cargo_bin("sara")
        .expect("binary")
        .arg("check")
        .arg(temporary.path())
        .assert()
        .success();
}

fn copy_tree(source: &std::path::Path, destination: &std::path::Path) {
    for entry in walkdir::WalkDir::new(source) {
        let entry = entry.expect("entry");
        let relative = entry.path().strip_prefix(source).expect("relative");
        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target).expect("directory");
        } else {
            fs::copy(entry.path(), target).expect("copy");
        }
    }
}
