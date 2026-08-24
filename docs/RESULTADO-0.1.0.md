# Resultado do lançamento interno 0.1.0

Data da medição: 2026-08-23.

## Decisão

Os Marcos 0 a 5 foram implementados e o Sara pode entrar no Marco 6, de uso
pessoal. O Portão 0 foi aprovado: as duas regressões históricas vermelhas falham, as
versões corrigidas passam e o BomberBoom atual não recebe erro bloqueante.

Isto autoriza uso interno, não publicação. Produto pago, código aberto, licença,
marca, SDK, runtime e engine própria continuam fora do escopo.

## Fitness functions medidas

| Medida | Resultado | Critério |
|---|---:|---:|
| testes unitários | 5 aprovados | todos aprovados |
| cenários CLI e fixtures | 16 aprovados | todos aprovados |
| fitness functions de governança | 13 aprovadas | toda ADR com conformidade executável |
| projetos reais analisados | 5 de 5 | 5 de 5 |
| arquivos do corpus aceitos ou recusados explicitamente | 694 de 694 | sem omissão silenciosa |
| erros bloqueantes no corpus | 0 | zero falso positivo bloqueante |
| mediana no Gods, cinco varreduras | 0,69 s | menor que 2 s |
| variação nas cinco varreduras | 0,64–0,74 s | registrar estabilidade |
| JSON repetido | idêntico byte a byte | determinístico |
| execução sem rede | código 0 em namespace sem rede | offline |
| binário distribuível | 3.892.496 bytes | Linux x86_64, sem Python/Node |
| portão em projeto ativo | porte BomberBoom | `AGENTS.md` e `CLAUDE.md` ativos |
| portões copiáveis | Godot e Defold | entrada ausente reprova explicitamente |

O teste offline executou o binário de release contra o porte Godot dentro de um
namespace Linux sem rede. As dependências dinâmicas restantes são apenas bibliotecas
de sistema do Linux; não há runtime de linguagem externo.

*(Atualizado em 23/08/2026: o artefato em `dist/` foi reconstruído três vezes no mesmo
dia — pela fronteira da Fase 1, pelo fechamento do A7 e pelas regras das ADRs 0009 e 0010 —,
e são 55.512 bytes a mais que o binário original do lançamento. O comportamento foi verificado
a cada vez rodando o binário anterior e o novo contra os cinco projetos do corpus: as
declarações são idênticas em todos, e a única mudança de diagnóstico em toda a sequência
é o aviso falso do `gods` desaparecendo. O `strip` que produz este tamanho vive em
`[profile.release]` do `Cargo.toml` e em `tools/dist.sh`, em vez de depender de um passo
manual lembrado.)*

O porte Godot recebeu `.sara/`, `sara.toml` e o portão curto tanto em `AGENTS.md`
quanto em `CLAUDE.md`. A inicialização preservou as instruções e mudanças que já
existiam no projeto. O kit também instala padrões copiáveis para RNG injetado,
normalização de entrada e log estruturado, sem criar uma dependência de runtime.

## Corpus

| Projeto | Engine | Arquivos | Declarações | Erros | Avisos |
|---|---|---:|---:|---:|---:|
| BomberBoom | Defold | 27 | 76 | 0 | 12 |
| porte BomberBoom | Godot | 69 | 5 | 0 | 0 |
| Boomlitude | Godot | 97 | 4 | 0 | 0 |
| Mineboom | Godot | 51 | 0 | 0 | 0 |
| Gods | Godot | 450 | 53 | 0 | 0 |

O Marco 6 classificou a baseline Defold: sete dos 19 avisos iniciais eram ruído
demonstrável e viraram fixtures de regressão. Os 12 restantes são úteis: onze mostram
substituições de animação deliberadas, mas sem cancelamento explícito, e um mantém
visível o alvo dinâmico `PAVIO[n]`. A fotografia atual tem zero aviso falso entre 76
declarações; o critério precisa continuar sendo medido nas próximas mudanças.

## Compatibilidade e trade-offs

- O núcleo comum concentra descoberta, configuração, modelo, ordenação e apresentação.
  A variação por engine existe e está contida em cinco arquivos nomeados — `config.rs`,
  `init.rs`, `model.rs`, `parser.rs` e `scanner.rs` — mais os dois adapters. O código
  compartilhado pelos adapters não conhece engine nenhuma. O conjunto é fechado por
  `tests/governanca.rs`, que reprova quando cresce em silêncio. Isso preserva a troca de
  foco sem reescrever o CLI. *(Corrigido em 23/08/2026: a redação anterior afirmava que
  o núcleo mantinha tudo fora dos adapters, o que o código não sustentava. Achado A1 de
  [`AUDITORIA-ARQUITETURAL.md`](AUDITORIA-ARQUITETURAL.md).)*
- Tree-sitter está fixado pelo manifesto e lockfile. Erro sintático relevante encerra
  com código 2, sem resultado parcial apresentado como sucesso.
- O parser comunitário de GDScript recusou inicialmente identificadores Unicode
  válidos encontrados no Gods. O adapter agora cria uma forma ASCII de mesmo tamanho
  apenas para parsing e usa o texto original nas localizações e diagnósticos.
- A análise privilegia precisão: concorrência no mesmo fluxo pode ser comprovada;
  relações entre fluxos ou alvos dinâmicos permanecem avisos.
- Consultas de entrada Godot só entram em conflito comprovado quando a ação existe em
  `project.godot`; ação ausente produz `SAR-PARSE-001` visível e não um bloqueio inventado.
- Godot continua como hipótese de foco, pois concentra quatro projetos e fechou o
  corpus sem aviso. A pontuação final continua bloqueada pela prova em aparelho Android,
  que tem poder de veto no plano.

## Riscos e próximo portão

O risco dominante deixou de ser incompatibilidade do parser e passou a ser utilidade
cotidiana: regras conservadoras podem avisar demais no Defold ou cobrir pouco código
dinâmico. O Marco 6 exige dez mudanças relevantes em pelo menos dois projetos, com o
registro em [`USO-PESSOAL.md`](USO-PESSOAL.md). A medição atual está em uma de dez
mudanças reais e dois de dois projetos integrados; baselines e instalação não contam
como mudanças de uso.

Se houver falso bloqueio, necessidade recorrente de inspeção humana ou ausência de
uso voluntário, o encerramento legítimo é manter apenas o kit AI-first ou congelar a
ferramenta privada. O spike visual Godot foi autorizado pelo ADR 0004, mas continua
condicionado à conclusão do Marco 6 e não autoriza SDK ou runtime.
