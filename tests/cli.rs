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
    let mut command = Command::cargo_bin("engine-sensor").expect("binary");
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
            .any(|item| { item["rule"] == "ESN-OWN-001" && item["severity"] == "error" })
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
            .any(|item| { item["rule"] == "ESN-OWN-002" && item["severity"] == "error" })
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
            .any(|item| { item["rule"] == "ESN-OWN-001" && item["severity"] == "error" })
    );

    let (green_code, green, _) = json_report("godot_animation_green", &[]);
    assert_eq!(green_code, 0);
    assert!(green["diagnostics"].as_array().unwrap().is_empty());
}

/// ADR 0009. O padrão de dono centralizado — cada escritor encerra o Tween guardado
/// antes de criar o seu — é a própria remediação que o `ESN-OWN-001` recomenda, e
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
            .any(|item| { item["rule"] == "ESN-OWN-002" && item["severity"] == "error" }),
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
            .any(|item| { item["rule"] == "ESN-OWN-001" && item["severity"] == "warning" }),
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
/// O engine-sensor declarava alvo, propriedade e dono, e nenhuma das três muda quando alguém
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

/// A profundidade de desenho, nascida do caso da aranha (28/08/2026).
///
/// A terceira das tres capacidades que o caso nomeou, e a unica que pegou um defeito
/// por conta propria: o fio de seda com `z_index = -1` nao apareceu em quadro nenhum e
/// nenhum teste, portao ou captura viu. `z_index` e a ordem entre irmaos decidem a
/// mesma coisa na tela sem se conhecerem, e nenhum dos dois virava declaracao.
///
/// Entra **declarando**, sem diagnostico novo, como a ADR 0010 e o relogio do Tween.
#[test]
fn godot_inventories_who_decides_draw_order() {
    let (code, report, _) = json_report("godot_draw_order_green", &[]);
    assert_eq!(code, 0);
    let claims = report["claims"].as_array().unwrap();
    let profundidade = claims
        .iter()
        .filter(|item| item["resource"]["kind"] == "draw_order")
        .collect::<Vec<_>>();

    // Tres, e o numero e a assertiva: a fixture tem QUATRO armadilhas que uma regra
    // boa demais transformaria em declaracao.
    assert_eq!(
        profundidade.len(),
        3,
        "declarou profundidade a mais ou a menos: {report:#}"
    );

    // Os dois mecanismos caem no MESMO recurso, com um controlador cada. E isso que
    // torna visivel a coisa que o caso nomeou: duas fontes de verdade sobre a mesma
    // coordenada, e nenhuma delas sabe da outra.
    let fio = profundidade
        .iter()
        .filter(|item| item["resource"]["target"] == "_fio")
        .collect::<Vec<_>>();
    assert_eq!(
        fio.len(),
        2,
        "o fio precisa dos dois mecanismos: {report:#}"
    );
    assert_eq!(
        fio[0]["resource"], fio[1]["resource"],
        "os dois mecanismos precisam cair no mesmo recurso: {report:#}"
    );
    let mecanismos = fio
        .iter()
        .map(|item| item["controller"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        mecanismos,
        ["ordem_de_filho", "z_index"].into_iter().collect(),
        "os dois mecanismos precisam ser controladores distintos: {report:#}"
    );
    let operacoes = fio
        .iter()
        .map(|item| item["operation"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        operacoes,
        ["CanvasItem.z_index", "Node.move_child"]
            .into_iter()
            .collect(),
        "as operacoes saem com o nome publico da API: {report:#}"
    );

    // A forma exata do porte: `z_index = 1` sem prefixo, no proprio no.
    assert!(
        profundidade.iter().any(|item| {
            item["resource"]["target"] == "self"
                && item["owner"].as_str().unwrap().ends_with("_montar")
        }),
        "a escrita sem prefixo e no proprio no, e ela some se a regra exigir alvo escrito: {report:#}"
    );

    // As quatro armadilhas, e todas existem HOJE no corpus pessoal: `z_index` dentro
    // de comentario (tres vezes no Gods e uma no porte), leitura, comparacao e alvo
    // dinamico (`_cards_ui[i]`, hand_ui.gd). Nenhuma pode virar declaracao.
    assert!(
        !profundidade
            .iter()
            .any(|item| item["owner"].as_str().unwrap().ends_with("_limites")),
        "declarou profundidade a partir de comentario, leitura, comparacao ou alvo dinamico: {report:#}"
    );

    assert!(
        report["diagnostics"].as_array().unwrap().is_empty(),
        "a capacidade entra sem diagnostico novo: {report:#}"
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
            .any(|item| { item["rule"] == "ESN-OWN-002" && item["severity"] == "error" })
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
        item["rule"] == "ESN-PARSE-001"
            && item["severity"] == "warning"
            && item["explanation"]
                .as_str()
                .is_some_and(|text| text.contains("não está definida"))
    }));
    assert!(!diagnostics.iter().any(|item| item["severity"] == "error"));
}

#[test]
fn syntax_error_exits_two_and_names_the_rule() {
    Command::cargo_bin("engine-sensor")
        .expect("binary")
        .arg("check")
        .arg(fixture("godot_invalid"))
        .assert()
        .code(2)
        .stderr(predicate::str::contains("ESN-PARSE-001"));
}

#[test]
fn incompatible_engine_version_exits_two() {
    Command::cargo_bin("engine-sensor")
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
            .any(|item| { item["rule"] == "ESN-PARSE-001" && item["severity"] == "warning" })
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
    Command::cargo_bin("engine-sensor")
        .expect("binary")
        .arg("check")
        .arg(temporary.path())
        .assert()
        .code(1)
        .stdout(predicate::str::contains("ESN-OWN-001"));
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

    Command::cargo_bin("engine-sensor")
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
    assert!(temporary.path().join(".engine-sensor/CONTRATO.md").is_file());
    assert!(temporary.path().join(".engine-sensor/PADROES.md").is_file());
    assert!(temporary.path().join(".engine-sensor/USOS.md").is_file());
    assert!(temporary.path().join(".engine-sensor/AGENTS.fragment.md").is_file());
    assert!(temporary.path().join(".engine-sensor/CLAUDE.fragment.md").is_file());
    assert!(
        temporary
            .path()
            .join(".engine-sensor/godot/portao_ai_first.gd")
            .is_file()
    );
    assert!(
        temporary
            .path()
            .join(".engine-sensor/godot/padroes_ai_first.gd")
            .is_file()
    );
}

#[test]
fn init_creates_defold_gate_and_patterns() {
    let temporary = TempDir::new().expect("tempdir");
    fs::write(
        temporary.path().join("game.project"),
        "[project]\ntitle = engine-sensor\n",
    )
    .expect("manifest");

    Command::cargo_bin("engine-sensor")
        .expect("binary")
        .arg("init")
        .arg(temporary.path())
        .assert()
        .success();

    assert!(
        temporary
            .path()
            .join(".engine-sensor/defold/portao_ai_first.lua")
            .is_file()
    );
    assert!(
        temporary
            .path()
            .join(".engine-sensor/defold/padroes_ai_first.lua")
            .is_file()
    );
}

#[test]
fn exact_exception_suppresses_only_the_named_conflict() {
    let temporary = TempDir::new().expect("tempdir");
    copy_tree(&fixture("defold_animation_red"), temporary.path());
    let request = engine_sensor::CheckRequest {
        project: temporary.path().to_path_buf(),
        engine: engine_sensor::config::EngineChoice::Auto,
        profiles: vec![engine_sensor::config::Profile::Desktop],
        allow: Vec::new(),
    };
    let report = engine_sensor::check_project(&request).expect("report");
    let diagnostic = report.diagnostics.first().expect("diagnostic");
    let owners = diagnostic
        .owners
        .iter()
        .map(|owner| format!("\"{}\"", owner))
        .collect::<Vec<_>>()
        .join(", ");
    fs::write(
        temporary.path().join("engine-sensor.toml"),
        format!(
            "schema_version = 1\nengine = \"defold\"\nprofiles = [\"desktop\"]\n\n[[allow]]\nrule = \"{}\"\nresource = \"{}\"\nowners = [{}]\nreason = \"ciclos mutuamente exclusivos provados pelo projeto\"\n",
            diagnostic.rule, diagnostic.resource, owners
        ),
    )
    .expect("config");
    Command::cargo_bin("engine-sensor")
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

// ---------------------------------------------------------------------------
// Fronteira entre o projeto e os checkouts aninhados nele
// ---------------------------------------------------------------------------

/// Copia a fixture e planta a marca com que o Git assinala a raiz de um checkout.
///
/// `gitdir` ausente escreve `.git` como **diretório** — o repositório comum; presente
/// escreve `.git` como **arquivo**, que é a forma de worktree vinculada e de submódulo.
/// O valor não precisa existir: cópia velha com referência quebrada é metade do caso
/// real medido no BomberBoom.
fn plant_checkout(source: &std::path::Path, destination: &std::path::Path, gitdir: Option<&str>) {
    copy_tree(source, destination);
    match gitdir {
        Some(alvo) => {
            fs::write(destination.join(".git"), format!("gitdir: {alvo}\n")).expect("gitfile")
        }
        None => fs::create_dir_all(destination.join(".git")).expect("gitdir"),
    }
}

/// Caminhos que os diagnósticos apontam, sem repetição.
fn diagnostic_paths(report: &Value) -> std::collections::BTreeSet<String> {
    report["diagnostics"]
        .as_array()
        .expect("diagnostics")
        .iter()
        .map(|item| item["primary"]["path"].as_str().expect("path").to_owned())
        .collect()
}

fn check_json(project: &std::path::Path) -> (i32, Value) {
    let output = Command::cargo_bin("engine-sensor")
        .expect("binary")
        .arg("check")
        .arg(project)
        .arg("--format")
        .arg("json")
        .output()
        .expect("run");
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    (
        output.status.code().expect("exit code"),
        serde_json::from_str(&stdout).unwrap_or_else(|error| {
            panic!(
                "json inválido: {error}\nstdout={stdout}\nstderr={}",
                String::from_utf8_lossy(&output.stderr)
            )
        }),
    )
}

/// O caso medido em 09/09/2026 no BomberBoom: `check .` na raiz do projeto devolveu
/// 141 arquivos e 56 avisos, e 48 deles vinham de quatro cópias do próprio projeto
/// em `.claude/worktrees/`. Não eram achados novos — eram os mesmos achados, contados
/// cinco vezes, com o caminho de uma árvore que ninguém está editando.
///
/// A marca do Git é o que separa as duas coisas, e ela vale nos dois estados que o
/// caso real tem: uma worktree com metadado válido e três cópias cujo `gitdir` aponta
/// para um diretório que não existe mais. Nenhuma das duas é fonte do projeto pedido.
///
/// A segunda metade é a que impede a correção fácil de passar: `.claude/ferramentas`
/// é diretório oculto, não é checkout, e continua analisado. O critério é a marca do
/// Git, não o ponto no começo do nome.
#[test]
fn nested_checkouts_are_not_sources_of_the_requested_project() {
    let temporary = TempDir::new().expect("tempdir");
    let raiz = temporary.path();
    copy_tree(&fixture("defold_animation_red"), raiz);

    plant_checkout(
        &fixture("defold_animation_red"),
        &raiz.join(".claude/worktrees/metadado-valido"),
        Some(
            &raiz
                .join(".git/worktrees/metadado-valido")
                .display()
                .to_string(),
        ),
    );
    fs::create_dir_all(raiz.join(".git/worktrees/metadado-valido")).expect("admin dir");

    plant_checkout(
        &fixture("defold_animation_red"),
        &raiz.join(".claude/worktrees/referencia-quebrada"),
        Some("/nao/existe/.git/worktrees/referencia-quebrada"),
    );

    plant_checkout(
        &fixture("defold_animation_red"),
        &raiz.join(".claude/clone-solto"),
        None,
    );

    let oculto = raiz.join(".claude/ferramentas");
    fs::create_dir_all(&oculto).expect("hidden dir");
    fs::copy(
        fixture("defold_animation_red").join("main/hud.gui_script"),
        oculto.join("hud.gui_script"),
    )
    .expect("hidden source");

    let (code, report) = check_json(raiz);
    let caminhos = diagnostic_paths(&report);

    assert_eq!(
        report["files_scanned"], 2,
        "o scanner leu {} arquivo(s); esperado 2 — o do projeto e o do diretório oculto \
         que não é checkout. Caminhos com diagnóstico: {caminhos:?}",
        report["files_scanned"]
    );
    assert_eq!(
        caminhos,
        [
            ".claude/ferramentas/hud.gui_script".to_owned(),
            "main/hud.gui_script".to_owned()
        ]
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>(),
        "checkout aninhado voltou a entrar como fonte do projeto, ou fonte legítima \
         em diretório oculto parou de ser analisada"
    );
    assert_eq!(code, 1, "o conflito do próprio projeto continua bloqueando");
}

/// A correção não pode custar a análise de uma worktree: é dentro delas que o agente
/// trabalha, e é lá que `engine-sensor check .` precisa responder. A marca do Git na
/// raiz pedida não a exclui — ela diz apenas onde outro checkout começa, e a raiz
/// pedida é sempre o projeto.
#[test]
fn a_worktree_is_analyzed_when_it_is_the_requested_root() {
    let temporary = TempDir::new().expect("tempdir");
    let raiz = temporary.path();
    copy_tree(&fixture("defold_animation_red"), raiz);

    for (nome, gitdir) in [
        (
            "metadado-valido",
            raiz.join(".git/worktrees/metadado-valido")
                .display()
                .to_string(),
        ),
        (
            "referencia-quebrada",
            "/nao/existe/.git/worktrees/referencia-quebrada".to_owned(),
        ),
    ] {
        let worktree = raiz.join(".claude/worktrees").join(nome);
        plant_checkout(&fixture("defold_animation_red"), &worktree, Some(&gitdir));
        fs::create_dir_all(raiz.join(".git/worktrees/metadado-valido")).expect("admin dir");

        let (code, report) = check_json(&worktree);
        assert_eq!(
            code, 1,
            "{nome}: a worktree pedida como raiz deixou de bloquear"
        );
        assert_eq!(
            report["files_scanned"], 1,
            "{nome}: fonte da worktree não foi lida"
        );
        assert_eq!(
            diagnostic_paths(&report),
            ["main/hud.gui_script".to_owned()]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>(),
            "{nome}: o diagnóstico saiu com caminho fora da raiz pedida"
        );
    }
}

/// Submódulo é checkout aninhado que o **próprio projeto declara** como conteúdo seu,
/// em `.gitmodules`. Worktree vinculada e clone solto não têm essa declaração, e é ela
/// — e não o gitdir, que numa cópia velha não existe mais — que separa os dois casos.
///
/// Sem esta metade, a correção trocaria um defeito por outro: deixaria de contar cinco
/// vezes o mesmo achado e passaria a não contar nenhuma vez o código que a engine
/// carrega junto com o projeto.
#[test]
fn declared_submodule_stays_project_content() {
    let temporary = TempDir::new().expect("tempdir");
    let raiz = temporary.path();
    copy_tree(&fixture("defold_animation_red"), raiz);
    fs::write(
        raiz.join(".gitmodules"),
        "[submodule \"compartilhado\"]\n\tpath = compartilhado\n\turl = ../compartilhado.git\n",
    )
    .expect("gitmodules");

    plant_checkout(
        &fixture("defold_animation_red"),
        &raiz.join("compartilhado"),
        Some("/nao/existe/.git/modules/compartilhado"),
    );

    let (_, report) = check_json(raiz);
    assert!(
        diagnostic_paths(&report).contains("compartilhado/main/hud.gui_script"),
        "o submódulo declarado saiu da análise. Caminhos: {:?}",
        diagnostic_paths(&report)
    );
}
