//! Fitness functions de governança.
//!
//! Estes testes não verificam o comportamento do Sara: verificam que decisões
//! arquiteturais registradas continuam válidas. São o mecanismo executável que a
//! seção de conformidade de cada ADR exige, no lugar de uma regra lembrada.
//! Referência: `docs/AUDITORIA-ARQUITETURAL.md`, seção 4.

use std::{collections::BTreeSet, fs, path::PathBuf};

use serde_json::Value;

fn raiz() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn ler(relativo: &str) -> String {
    let caminho = raiz().join(relativo);
    fs::read_to_string(&caminho).unwrap_or_else(|erro| panic!("{}: {erro}", caminho.display()))
}

// ---------------------------------------------------------------------------
// F1 - escopo (ADR 0001)
// ---------------------------------------------------------------------------

/// Dependências autorizadas pela ADR 0001. Acrescentar uma linha aqui é uma
/// decisão arquitetural: exige justificativa escrita, e no caso de framework,
/// runtime ou serviço remoto exige uma ADR que substitua a 0001.
const DEPENDENCIAS_AUTORIZADAS: &[&str] = &[
    "anyhow",
    "clap",
    "regex",
    "serde",
    "serde_json",
    "toml",
    "tree-sitter",
    "tree-sitter-gdscript",
    "tree-sitter-lua",
    "walkdir",
    // desenvolvimento
    "assert_cmd",
    "predicates",
    "tempfile",
];

fn dependencias_declaradas() -> Vec<String> {
    let manifesto = ler("Cargo.toml");
    let mut dentro = false;
    let mut encontradas = Vec::new();
    for linha in manifesto.lines() {
        let limpa = linha.trim();
        if limpa.starts_with('[') {
            dentro = limpa == "[dependencies]" || limpa == "[dev-dependencies]";
            continue;
        }
        if !dentro || limpa.is_empty() || limpa.starts_with('#') {
            continue;
        }
        if let Some((nome, _)) = limpa.split_once('=') {
            encontradas.push(nome.trim().to_owned());
        }
    }
    encontradas
}

#[test]
fn adr_0001_nao_admite_dependencia_fora_da_lista_autorizada() {
    let intrusas = dependencias_declaradas()
        .into_iter()
        .filter(|nome| !DEPENDENCIAS_AUTORIZADAS.contains(&nome.as_str()))
        .collect::<Vec<_>>();
    assert!(
        intrusas.is_empty(),
        "dependência não autorizada: {intrusas:?}. A ADR 0001 proíbe iniciar editor, \
         renderizador, áudio, física geral, loja de assets ou cadeia multiplataforma \
         próprios. Se a dependência é legítima, acrescente-a a DEPENDENCIAS_AUTORIZADAS \
         com justificativa; se ela abre uma dessas frentes, escreva a ADR que substitui a 0001."
    );
}

#[test]
fn adr_0001_nao_admite_dependencia_autorizada_que_sumiu() {
    let declaradas = dependencias_declaradas();
    let orfas = DEPENDENCIAS_AUTORIZADAS
        .iter()
        .filter(|nome| !declaradas.iter().any(|item| item == *nome))
        .collect::<Vec<_>>();
    assert!(
        orfas.is_empty(),
        "a lista autorizada tem entradas que o manifesto não usa mais: {orfas:?}. \
         Remova-as para que a lista continue sendo a verdade e não um histórico."
    );
}

// ---------------------------------------------------------------------------
// F3 - fronteira núcleo x adapter (achado A1)
// ---------------------------------------------------------------------------

/// Vocabulário que só um adapter tem o direito de conhecer.
const TOKENS_DE_ENGINE: &[&str] = &[
    "godot",
    "defold",
    "gdscript",
    "tween",
    "input_binding",
    "gui_script",
    "render_script",
];

/// Os únicos arquivos do núcleo autorizados a ramificar por engine. A lista é
/// fechada de propósito: ela cresce por decisão, não por descuido.
const PONTOS_DE_VARIACAO: &[&str] = &[
    "config.rs",
    "init.rs",
    "model.rs",
    "parser.rs",
    "scanner.rs",
];

fn menciona_engine(texto: &str) -> Vec<&'static str> {
    let minusculo = texto.to_lowercase();
    TOKENS_DE_ENGINE
        .iter()
        .filter(|token| minusculo.contains(*token))
        .copied()
        .collect()
}

#[test]
fn codigo_compartilhado_entre_adapters_nao_conhece_engine() {
    let achados = menciona_engine(&ler("src/adapters/common.rs"));
    assert!(
        achados.is_empty(),
        "src/adapters/common.rs menciona {achados:?}. Código compartilhado pelos dois \
         adapters não pode carregar semântica de um deles: passe a variação como dado, \
         no modelo de common::BlockSyntax. Achado A1 da auditoria."
    );
}

#[test]
fn variacao_por_engine_no_nucleo_fica_nos_pontos_declarados() {
    let mut fora = Vec::new();
    for entrada in fs::read_dir(raiz().join("src")).expect("src/") {
        let caminho = entrada.expect("entrada").path();
        if caminho.extension().and_then(|valor| valor.to_str()) != Some("rs") {
            continue;
        }
        let nome = caminho
            .file_name()
            .and_then(|valor| valor.to_str())
            .unwrap_or_default()
            .to_owned();
        if PONTOS_DE_VARIACAO.contains(&nome.as_str()) {
            continue;
        }
        let achados = menciona_engine(&fs::read_to_string(&caminho).expect("leitura"));
        if !achados.is_empty() {
            fora.push(format!("{nome}: {achados:?}"));
        }
    }
    assert!(
        fora.is_empty(),
        "surgiu variação por engine fora dos pontos declarados: {fora:?}. Ou a semântica \
         pertence a um adapter, ou o arquivo entra em PONTOS_DE_VARIACAO por decisão \
         registrada. Achado A1 da auditoria."
    );
}

// ---------------------------------------------------------------------------
// F5 - fonte única de compatibilidade (achado A7)
// ---------------------------------------------------------------------------

#[test]
fn contrato_de_compatibilidade_cobre_toda_extensao_que_o_scanner_aceita() {
    let scanner = ler("src/scanner.rs");
    let inicio = scanner
        .find("fn supported(")
        .expect("fn supported ausente em src/scanner.rs");
    let corpo = &scanner[inicio..];
    let fim = corpo
        .find("\n}\n")
        .map(|pos| pos + 2)
        .unwrap_or(corpo.len());
    let documento = ler("docs/COMPATIBILIDADE.md");

    let mut ausentes = Vec::new();
    for pedaco in corpo[..fim].split('"').skip(1).step_by(2) {
        if pedaco.is_empty() || !documento.contains(pedaco) {
            ausentes.push(pedaco.to_owned());
        }
    }
    assert!(
        ausentes.is_empty(),
        "o scanner aceita extensões que docs/COMPATIBILIDADE.md não declara: {ausentes:?}. \
         O contrato publicado e o código precisam ser a mesma verdade. Achado A7 da auditoria."
    );
}

// ---------------------------------------------------------------------------
// F7 - construções reconhecidas, nos dois sentidos (achado A7)
// ---------------------------------------------------------------------------

/// Lê a tabela "Construções reconhecidas" do contrato publicado e devolve uma
/// linha por construção, no formato `engine|eixo|token`.
fn construcoes_do_contrato() -> BTreeSet<String> {
    let documento = ler("docs/COMPATIBILIDADE.md");
    let inicio = documento
        .find("## Construções reconhecidas")
        .expect("a seção 'Construções reconhecidas' sumiu de docs/COMPATIBILIDADE.md");
    let resto = &documento[inicio..];
    let fim = resto[3..]
        .find("\n## ")
        .map(|pos| pos + 3)
        .unwrap_or(resto.len());

    let mut linhas = BTreeSet::new();
    for linha in resto[..fim].lines() {
        let limpa = linha.trim();
        if !limpa.starts_with('|') || limpa.contains("---") || limpa.contains("Construção") {
            continue;
        }
        let celulas = limpa
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        if celulas.len() != 3 {
            continue;
        }
        linhas.insert(format!(
            "{}|{}|{}",
            celulas[0].to_lowercase(),
            celulas[1],
            celulas[2].trim_matches('`')
        ));
    }
    linhas
}

fn construcoes_declaradas() -> BTreeSet<String> {
    sara_ai_first::adapters::recognized_constructs()
        .into_iter()
        .map(|item| format!("{}|{}|{}", item.engine, item.axis.label(), item.token))
        .collect()
}

#[test]
fn a7_contrato_publicado_e_adapters_declaram_as_mesmas_construcoes() {
    let declaradas = construcoes_declaradas();
    let publicadas = construcoes_do_contrato();
    let caladas = declaradas
        .difference(&publicadas)
        .cloned()
        .collect::<Vec<_>>();
    let prometidas = publicadas
        .difference(&declaradas)
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        caladas.is_empty(),
        "os adapters reconhecem construções que docs/COMPATIBILIDADE.md não declara: \
         {caladas:?}. Quem lê o contrato precisa saber o que a ferramenta enxerga. Achado A7."
    );
    assert!(
        prometidas.is_empty(),
        "docs/COMPATIBILIDADE.md promete construções que nenhum adapter reconhece: \
         {prometidas:?}. Contrato que promete a mais é pior que contrato ausente: ele \
         é acreditado. Achado A7."
    );
}

/// Fonte do adapter **sem** o bloco `CONSTRUCTS`. Sem essa remoção o teste abaixo é
/// vacuoso: a própria declaração satisfaz a busca, e um token inventado passaria.
fn fonte_do_adapter_sem_a_declaracao(engine: &str) -> String {
    let fonte = ler(&format!("src/adapters/{engine}.rs"));
    let inicio = fonte
        .find("pub const CONSTRUCTS")
        .unwrap_or_else(|| panic!("CONSTRUCTS sumiu de src/adapters/{engine}.rs"));
    let fim = inicio
        + fonte[inicio..]
            .find("];")
            .expect("bloco CONSTRUCTS sem fechamento")
        + 2;
    format!("{}{}", &fonte[..inicio], &fonte[fim..])
}

#[test]
fn a7_toda_construcao_declarada_existe_no_fonte_do_adapter() {
    let mut ausentes = Vec::new();
    for item in sara_ai_first::adapters::recognized_constructs() {
        let fonte = fonte_do_adapter_sem_a_declaracao(&item.engine.to_string());
        if !fonte.contains(item.token) {
            ausentes.push(format!("{}: {}", item.engine, item.token));
        }
    }
    assert!(
        ausentes.is_empty(),
        "há construção declarada cujo token não aparece no fonte do adapter: {ausentes:?}. \
         A lista virou histórico em vez de contrato. Achado A7."
    );
}

#[test]
fn adr_0005_lista_de_construcoes_do_defold_esta_congelada() {
    let defold = construcoes_declaradas()
        .into_iter()
        .filter(|linha| linha.starts_with("defold|"))
        .count();
    assert_eq!(
        defold, 5,
        "o adapter Defold passou a reconhecer outro conjunto de construções. Pela ADR 0005 \
         ele está congelado: sem regra nova, com as regressões históricas preservadas. \
         Mudar este número exige a ADR que substitua a 0005."
    );
}

// ---------------------------------------------------------------------------
// ADR 0005 - Defold congelado como corpus de regressão
// ---------------------------------------------------------------------------

#[test]
fn adr_0005_regressoes_historicas_defold_continuam_no_repositorio() {
    for fixture in [
        "tests/fixtures/defold_animation_red",
        "tests/fixtures/defold_animation_green",
        "tests/fixtures/defold_input_red",
        "tests/fixtures/defold_input_green",
    ] {
        assert!(
            raiz().join(fixture).is_dir(),
            "{fixture} desapareceu. As duas regressões históricas do Defold são a única \
             prova causal do Portão 0: elas ficam no repositório mesmo com o foco em Godot. \
             ADR 0005."
        );
    }

    let cenarios = ler("tests/cli.rs");
    for teste in [
        "defold_animation_red_fails_and_green_passes",
        "defold_input_is_profile_aware",
    ] {
        assert!(
            cenarios.contains(teste),
            "o cenário {teste} sumiu de tests/cli.rs. Congelar o Defold significa parar de \
             acrescentar regra, não parar de provar que as regressões históricas falham. ADR 0005."
        );
    }
}

// ---------------------------------------------------------------------------
// F2 - contrato estrito de relatório e códigos de saída (ADR 0006)
// ---------------------------------------------------------------------------

/// Forma congelada do relatório JSON. Cada entrada é um caminho de chave; `[]`
/// indica elemento de lista. Acrescentar, remover ou renomear qualquer linha aqui
/// é uma quebra de contrato e exige subir `REPORT_SCHEMA_VERSION` no mesmo commit.
const FORMA_DO_RELATORIO: &[&str] = &[
    "claims",
    "claims[].confidence",
    "claims[].controller",
    "claims[].flow",
    "claims[].operation",
    "claims[].owner",
    "claims[].resource",
    "claims[].resource.engine",
    "claims[].resource.kind",
    "claims[].resource.profile",
    "claims[].resource.property",
    "claims[].resource.scope",
    "claims[].resource.target",
    "claims[].span",
    "claims[].span.column",
    "claims[].span.line",
    "claims[].span.path",
    "diagnostics",
    "diagnostics[].explanation",
    "diagnostics[].owners",
    "diagnostics[].primary",
    "diagnostics[].primary.column",
    "diagnostics[].primary.line",
    "diagnostics[].primary.path",
    "diagnostics[].related",
    "diagnostics[].related[].column",
    "diagnostics[].related[].line",
    "diagnostics[].related[].path",
    "diagnostics[].remediation",
    "diagnostics[].resource",
    "diagnostics[].rule",
    "diagnostics[].severity",
    "engine",
    "files_scanned",
    "profiles",
    "project",
    "schema_version",
    "tool_version",
];

fn caminhos_de_chave(valor: &Value, prefixo: &str, saida: &mut BTreeSet<String>) {
    match valor {
        Value::Object(mapa) => {
            for (chave, filho) in mapa {
                let caminho = if prefixo.is_empty() {
                    chave.clone()
                } else {
                    format!("{prefixo}.{chave}")
                };
                saida.insert(caminho.clone());
                caminhos_de_chave(filho, &caminho, saida);
            }
        }
        Value::Array(itens) => {
            for item in itens {
                caminhos_de_chave(item, &format!("{prefixo}[]"), saida);
            }
        }
        _ => {}
    }
}

fn forma_atual() -> BTreeSet<String> {
    use sara_ai_first::{CheckRequest, check_project, config::EngineChoice, config::Profile};

    let mut caminhos = BTreeSet::new();
    for fixture in ["defold_animation_red", "defold_input_red"] {
        let relatorio = check_project(&CheckRequest {
            project: raiz().join("tests/fixtures").join(fixture),
            engine: EngineChoice::Auto,
            profiles: vec![Profile::Desktop, Profile::Android],
            allow: Vec::new(),
        })
        .unwrap_or_else(|erro| panic!("{fixture}: {erro:#}"));
        let valor: Value = serde_json::from_str(&sara_ai_first::report::json(&relatorio).unwrap())
            .expect("json inválido");
        caminhos_de_chave(&valor, "", &mut caminhos);
    }
    caminhos
}

#[test]
fn adr_0006_forma_do_relatorio_json_esta_congelada() {
    let atual = forma_atual();
    let declarada = FORMA_DO_RELATORIO
        .iter()
        .map(|item| (*item).to_owned())
        .collect::<BTreeSet<_>>();
    let novos = atual.difference(&declarada).cloned().collect::<Vec<_>>();
    let sumidos = declarada.difference(&atual).cloned().collect::<Vec<_>>();
    assert!(
        novos.is_empty() && sumidos.is_empty(),
        "a forma do relatório mudou. Campos novos: {novos:?}. Campos que sumiram: {sumidos:?}. \
         Pela ADR 0006 o contrato é estrito: qualquer mudança de forma sobe \
         REPORT_SCHEMA_VERSION no mesmo commit. Atualize FORMA_DO_RELATORIO e a versão juntas."
    );
}

#[test]
fn adr_0006_versao_do_esquema_e_a_do_modelo() {
    use sara_ai_first::{CheckRequest, check_project, config::EngineChoice, config::Profile};

    let relatorio = check_project(&CheckRequest {
        project: raiz().join("tests/fixtures/defold_animation_green"),
        engine: EngineChoice::Auto,
        profiles: vec![Profile::Desktop],
        allow: Vec::new(),
    })
    .expect("relatório");
    assert_eq!(
        relatorio.schema_version,
        sara_ai_first::model::REPORT_SCHEMA_VERSION,
        "o relatório emitiu versão diferente da constante do modelo. ADR 0006."
    );
}

#[test]
fn adr_0006_codigos_de_saida_continuam_exercitados() {
    let cenarios = ler("tests/cli.rs");
    for (codigo, significado) in [
        (".code(1)", "conflito comprovado"),
        (".code(2)", "análise incompleta"),
    ] {
        assert!(
            cenarios.contains(codigo),
            "nenhum cenário de tests/cli.rs exercita {codigo} ({significado}). \
             Os três códigos de saída são contrato pela ADR 0006 e precisam de teste vivo."
        );
    }
    assert!(
        cenarios.contains(".success()"),
        "nenhum cenário exercita a saída 0. ADR 0006."
    );
}

// ---------------------------------------------------------------------------
// F4 - quantum do binário `sara` (ADR 0007)
// ---------------------------------------------------------------------------

/// Binários autorizados. `sara` é o quantum offline medido no 0.1.0; `sara-observe`
/// é o experimento do Marco 7, que pode exigir Godot instalado sem contaminar o portão.
const BINARIOS_AUTORIZADOS: &[&str] = &["sara", "sara-observe"];

/// Nomes de binário lidos do `Cargo.toml`.
fn binarios_declarados() -> Vec<String> {
    let manifesto = ler("Cargo.toml");
    let mut dentro = false;
    let mut nomes = Vec::new();
    for linha in manifesto.lines() {
        let limpa = linha.trim();
        if limpa.starts_with('[') {
            dentro = limpa == "[[bin]]";
            continue;
        }
        if !dentro {
            continue;
        }
        if let Some(valor) = limpa.strip_prefix("name") {
            nomes.push(
                valor
                    .trim_start_matches([' ', '=', '"'])
                    .trim_end_matches('"')
                    .to_owned(),
            );
        }
    }
    nomes
}

// ---------------------------------------------------------------------------
// ADR 0011 - o Marco 7 não começa sem comparação com ferramenta existente
// ---------------------------------------------------------------------------

/// Freio, não escopo. Em 26/08/2026 descobriu-se que a unidade de evidência que a ADR
/// 0004 manda construir já existe pronta: a `extension-automation-bridge` oficial da
/// Defold e pelo menos quatro implementações comunitárias em Godot. A ADR 0004 não tem
/// seção de opções consideradas e autorizou construir sem perguntar se já existia.
///
/// Este teste garante que o segundo binário não nasça antes da ADR que compara.
#[test]
fn adr_0011_observe_exige_adr_de_comparacao() {
    if !binarios_declarados()
        .iter()
        .any(|nome| nome == "sara-observe")
    {
        return;
    }
    let comparacao = fs::read_dir(raiz().join("docs/decisoes"))
        .expect("docs/decisoes")
        .filter_map(|entrada| entrada.ok())
        .any(|entrada| {
            let nome = entrada.file_name().to_string_lossy().to_lowercase();
            nome.contains("comparacao") && nome.ends_with(".md") && !nome.starts_with("0011")
        });
    assert!(
        comparacao,
        "o binário `sara-observe` existe, mas não há ADR de comparação em docs/decisoes/. \
         Pela ADR 0011 o Marco 7 não começa sem confrontar o spike da ADR 0004, item a item \
         contra as sete fitness functions dela, com as ferramentas que já entregam a mesma \
         unidade de evidência — a extension-automation-bridge oficial da Defold e as \
         implementações comunitárias em Godot. Escreva a ADR de comparação primeiro."
    );
}

#[test]
fn adr_0007_apenas_binarios_autorizados() {
    let intrusos = binarios_declarados()
        .into_iter()
        .filter(|nome| !BINARIOS_AUTORIZADOS.contains(&nome.as_str()))
        .collect::<Vec<_>>();
    assert!(
        intrusos.is_empty(),
        "binário não autorizado: {intrusos:?}. Pela ADR 0007, `sara` é um quantum \
         independente — sem Godot, sem rede, sem runtime externo — e todo experimento \
         que exija ambiente vive em binário próprio."
    );
}
