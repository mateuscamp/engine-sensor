use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result};
use regex::Regex;

use crate::{
    adapters::{AdapterOutput, Axis, Construct, common},
    config::Profile,
    model::{
        Confidence, Diagnostic, Engine, OwnershipClaim, ResourceKey, ResourceKind, Severity, Span,
    },
    parser::ParsedSource,
};

pub fn analyze(
    project: &Path,
    sources: &[ParsedSource],
    profiles: &[Profile],
) -> Result<AdapterOutput> {
    let declared_actions = declared_actions(project)?;
    let mut output = AdapterOutput::default();
    for source in sources {
        animation_claims(source, &mut output);
        depth_claims(source, &mut output);
        input_claims(source, &declared_actions, &mut output);
    }
    // O conflito de canal físico só existe onde toque e mouse são o mesmo dedo. Pelo
    // mesmo motivo que no adapter Defold, fora do perfil android não há o que declarar.
    if profiles.contains(&Profile::Android) {
        for source in sources {
            physical_channel_claims(source, &mut output);
        }
    }
    diagnose_animations(&output.claims, sources, &mut output.diagnostics);
    diagnose_inputs(&output.claims, sources, &mut output.diagnostics);
    diagnose_physical_channels(
        &output.claims,
        emulates_mouse_from_touch(project),
        &mut output.diagnostics,
    );
    Ok(output)
}

/// Construções de API que este adapter reconhece. É a lista que
/// `docs/COMPATIBILIDADE.md` publica e que `tests/governanca.rs` confere nos dois
/// sentidos (achado A7).
pub const CONSTRUCTS: &[Construct] = &[
    Construct {
        engine: Engine::Godot,
        axis: Axis::Animation,
        token: "tween_property",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Animation,
        token: "kill",
    },
    // O relógio da trajetória. Nenhuma destas toca a propriedade animada, e todas
    // mudam o que acontece na tela.
    Construct {
        engine: Engine::Godot,
        axis: Axis::Animation,
        token: "pause",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Animation,
        token: "play",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Animation,
        token: "stop",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Animation,
        token: "set_speed_scale",
    },
    // Profundidade de desenho. Os dois decidem quem aparece na frente de quem, e
    // nenhum é propriedade animada: são mecanismos independentes com o mesmo efeito.
    Construct {
        engine: Engine::Godot,
        axis: Axis::Depth,
        token: "z_index",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Depth,
        token: "move_child",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "_input",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "_unhandled_input",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "_gui_input",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "set_input_as_handled",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "InputEventScreenTouch",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "InputEventScreenDrag",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "InputEventMouseButton",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "InputEventMouseMotion",
    },
    Construct {
        engine: Engine::Godot,
        axis: Axis::Input,
        token: "emulate_mouse_from_touch",
    },
];

/// Marca de propriedade das declarações de canal físico. Separa-as das declarações
/// por ação: são regras diferentes sobre o mesmo eixo.
const PHYSICAL_CHANNEL: &str = "physical_channel";

/// Canal físico de onde a entrada vem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Channel {
    Mouse,
    Touch,
    Other,
}

/// Canal de uma classe de evento do Godot.
///
/// No Defold o canal vem do `game.input_binding`, que é obrigatório. No Godot não há
/// arquivo equivalente quando o projeto não usa mapa de ações: o canal está na classe
/// do evento testada no ramo. Ver ADR 0010.
fn channel_of(event: &str) -> Channel {
    match event {
        "InputEventScreenTouch" | "InputEventScreenDrag" => Channel::Touch,
        "InputEventMouseButton" | "InputEventMouseMotion" => Channel::Mouse,
        _ => Channel::Other,
    }
}

/// Mouse e toque são o mesmo dedo no aparelho: um toque entrega os dois eventos.
fn physical_duplicate(first: Channel, second: Channel) -> bool {
    matches!(
        (first, second),
        (Channel::Mouse, Channel::Touch) | (Channel::Touch, Channel::Mouse)
    )
}

/// O Godot emula mouse a partir do toque por padrão. Desligar isso no `project.godot`
/// separa os canais de verdade, e é a única saída provável pelo texto.
fn emulates_mouse_from_touch(project: &Path) -> bool {
    let Ok(source) = fs::read_to_string(project.join("project.godot")) else {
        return true;
    };
    !source
        .lines()
        .filter_map(|line| line.split_once('='))
        .any(|(chave, valor)| {
            chave.trim().ends_with("emulate_mouse_from_touch") && valor.trim() == "false"
        })
}

/// Sintaxe de bloco do GDScript. Fica aqui, e não no núcleo compartilhado: o
/// `common` não deve saber qual engine está analisando (achado A1).
const GDSCRIPT_BLOCKS: common::BlockSyntax = common::BlockSyntax {
    opens_branch: &["if ", "elif "],
    condition_end: common::ConditionEnd::LineEndsWith(':'),
    closes_body_prefix: &["elif "],
    closes_body_exact: &["else:"],
};

fn animation_claims(source: &ParsedSource, output: &mut AdapterOutput) {
    let assignments = tween_assignments(source);
    let mut trajetorias: Vec<OwnershipClaim> = Vec::new();
    let naked_tween =
        Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)\s*\.\s*tween_property\s*\(").expect("regex válida");
    for call in &source.calls {
        let callee = if call.callee.ends_with(".tween_property") {
            call.callee.clone()
        } else if call.callee == "tween_property" {
            let line = source
                .source
                .lines()
                .nth(call.span.line.saturating_sub(1) as usize)
                .unwrap_or_default();
            let column = call.span.column.saturating_sub(1) as usize;
            let Some(captures) = naked_tween.captures_iter(line).find(|captures| {
                captures
                    .get(0)
                    .is_some_and(|matched| matched.start() <= column && column < matched.end())
            }) else {
                continue;
            };
            format!("{}.tween_property", &captures[1])
        } else {
            continue;
        };
        if call.args.len() < 2 {
            continue;
        }
        let (target, target_confidence) = common::normalized_expression(&call.args[0]);
        let (property, property_confidence) = common::normalized_property(&call.args[1]);
        let confidence = if target_confidence == Confidence::Proven
            && property_confidence == Confidence::Proven
        {
            Confidence::Proven
        } else {
            Confidence::Ambiguous
        };
        let prefix = callee.strip_suffix(".tween_property").unwrap_or(&callee);
        let controller = assignments
            .get(&(call.owner.clone(), prefix.to_owned()))
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "{}::{}::{}@{}",
                    source.path, call.owner, prefix, call.span.line
                )
            });
        let claim = OwnershipClaim {
            resource: ResourceKey {
                engine: Engine::Godot,
                kind: ResourceKind::AnimationProperty,
                scope: common::symbolic_scope(
                    &source.path,
                    &call.owner,
                    &target,
                    &call.control_path,
                ),
                target,
                property,
                profile: None,
            },
            owner: format!("{}::{}", source.path, call.owner),
            span: call.span.clone(),
            confidence,
            operation: "Tween.tween_property".to_owned(),
            controller,
            flow: call.control_path.clone(),
        };
        if confidence == Confidence::Ambiguous {
            output.diagnostics.push(common::unresolved_diagnostic(
                &claim,
                "o alvo ou a propriedade do Tween",
            ));
        }
        trajetorias.push(claim);
    }
    clock_claims(source, &assignments, &trajetorias, output);
    output.claims.append(&mut trajetorias);
}

/// Métodos que mexem no relógio de um Tween sem tocar na propriedade animada.
///
/// A Sara declarava alvo, propriedade e dono, e nenhuma destas três coordenadas muda
/// quando alguém pausa ou desacelera a trajetória. O resultado na tela muda inteiro:
/// uma bomba parada para sempre com o Tween pausado é indistinguível de uma queimando,
/// e um pavio que passou a queimar na metade da velocidade é indistinguível de um que
/// não passou.
const CLOCK_METHODS: &[&str] = &["pause", "play", "stop", "set_speed_scale"];

/// Declara quem mexe no relógio de uma trajetória já declarada.
///
/// **Só declara, e de propósito.** Pela ADR 0012 §3 uma capacidade generalizável é
/// confrontada com o corpus antes de virar regra; abrir o olho e reclamar no mesmo
/// commit impede saber qual das duas coisas produziu o ruído. Foi assim que a ADR 0010
/// entrou: as declarações invisíveis apareceram primeiro, sem um único diagnóstico novo.
fn clock_claims(
    source: &ParsedSource,
    assignments: &BTreeMap<(String, String), String>,
    trajetorias: &[OwnershipClaim],
    output: &mut AdapterOutput,
) {
    if trajetorias.is_empty() {
        return;
    }
    // Um Tween guardado em campo do nó é criado numa função e pausado em outra -- que é
    // justamente o caso interessante. Resolver só por (função, variável) perderia todos
    // eles, então o nome também resolve sozinho, e apenas quando ele é único no arquivo.
    let mut por_variavel: BTreeMap<&str, Option<&str>> = BTreeMap::new();
    for ((_, variavel), controlador) in assignments {
        por_variavel
            .entry(variavel.as_str())
            .and_modify(|atual| {
                if *atual != Some(controlador.as_str()) {
                    *atual = None;
                }
            })
            .or_insert(Some(controlador.as_str()));
    }

    let laços = tween_loop_bindings(source);
    for call in &source.calls {
        let Some((prefixo, metodo)) = call.callee.rsplit_once('.') else {
            continue;
        };
        if !CLOCK_METHODS.contains(&metodo) {
            continue;
        }
        let variavel = prefixo.strip_prefix("self.").unwrap_or(prefixo);
        let diretos = assignments
            .get(&(call.owner.clone(), variavel.to_owned()))
            .map(String::as_str)
            .or_else(|| por_variavel.get(variavel).copied().flatten())
            .map(|controlador| vec![controlador])
            .unwrap_or_default();
        let controladores = if diretos.is_empty() {
            // Sem saber qual Tween é, não há o que declarar -- salvo quando o nome é a
            // variável de um laço sobre uma lista de Tweens conhecidos, que foi como o
            // caso que originou esta capacidade estava escrito.
            laços
                .get(&(call.owner.clone(), variavel.to_owned()))
                .map_or_else(Vec::new, |nomes| {
                    nomes
                        .iter()
                        .filter_map(|nome| {
                            assignments
                                .get(&(call.owner.clone(), nome.clone()))
                                .map(String::as_str)
                                .or_else(|| por_variavel.get(nome.as_str()).copied().flatten())
                        })
                        .collect()
                })
        } else {
            diretos
        };
        let mut vistos = BTreeSet::new();
        for controlador in controladores {
            for trajetoria in trajetorias
                .iter()
                .filter(|item| item.controller == controlador)
            {
                if !vistos.insert(trajetoria.resource.clone()) {
                    continue;
                }
                output.claims.push(OwnershipClaim {
                    resource: trajetoria.resource.clone(),
                    owner: format!("{}::{}", source.path, call.owner),
                    span: call.span.clone(),
                    confidence: Confidence::Proven,
                    operation: format!("Tween.{metodo}"),
                    controller: controlador.to_owned(),
                    flow: call.control_path.clone(),
                });
            }
        }
    }
}

/// Variáveis de laço que percorrem uma lista literal de Tweens, por função.
///
/// `for animacao: Tween in [_caminhada, _espera]:` faz `animacao` valer pelos dois, e
/// sem isto a chamada de relógio dentro do laço não se liga a trajetória nenhuma. É a
/// forma exata em que o `set_speed_scale` que motivou esta capacidade estava escrito --
/// uma capacidade que não vê o próprio caso de origem não foi construída, foi anunciada.
fn tween_loop_bindings(source: &ParsedSource) -> BTreeMap<(String, String), Vec<String>> {
    let laço = Regex::new(
        r"(?m)^[ 	]*for[ 	]+([A-Za-z_][A-Za-z0-9_]*)(?:[ 	]*:[ 	]*[A-Za-z_][A-Za-z0-9_.]*)?[ 	]+in[ 	]*\[([^\]
]*)\][ 	]*:",
    )
    .expect("regex válida");
    let mut ligações = BTreeMap::new();
    for captures in laço.captures_iter(&source.source) {
        let linha = source.source[..captures.get(0).unwrap().start()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count() as u32
            + 1;
        let dono = source
            .functions
            .iter()
            .find(|function| linha >= function.start_line && linha <= function.end_line)
            .map(|function| function.name.clone())
            .unwrap_or_else(|| "<arquivo>".to_owned());
        let nomes = captures[2]
            .split(',')
            .map(|item| {
                let limpo = item.trim();
                limpo.strip_prefix("self.").unwrap_or(limpo).to_owned()
            })
            .filter(|item| {
                !item.is_empty()
                    && item
                        .chars()
                        .all(|letra| letra.is_alphanumeric() || letra == '_')
            })
            .collect::<Vec<_>>();
        if !nomes.is_empty() {
            ligações.insert((dono, captures[1].to_owned()), nomes);
        }
    }
    ligações
}

fn tween_assignments(source: &ParsedSource) -> BTreeMap<(String, String), String> {
    let expression = Regex::new(
        r"(?m)^\s*(?:var\s+)?([A-Za-z_][A-Za-z0-9_]*)(?:\s*:\s*[A-Za-z_][A-Za-z0-9_.]*)?\s*(?::=|=)\s*(?:[A-Za-z_][A-Za-z0-9_.]*\.)?create_tween\(\)(?:\.[A-Za-z_][A-Za-z0-9_]*\([^\n)]*\))*",
    )
    .unwrap();
    let mut assignments = BTreeMap::new();
    for captures in expression.captures_iter(&source.source) {
        let line = source.source[..captures.get(0).unwrap().start()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count() as u32
            + 1;
        let owner = source
            .functions
            .iter()
            .find(|function| line >= function.start_line && line <= function.end_line)
            .map(|function| function.name.clone())
            .unwrap_or_else(|| "<arquivo>".to_owned());
        let variable = captures[1].to_owned();
        assignments.insert(
            (owner.clone(), variable.clone()),
            format!("{}::{owner}::{variable}@{line}", source.path),
        );
    }
    assignments
}

/// Como o relatório publica o eixo de profundidade.
///
/// A propriedade nomeia o **efeito**, não a API: `z_index` e a ordem entre irmãos são
/// coisas diferentes no motor e decidem a mesma coisa na tela. Nomeá-las igual é o que
/// faz as duas caírem no mesmo recurso, com um controlador cada -- que é a forma que a
/// Sara já usa para duas fontes de verdade sobre uma coordenada.
const DEPTH_PROPERTY: &str = "profundidade";
const DEPTH_BY_Z_INDEX: &str = "z_index";
const DEPTH_BY_CHILD_ORDER: &str = "ordem_de_filho";

/// Declara quem decide a profundidade de desenho de cada nó.
///
/// A terceira capacidade que o caso da aranha nomeou (`docs/CASO-DA-ARANHA.md` §5.1), e
/// a única das três que pegou um defeito por conta própria: o fio de seda com
/// `z_index = -1` não apareceu em quadro nenhum, e nenhum teste, portão ou captura viu.
/// A Sara modelava quem **anima** uma propriedade; não modelava quem **desenha na
/// frente de quem**, e um sprite invisível passava por todos os portões.
///
/// **Só declara, sem diagnóstico novo**, como a ADR 0010 e o relógio do Tween entraram.
/// Se profundidade decidida duas vezes merece aviso, é decisão própria com outra rodada
/// de evidência: abrir o olho e reclamar no mesmo commit impede saber qual das duas
/// coisas produziu o ruído.
///
/// **O que ela não sabe, e não finge saber.** `z_index` é relativo ao pai, e o pai é
/// informação de cena (`.tscn`), que esta ferramenta não lê. Por isso cada declaração é
/// sempre sobre **um** nó -- quem decide a profundidade dele --, e nunca sobre a
/// comparação entre dois nós, que é o que exigiria a árvore. Foi exatamente essa
/// relatividade que produziu o defeito de origem; declarar mais do que isto seria
/// inventar a árvore que falta.
fn depth_claims(source: &ParsedSource, output: &mut AdapterOutput) {
    // A busca é a linha inteira, comentário incluído -- e o comentário não escapa por
    // descuido. O próprio caso de origem escreve ``[b]`z_index = -1` nao serve[/b]``
    // dentro de um comentário, e o Gods repete a forma em três lugares: em todas, o `#`
    // cai dentro do prefixo, e prefixo que não termina em ponto não é escrita nesta
    // propriedade. Uma passagem que apagasse comentários foi escrita, medida contra os
    // 2137 arquivos do corpus pessoal e removida: não mudou uma declaração sequer, e
    // nenhum caso da fixture a reprovava.
    let código = &source.source;
    let escrita =
        Regex::new(r"(?m)^[ \t]*([^=\r\n]*?)z_index[ \t]*(\+=|-=|=)").expect("regex válida");
    for captures in escrita.captures_iter(código) {
        let inteiro = captures.get(0).expect("captura 0");
        // `z_index == 3` é leitura, não escrita. Sem look-around no `regex`, a
        // distinção é feita aqui.
        if código.as_bytes().get(inteiro.end()) == Some(&b'=') {
            continue;
        }
        let prefixo = &captures[1];
        // Prefixo vazio é `self`; prefixo que termina em ponto é o nó. Qualquer outro
        // prefixo -- `var `, `push_error("` -- não é escrita nesta propriedade.
        let alvo_bruto = if prefixo.is_empty() {
            "self"
        } else if let Some(sem_ponto) = prefixo.strip_suffix('.') {
            sem_ponto
        } else {
            continue;
        };
        let (target, confidence) = common::normalized_expression(alvo_bruto);
        if confidence != Confidence::Proven {
            continue;
        }
        // O vão aponta para o começo da expressão do alvo, que é onde o tree-sitter
        // põe o vão de uma chamada. Sem isto, a recuada da linha entraria no vão e as
        // duas metades do mesmo eixo apontariam para colunas diferentes.
        let início = captures.get(1).expect("captura 1").start();
        let linha = código[..início]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count() as u32
            + 1;
        let coluna = código[..início]
            .rfind('\n')
            .map_or(início, |quebra| início - quebra - 1) as u32
            + 1;
        let dono = enclosing_function(source, linha);
        output.claims.push(OwnershipClaim {
            resource: depth_resource(source, &dono, &target, ""),
            owner: format!("{}::{dono}", source.path),
            span: Span {
                path: source.path.clone(),
                line: linha,
                column: coluna,
            },
            confidence: Confidence::Proven,
            operation: "CanvasItem.z_index".to_owned(),
            controller: DEPTH_BY_Z_INDEX.to_owned(),
            // Vazio, e o motivo fica escrito: a escrita de propriedade não é um sítio de
            // chamada, então não há caminho de controle a copiar. Inventar um ramo seria
            // pior que declarar nenhum -- e uma regra futura que compare fluxos precisa
            // saber que esta coordenada está em branco por limite, não por acaso.
            flow: String::new(),
        });
    }

    for call in &source.calls {
        let método = call
            .callee
            .rsplit_once('.')
            .map_or(call.callee.as_str(), |(_, nome)| nome);
        // Igualdade exata, e ela importa: `remove_child` termina com o mesmo texto e
        // não decide profundidade nenhuma.
        if método != "move_child" || call.args.is_empty() {
            continue;
        }
        let (target, confidence) = common::normalized_expression(&call.args[0]);
        if confidence != Confidence::Proven {
            continue;
        }
        output.claims.push(OwnershipClaim {
            resource: depth_resource(source, &call.owner, &target, &call.control_path),
            owner: format!("{}::{}", source.path, call.owner),
            span: call.span.clone(),
            confidence: Confidence::Proven,
            operation: "Node.move_child".to_owned(),
            controller: DEPTH_BY_CHILD_ORDER.to_owned(),
            flow: call.control_path.clone(),
        });
    }
}

/// O recurso é o nó, e não o mecanismo: é isso que põe `z_index` e ordem de filho lado
/// a lado quando os dois decidem a profundidade do mesmo nó.
fn depth_resource(
    source: &ParsedSource,
    owner: &str,
    target: &str,
    control_path: &str,
) -> ResourceKey {
    ResourceKey {
        engine: Engine::Godot,
        kind: ResourceKind::DrawOrder,
        scope: common::symbolic_scope(&source.path, owner, target, control_path),
        target: target.to_owned(),
        property: DEPTH_PROPERTY.to_owned(),
        profile: None,
    }
}

/// Função que contém uma linha, ou `<arquivo>` quando a escrita mora no corpo da classe.
fn enclosing_function(source: &ParsedSource, line: u32) -> String {
    source
        .functions
        .iter()
        .find(|function| line >= function.start_line && line <= function.end_line)
        .map(|function| function.name.clone())
        .unwrap_or_else(|| "<arquivo>".to_owned())
}

fn diagnose_animations(
    all_claims: &[OwnershipClaim],
    sources: &[ParsedSource],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut groups: BTreeMap<ResourceKey, Vec<&OwnershipClaim>> = BTreeMap::new();
    for claim in all_claims.iter().filter(|item| {
        item.resource.engine == Engine::Godot
            && item.resource.kind == ResourceKind::AnimationProperty
            && item.confidence == Confidence::Proven
            // Controle de relógio não é uma segunda trajetória, e parear as duas
            // produziria o aviso errado com o texto de outra regra.
            && item.operation == "Tween.tween_property"
    }) {
        groups
            .entry(claim.resource.clone())
            .or_default()
            .push(claim);
    }
    let kill =
        Regex::new(r"([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)\.kill\(\)").unwrap();

    // Um mapa por arquivo que tem declaração, e não por par: varrer as funções de novo
    // a cada comparação seria trabalho repetido sem motivo.
    let mut helpers: BTreeMap<String, BTreeMap<String, BTreeSet<String>>> = BTreeMap::new();
    for claim in groups.values().flatten() {
        if helpers.contains_key(&claim.span.path) {
            continue;
        }
        if let Some(source) = sources.iter().find(|item| item.path == claim.span.path) {
            helpers.insert(claim.span.path.clone(), cancelling_helpers(source, &kill));
        }
    }

    for claims in groups.values_mut() {
        claims.sort_by_key(|item| item.span.line);
        let mut warned_between_owners = false;
        for pair in claims.windows(2) {
            let first = pair[0];
            let second = pair[1];
            if first.owner == second.owner
                && first.flow == second.flow
                && first.controller != second.controller
                && !has_ordering_barrier(first, second, sources)
            {
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-001",
                    Severity::Error,
                    first,
                    second,
                    "dois Tweens distintos começam na mesma função sobre a mesma propriedade",
                    "use um único Tween sequencial, centralize o proprietário ou encerre o Tween anterior antes de criar o próximo",
                ));
            } else if (first.owner != second.owner || first.flow != second.flow)
                && !warned_between_owners
                && !serialized_by_cancel(first, second, sources, &helpers, &kill)
            {
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-001",
                    Severity::Warning,
                    first,
                    second,
                    "duas trajetórias podem animar a mesma propriedade, mas seus ciclos de vida não são prováveis apenas pelo texto",
                    "centralize o proprietário ou registre uma exceção exata se os ciclos forem mutuamente exclusivos",
                ));
                warned_between_owners = true;
            }
        }
    }
}

/// Alvos que o texto encerra explicitamente, na forma `alvo.kill()`.
fn cancelled_targets(text: &str, kill: &Regex) -> BTreeSet<String> {
    kill.captures_iter(text)
        .map(|captures| captures[1].to_owned())
        .collect()
}

/// Funções do arquivo que encerram algum Tween: nome da função para os alvos que ela
/// encerra. É por elas que passa o padrão de dono centralizado.
fn cancelling_helpers(source: &ParsedSource, kill: &Regex) -> BTreeMap<String, BTreeSet<String>> {
    let mut helpers = BTreeMap::new();
    for function in &source.functions {
        let targets = cancelled_targets(&function.text, kill);
        if !targets.is_empty() {
            helpers.insert(function.name.clone(), targets);
        }
    }
    helpers
}

/// Alvos encerrados dentro da função dona, antes da linha da declaração — direto por
/// `alvo.kill()` ou por chamada a uma função que encerra, um nível de indireção.
fn cancelled_before(
    claim: &OwnershipClaim,
    source: &ParsedSource,
    helpers: &BTreeMap<String, BTreeSet<String>>,
    kill: &Regex,
) -> BTreeSet<String> {
    let name = claim.owner.split("::").last().unwrap_or_default();
    let Some(function) = source.functions.iter().find(|item| item.name == name) else {
        return BTreeSet::new();
    };
    let mut targets = BTreeSet::new();
    for (offset, line) in function.text.lines().enumerate() {
        if function.start_line as usize + offset >= claim.span.line as usize {
            break;
        }
        targets.extend(cancelled_targets(line, kill));
        for (helper, cancelled) in helpers {
            if line.contains(&format!("{helper}(")) {
                targets.extend(cancelled.iter().cloned());
            }
        }
    }
    targets
}

/// Duas trajetórias se serializam quando **as duas** encerram o mesmo alvo antes de
/// começar: quem entra depois desligou quem estava correndo.
///
/// Exigir os dois lados é deliberado. Cancelar de um lado só não serializa nada, e é
/// o que separa esta regra de uma que cala aviso legítimo. Ver ADR 0009: sem ela, o
/// padrão de dono centralizado — a própria remediação que o `SAR-OWN-001` sugere —
/// era invisível sempre que o cancelamento passava por um método auxiliar.
fn serialized_by_cancel(
    first: &OwnershipClaim,
    second: &OwnershipClaim,
    sources: &[ParsedSource],
    helpers: &BTreeMap<String, BTreeMap<String, BTreeSet<String>>>,
    kill: &Regex,
) -> bool {
    if first.span.path != second.span.path {
        return false;
    }
    let Some(source) = sources.iter().find(|item| item.path == first.span.path) else {
        return false;
    };
    let Some(helpers) = helpers.get(&first.span.path) else {
        return false;
    };
    let by_first = cancelled_before(first, source, helpers, kill);
    if by_first.is_empty() {
        return false;
    }
    !by_first.is_disjoint(&cancelled_before(second, source, helpers, kill))
}

fn has_ordering_barrier(
    first: &OwnershipClaim,
    second: &OwnershipClaim,
    sources: &[ParsedSource],
) -> bool {
    let Some(source) = sources.iter().find(|item| item.path == first.span.path) else {
        return false;
    };
    let variable = first
        .controller
        .split("::")
        .last()
        .unwrap_or_default()
        .split('@')
        .next()
        .unwrap_or_default();
    let start = first.span.line.saturating_sub(1) as usize;
    let end = second.span.line as usize;
    let between = source
        .source
        .lines()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect::<Vec<_>>()
        .join("\n");
    between.contains(&format!("await {variable}.finished"))
        || between.contains(&format!("{variable}.kill()"))
}

fn input_claims(
    source: &ParsedSource,
    declared_actions: &BTreeSet<String>,
    output: &mut AdapterOutput,
) {
    let action_regex = Regex::new(
        r#"(?:is_action|is_action_pressed|is_action_released)\s*\(\s*&?["']([^"']+)["']"#,
    )
    .unwrap();
    for function in source.functions.iter().filter(|item| {
        matches!(
            item.name.as_str(),
            "_input" | "_unhandled_input" | "_gui_input"
        )
    }) {
        for branch in
            common::action_branches(function, &source.calls, &action_regex, GDSCRIPT_BLOCKS)
        {
            for action in branch.actions {
                for (handler, span) in &branch.handlers {
                    let declared = declared_actions.contains(&action);
                    let claim = OwnershipClaim {
                        resource: ResourceKey {
                            engine: Engine::Godot,
                            kind: ResourceKind::InputEffect,
                            scope: source.path.clone(),
                            target: handler.clone(),
                            property: action.clone(),
                            profile: None,
                        },
                        owner: format!("{}::{}[{action}]", source.path, function.name),
                        span: span.clone(),
                        confidence: if declared {
                            Confidence::Proven
                        } else {
                            Confidence::Ambiguous
                        },
                        operation: function.name.clone(),
                        controller: handler.clone(),
                        flow: String::new(),
                    };
                    if !declared {
                        output.diagnostics.push(Diagnostic {
                            rule: "SAR-PARSE-001".to_owned(),
                            severity: Severity::Warning,
                            resource: claim.resource.id(),
                            primary: span.clone(),
                            related: Vec::new(),
                            owners: vec![claim.owner.clone()],
                            explanation: format!(
                                "a ação `{action}` é consultada no código, mas não está definida na seção [input] de project.godot"
                            ),
                            remediation: "declare a ação em project.godot ou remova a consulta obsoleta"
                                .to_owned(),
                        });
                    }
                    output.claims.push(claim);
                }
            }
        }
    }
}

/// Declarações de canal físico: o ramo testa a classe do evento e o corpo chama o
/// efeito. Não exige mapa de ações, que é o que faltava para enxergar jogo de toque.
fn physical_channel_claims(source: &ParsedSource, output: &mut AdapterOutput) {
    let event_regex = Regex::new(r"\bis\s+(InputEvent[A-Za-z0-9_]*)").unwrap();
    for function in source.functions.iter().filter(|item| {
        matches!(
            item.name.as_str(),
            "_input" | "_unhandled_input" | "_gui_input"
        )
    }) {
        for branch in
            common::action_branches(function, &source.calls, &event_regex, GDSCRIPT_BLOCKS)
        {
            for event in branch.actions {
                if channel_of(&event) == Channel::Other {
                    continue;
                }
                for (handler, span) in &branch.handlers {
                    output.claims.push(OwnershipClaim {
                        resource: ResourceKey {
                            engine: Engine::Godot,
                            kind: ResourceKind::InputEffect,
                            scope: source.path.clone(),
                            target: handler.clone(),
                            property: PHYSICAL_CHANNEL.to_owned(),
                            profile: Some(Profile::Android),
                        },
                        owner: format!("{}::{}[{event}]", source.path, function.name),
                        span: span.clone(),
                        confidence: Confidence::Proven,
                        operation: event.clone(),
                        controller: handler.clone(),
                        flow: String::new(),
                    });
                }
            }
        }
    }
}

/// Dois canais físicos distintos chegando ao mesmo efeito. É a definição que o
/// `ROTEIRO.md` dá do que o Sara bloqueia, e o adapter Godot não a aplicava quando o
/// projeto não declarava ações. ADR 0010.
fn diagnose_physical_channels(
    all_claims: &[OwnershipClaim],
    emulates: bool,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !emulates {
        return;
    }
    let mut groups: BTreeMap<ResourceKey, Vec<&OwnershipClaim>> = BTreeMap::new();
    for claim in all_claims
        .iter()
        .filter(|item| item.resource.property == PHYSICAL_CHANNEL)
    {
        groups
            .entry(claim.resource.clone())
            .or_default()
            .push(claim);
    }
    for claims in groups.values_mut() {
        claims.sort_by(|left, right| left.owner.cmp(&right.owner));
        for pair in claims.windows(2) {
            let (first, second) = (pair[0], pair[1]);
            if !physical_duplicate(channel_of(&first.operation), channel_of(&second.operation)) {
                continue;
            }
            diagnostics.push(common::conflict_diagnostic(
                "SAR-OWN-002",
                Severity::Error,
                first,
                second,
                "toque e mouse chegam ao mesmo efeito, e no aparelho um toque entrega os dois eventos porque o projeto não desliga emulate_mouse_from_touch",
                "consuma o evento com set_input_as_handled, desligue emulate_mouse_from_touch em project.godot, ou trate um canal só",
            ));
        }
    }
}

fn diagnose_inputs(
    all_claims: &[OwnershipClaim],
    sources: &[ParsedSource],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut groups: BTreeMap<ResourceKey, Vec<&OwnershipClaim>> = BTreeMap::new();
    for claim in all_claims.iter().filter(|item| {
        item.resource.engine == Engine::Godot
            && item.resource.kind == ResourceKind::InputEffect
            && item.confidence == Confidence::Proven
            && item.resource.property != PHYSICAL_CHANNEL
    }) {
        groups
            .entry(claim.resource.clone())
            .or_default()
            .push(claim);
    }
    for claims in groups.values_mut() {
        claims.sort_by(|left, right| left.owner.cmp(&right.owner));
        for pair in claims.windows(2) {
            let first = pair[0];
            let second = pair[1];
            if first.operation == second.operation {
                continue;
            }
            let operations = BTreeMap::from([
                (first.operation.as_str(), first),
                (second.operation.as_str(), second),
            ]);
            if let (Some(input), Some(unhandled)) =
                (operations.get("_input"), operations.get("_unhandled_input"))
            {
                if input_marks_handled(input, sources) {
                    continue;
                }
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-002",
                    Severity::Error,
                    input,
                    unhandled,
                    "_input e _unhandled_input encaminham a mesma ação ao mesmo efeito sem marcar o evento como tratado",
                    "escolha um único proprietário ou marque o evento como tratado antes que ele alcance o segundo callback",
                ));
            } else {
                diagnostics.push(common::conflict_diagnostic(
                    "SAR-OWN-002",
                    Severity::Warning,
                    first,
                    second,
                    "dois callbacks de entrada encaminham a mesma ação ao mesmo efeito, mas a propagação depende da árvore de cena",
                    "centralize a entrada ou prove e documente onde o evento é consumido",
                ));
            }
        }
    }
}

fn input_marks_handled(claim: &OwnershipClaim, sources: &[ParsedSource]) -> bool {
    let Some(source) = sources.iter().find(|item| item.path == claim.span.path) else {
        return false;
    };
    source
        .functions
        .iter()
        .find(|function| function.name == "_input")
        .is_some_and(|function| function.text.contains("set_input_as_handled"))
}

fn declared_actions(project: &Path) -> Result<BTreeSet<String>> {
    let path = project.join("project.godot");
    let source = fs::read_to_string(&path)
        .with_context(|| format!("não consegui ler {}", path.display()))?;
    let section = source
        .split("\n[input]\n")
        .nth(1)
        .unwrap_or_default()
        .split("\n[")
        .next()
        .unwrap_or_default();
    let expression = Regex::new(r"(?m)^([^=\r\n]+?)\s*=\s*\{").unwrap();
    Ok(expression
        .captures_iter(section)
        .map(|captures| captures[1].trim().trim_matches('"').to_owned())
        .collect())
}
