//! Portão do corpus pessoal.
//!
//! A [ADR 0012](../docs/decisoes/0012-sara-e-corpus-coevoluem.md) §3 obriga a
//! confrontar toda capacidade generalizável com os cinco projetos reais antes de
//! incorporá-la. Este é o portão executável desse confronto.
//!
//! Ele **roda na suíte padrão**. Até 29/08/2026 era `#[ignore]`, e a ADR 0017
//! registra o preço que isso cobrou: os cinco caminhos migraram de `~/Godot` para
//! `~/godot`, um projeto mudou de nome, e a defasagem ficou invisível porque um teste
//! que só roda quando alguém lembra de invocá-lo à mão não avisa quando envelhece.
//!
//! São **três estados**, e o terceiro é o motivo desta reescrita:
//!
//! - **aprovado** — os cinco projetos foram lidos e nenhum tem conflito bloqueante;
//! - **reprovado** — algum projeto estava lá e o confronto encontrou erro nele;
//! - **inconclusivo** — o corpus não estava no lugar declarado.
//!
//! Ausência não pode reprovar, porque falhar por ausência treina quem roda a ignorar
//! o vermelho. E não pode aprovar, porque **não poder conferir não é ter conferido**.
//! O arnês do Cargo só tem dois estados, então o inconclusivo sai por três canais que
//! ele não apaga: um bloco no descritor real do processo, o veredito em
//! `$SARA_CORPUS_VEREDITO` e o código de saída 2 de `tools/check_corpus.sh`.
//!
//! Os caminhos vêm do ambiente. Os valores de hoje são o **padrão documentado**, e
//! não a fonte:
//!
//! | Variável | Padrão |
//! |---|---|
//! | `SARA_CORPUS_RAIZ` | `/home/mateus` |
//! | `SARA_CORPUS_BOMBERBOOM_DF` | `$SARA_CORPUS_RAIZ/defold/bomberboom-df` |
//! | `SARA_CORPUS_BOMBERBOOM_GD` | `$SARA_CORPUS_RAIZ/godot/bomberboom-gd` |
//! | `SARA_CORPUS_BOOMLITUDE` | `$SARA_CORPUS_RAIZ/godot/boomlitude` |
//! | `SARA_CORPUS_MINEBOOM` | `$SARA_CORPUS_RAIZ/godot/mineboom` |
//! | `SARA_CORPUS_GODS` | `$SARA_CORPUS_RAIZ/godot/gods` |

use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use sara_ai_first::{
    CheckRequest, check_project,
    config::{EngineChoice, Profile},
};

/// Raiz dos cinco, quando nenhuma variável individual manda outra coisa.
const VARIAVEL_RAIZ: &str = "SARA_CORPUS_RAIZ";

/// O valor de hoje na máquina do proprietário. É padrão, e o comentário existe para
/// que ninguém volte a lê-lo como fonte: quando o corpus se mexer de novo, quem se
/// mexe é o ambiente, e o portão avisa no mesmo dia.
const RAIZ_PADRAO: &str = "/home/mateus";

/// Arquivo onde o veredito é escrito para quem chama o portão de fora do Cargo.
/// Ausente, o teste só imprime.
const VARIAVEL_VEREDITO: &str = "SARA_CORPUS_VEREDITO";

struct Projeto {
    nome: &'static str,
    variavel: &'static str,
    relativo: &'static str,
}

/// O corpus aplicável à regra estática, na redação da ADR 0012 §3: os cinco projetos.
const CORPUS: &[Projeto] = &[
    Projeto {
        nome: "bomberboom-df",
        variavel: "SARA_CORPUS_BOMBERBOOM_DF",
        relativo: "defold/bomberboom-df",
    },
    Projeto {
        nome: "bomberboom-gd",
        variavel: "SARA_CORPUS_BOMBERBOOM_GD",
        relativo: "godot/bomberboom-gd",
    },
    Projeto {
        nome: "boomlitude",
        variavel: "SARA_CORPUS_BOOMLITUDE",
        relativo: "godot/boomlitude",
    },
    Projeto {
        nome: "mineboom",
        variavel: "SARA_CORPUS_MINEBOOM",
        relativo: "godot/mineboom",
    },
    Projeto {
        nome: "gods",
        variavel: "SARA_CORPUS_GODS",
        relativo: "godot/gods",
    },
];

/// Variável de ambiente definida e não vazia. Vazia conta como ausente: `VAR=` é
/// engano de shell, e tratá-lo como caminho válido produziria "faltou ``".
fn variavel(nome: &str) -> Option<String> {
    env::var(nome).ok().filter(|valor| !valor.trim().is_empty())
}

/// Caminho do projeto e a origem dele. A origem entra na saída porque dizer "faltou"
/// sem dizer de onde veio o caminho manda procurar no lugar errado — foi assim que a
/// defasagem de 28/08 sobreviveu.
fn resolver(projeto: &Projeto) -> (PathBuf, String) {
    if let Some(valor) = variavel(projeto.variavel) {
        return (PathBuf::from(valor), format!("${}", projeto.variavel));
    }
    match variavel(VARIAVEL_RAIZ) {
        Some(raiz) => (
            PathBuf::from(raiz).join(projeto.relativo),
            format!("${VARIAVEL_RAIZ}"),
        ),
        None => (
            PathBuf::from(RAIZ_PADRAO).join(projeto.relativo),
            "padrão".to_owned(),
        ),
    }
}

/// Escreve no descritor real do processo, que a captura do libtest não alcança.
///
/// O arnês guarda a saída de cada teste e só a mostra quando ele falha, e o estado
/// inconclusivo é exatamente o que **passa** no arnês e precisa ser visto assim mesmo:
/// por `println!` ele sairia invisível num `cargo test` sem `--nocapture`, que é a forma
/// como quase todo mundo roda. A captura vale para a família `print!`/`eprint!`; o
/// descritor devolvido por `io::stderr()` não passa por ela.
///
/// Escrever nele, e não em `/dev/stderr` reaberto, é deliberado: reabrir cria uma
/// descrição de arquivo com deslocamento próprio, e sob `2> arquivo` as duas metades
/// da saída passam a se sobrescrever. Foi o que aconteceu na primeira versão deste
/// portão, e o texto que se perdia era justamente o do inconclusivo.
fn gritar(texto: &str) {
    if writeln!(io::stderr(), "{texto}").is_err() {
        eprintln!("{texto}");
    }
}

/// Veredito legível por máquina: primeira linha o estado, o resto o detalhe.
fn registrar(estado: &str, detalhe: &str) {
    let Some(caminho) = variavel(VARIAVEL_VEREDITO) else {
        return;
    };
    if let Err(erro) = fs::write(&caminho, format!("{estado}\n{detalhe}\n")) {
        gritar(&format!(
            "SARA-CORPUS: não consegui escrever o veredito em {caminho}: {erro}"
        ));
    }
}

#[test]
fn five_personal_projects_have_no_blocking_false_positive() {
    let mut ausentes = Vec::new();
    let mut conferidos = Vec::new();
    let mut reprovados = Vec::new();

    for projeto in CORPUS {
        let (caminho, origem) = resolver(projeto);
        if !caminho.is_dir() {
            ausentes.push(format!(
                "{} — {} (via {origem})",
                projeto.nome,
                caminho.display()
            ));
            continue;
        }
        match check_project(&CheckRequest {
            project: caminho.clone(),
            engine: EngineChoice::Auto,
            profiles: vec![Profile::Desktop, Profile::Android],
            allow: Vec::new(),
        }) {
            Err(erro) => reprovados.push(format!(
                "{} — {}: {erro:#}",
                projeto.nome,
                caminho.display()
            )),
            Ok(report) if report.has_errors() => reprovados.push(format!(
                "{} — {} produziu erro(s): {:?}",
                projeto.nome,
                caminho.display(),
                report.diagnostics
            )),
            Ok(_) => conferidos.push(format!("{} — {}", projeto.nome, caminho.display())),
        }
    }

    // Reprovado ganha de inconclusivo: conflito comprovado é fato, ausência é a falta
    // de um. Um projeto que sumiu não apaga o erro que outro mostrou.
    if !reprovados.is_empty() {
        let mut detalhe = reprovados.join("\n");
        if !ausentes.is_empty() {
            detalhe.push_str(&format!(
                "\n(e {} projeto(s) sequer estavam lá: {})",
                ausentes.len(),
                ausentes.join("; ")
            ));
        }
        registrar("reprovado", &detalhe);
        panic!(
            "SARA-CORPUS: reprovado — o confronto da ADR 0012 §3 encontrou conflito \
             bloqueante no corpus:\n{detalhe}\n\nIsto é resultado, não ausência: os \
             projetos acima estavam no lugar declarado e foram lidos."
        );
    }

    if !ausentes.is_empty() {
        let detalhe = ausentes.join("\n");
        registrar("inconclusivo", &detalhe);
        gritar(&format!(
            "\n\
             ┌─ SARA-CORPUS: INCONCLUSIVO ─────────────────────────────────────────────\n\
             │ {} de {} projetos do corpus fora do lugar declarado:\n\
             │   {}\n\
             │\n\
             │ Não poder conferir não é ter conferido. A suíte segue verde porque corpus\n\
             │ ausente não é defeito da Sara, e este bloco é o registro de que o confronto\n\
             │ da ADR 0012 §3 NÃO aconteceu — verde aqui não conta como aprovação.\n\
             │\n\
             │ Aponte {VARIAVEL_RAIZ} ou a variável do projeto para o lugar certo, ou\n\
             │ registre a indisponibilidade como bloqueio externo. `tools/check_corpus.sh`\n\
             │ devolve 2 neste estado, e 0 só quando os cinco foram lidos.\n\
             └─────────────────────────────────────────────────────────────────────────",
            ausentes.len(),
            CORPUS.len(),
            ausentes.join("\n│   ")
        ));
        return;
    }

    registrar("aprovado", &conferidos.join("\n"));
    println!(
        "SARA-CORPUS: aprovado — {} de {} projetos lidos, nenhum conflito bloqueante:\n  {}",
        conferidos.len(),
        CORPUS.len(),
        conferidos.join("\n  ")
    );
}
