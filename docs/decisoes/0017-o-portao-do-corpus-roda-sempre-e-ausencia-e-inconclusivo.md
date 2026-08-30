# ADR 0017 - O portão do corpus roda na suíte padrão, e ausência de corpus é inconclusivo

**Status:** Aceita
**Data:** 29 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** o portão executável do confronto que a [ADR 0012](0012-sara-e-corpus-coevoluem.md) §3
exige. **Não altera o verificador**: nenhuma regra, nenhum adapter e nenhum código de
diagnóstico muda por causa desta ADR.

## Contexto

A ADR 0012 §3 obriga a confrontar toda capacidade generalizável com os cinco projetos
do corpus antes de incorporá-la, e nomeia `tools/check_corpus.sh` como o mecanismo. O
mecanismo existia e não mecanizava nada.

`tests/corpus.rs` era o **único `#[ignore]` desta árvore**. Ele só rodava por invocação
manual, e dependia de cinco caminhos absolutos escritos como literal. Em 28 de agosto de
2026 descobriu-se que os cinco tinham migrado de `~/Godot` para `~/godot` e que o porte
mudara de nome. O próprio arquivo registrou o efeito, em comentário:

> O teste é `#[ignore]`, então a defasagem ficou invisível — e com ela o confronto com o
> corpus que a ADR 0012 §3 exige.

A forma do defeito é conhecida neste repositório e tem nome: **verde por construção**. É
a mesma da [ADR 0016](0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md),
onde quatro rodadas de auditoria aprovaram uma árvore que não continha o acervo inteiro.
Aqui a pergunta não foi feita a um recorte errado: **ela não foi feita**, e nada disse
isso em voz alta.

O pedido de conserto chegou de fora deste repositório. A razão dele, porém, é um fato
medido **aqui**, e é por isso que esta decisão é deste lado da fronteira: pela ADR 0016
este repositório tem série de decisões própria e não importa a de outro produto.

### Por que consertar vem antes de agendar

A saída óbvia seria agendar a execução do portão — um lembrete, uma cadência, uma data.
**Agendar execução de um teste que só roda quando alguém lembra de invocá-lo à mão
reproduz o defeito com calendário em cima**: o calendário passa a ser a única coisa que
avisa, e ele avisa sobre a data, não sobre o corpus. Um portão que já defasou sem ninguém
ver defasa de novo entre duas execuções agendadas.

### O nó, que é do arnês e não do teste

Ausência de corpus não pode **reprovar**. A maioria das máquinas não tem os cinco
projetos, e falhar por ausência treina quem roda a ignorar o vermelho — que é a única
coisa pior que não ter portão.

E não pode **aprovar**. Corpus ausente não é corpus limpo: **não poder conferir não é ter
conferido**.

O arnês do Cargo tem dois estados. O confronto precisa de três.

## Opções consideradas

1. **Agendar a execução do portão como ele estava.** Recusada pelo argumento acima: é o
   defeito com calendário em cima.
2. **Manter o `#[ignore]` e confiar no hábito.** Recusada: é exatamente o que já falhou, e
   o custo dele já está medido em dias de invisibilidade.
3. **Tirar o `#[ignore]` mantendo o `assert!(path.is_dir())`.** Recusada: transforma
   ausência em reprovação, e uma suíte que fica vermelha por um motivo que não é defeito
   ensina a passar os olhos pelo vermelho.
4. **Remover o teste e o script.** Recusada: a ADR 0012 §3 continua exigindo o confronto,
   e retirar o instrumento seria decidir por omissão o que ela decidiu por escrito.
5. **Tirar o `#[ignore]`, parametrizar os cinco caminhos e dar um terceiro estado à
   ausência.** Adotada.

## Decisão

Nós adotamos a opção 5.

### 1. O portão roda na suíte padrão

`five_personal_projects_have_no_blocking_false_positive` deixa de ser `#[ignore]`.
`cargo test` sozinho o executa. Portão que espera ser lembrado envelhece junto com o que
ele mediria.

### 2. Os cinco caminhos vêm do ambiente

Os valores da máquina do proprietário passam a ser **padrão documentado**, e não fonte:

| Variável | Padrão |
|---|---|
| `SARA_CORPUS_RAIZ` | `/home/mateus` |
| `SARA_CORPUS_BOMBERBOOM_DF` | `$SARA_CORPUS_RAIZ/defold/bomberboom-df` |
| `SARA_CORPUS_BOMBERBOOM_GD` | `$SARA_CORPUS_RAIZ/godot/bomberboom-gd` |
| `SARA_CORPUS_BOOMLITUDE` | `$SARA_CORPUS_RAIZ/godot/boomlitude` |
| `SARA_CORPUS_MINEBOOM` | `$SARA_CORPUS_RAIZ/godot/mineboom` |
| `SARA_CORPUS_GODS` | `$SARA_CORPUS_RAIZ/godot/gods` |

A saída nomeia **o caminho e a origem dele** — variável individual, raiz ou padrão.
Dizer "faltou" sem dizer de onde veio o caminho manda procurar no lugar errado, e foi
assim que a defasagem de 28/08 sobreviveu.

### 3. Três estados, três códigos de saída

`tools/check_corpus.sh` continua sendo o portão, agora com os mesmos três códigos do
contrato do `sara` ([ADR 0006](0006-contrato-estrito-de-relatorio-e-codigos-de-saida.md)),
pelo mesmo motivo:

| Código | Estado | Significa |
|---|---|---|
| 0 | aprovado | os cinco foram lidos e nenhum tem conflito bloqueante |
| 1 | reprovado | algum projeto estava lá e o confronto encontrou erro nele |
| 2 | inconclusivo | o corpus não estava no lugar declarado — não deu para conferir |

Como o arnês do Cargo não tem o terceiro estado, o inconclusivo sai por três canais que
ele não apaga: um bloco escrito no descritor real do processo — visível num `cargo test`
comum, apesar da captura do libtest —, o veredito em `$SARA_CORPUS_VEREDITO`, e o
código 2 do script.

### 4. Reprovado ganha de inconclusivo

Se um projeto sumiu e outro mostrou conflito bloqueante, o resultado é **reprovado**, com
os ausentes listados junto. Conflito comprovado é fato; ausência é a falta de um, e a
falta de um fato não apaga o outro.

### 5. Verde na suíte padrão não é aprovação do corpus

Esta é a consequência que precisa estar escrita, porque ela troca o significado de uma
coisa que as pessoas já leem: **`cargo test` verde não certifica o confronto da ADR 0012
§3.** Quem certifica é a saída 0 de `tools/check_corpus.sh`, e ela só acontece quando os
cinco projetos foram lidos.

### 6. O preço, medido antes de aceito

Rodar o corpus inteiro em toda suíte custava **40 s**, porque as gramáticas tree-sitter
iam sem otimização. Portão caro é portão que alguém desliga, e desligar é o defeito que
esta ADR conserta. `[profile.dev.package."*"] opt-level = 2` otimiza **só as
dependências** — o código do Sara continua sem otimização, para o pânico e o depurador
continuarem legíveis — e devolve a suíte a **10 s**, sem tocar no que ela mede.

## Consequências

### Positivas

- A defasagem de 28/08 teria sido visível **no dia em que aconteceu**, e nomeando qual
  projeto: é o critério que compra o conserto.
- O corpus deixa de ser um caminho na máquina de uma pessoa e passa a ser configuração.
  Outra máquina, outro clone e um subconjunto do corpus passam a ser exprimíveis.
- Indisponibilidade de corpus vira **registro**, não silêncio: quem roda vê o bloco, e
  quem automatiza lê o código 2.
- O portão foi exercitado nos quatro modos antes de entrar — aprovado, reprovado e as
  duas formas de inconclusivo. Portão que nunca reprovou não foi mostrado capaz de
  reprovar.

### Negativas

- **A suíte padrão ficou dez vezes mais lenta**, de ~1 s para ~10 s. É o preço de o
  portão existir de verdade, e ele fica registrado em vez de descoberto depois.
- **`cargo test` verde passa a significar menos do que significava.** Antes, o corpus
  simplesmente não estava na suíte; agora ele está e pode ter sido pulado. Sem o item 5
  escrito, isso seria uma armadilha nova no lugar da antiga.
- O terceiro estado mora fora do arnês, em script e em canal de saída próprio. É mais
  peça do que um teste comum tem, e a razão é do arnês, não do problema.
- A parametrização é uma porta: um `SARA_CORPUS_RAIZ` apontado para um corpus falso
  produz aprovado com facilidade. A ADR 0012 §3 continua exigindo que o **diff de
  diagnóstico** seja lido por uma pessoa, e nenhum teste prova leitura.

## Conformidade

Duas fitness functions automáticas em `tests/governanca.rs`:

- `adr_0017_nenhum_teste_desta_arvore_espera_ser_lembrado` varre `tests/` e `src/` e
  reprova quando um atributo `#[ignore]` reaparece. Se algum dia um teste precisar mesmo
  dele, isso é decisão arquitetural e vem com ADR, como veio esta.
- `adr_0017_o_portao_do_corpus_tem_tres_estados` reprova quando `tools/check_corpus.sh`
  volta a depender da flag `--ignored` — que agora faria o portão rodar zero teste e sair
  0, aprovação por vacuidade —, quando algum dos três estados some do script, ou quando
  `tests/corpus.rs` deixa de declarar alguma das seis variáveis.

O resultado de cada execução do portão é registrado em
[`docs/DOSSIE-DO-PORTAO-DO-CORPUS.md`](../DOSSIE-DO-PORTAO-DO-CORPUS.md), com quatro
campos — **comando, revisão, máquina e resultado**. Número copiado de documento não é
evidência; sem os quatro campos, uma execução relatada vale como inconclusiva.

Continua **manual**, como a ADR 0012 já declarava: ler o diff de diagnóstico. Nenhum
teste prova que uma comparação foi lida.

## Critério de revisão

- Se a suíte padrão ficar cara a ponto de alguém filtrar o teste do corpus na rotina, o
  desenho falhou e a revisão é do item 1, não do orçamento de tempo.
- Se aparecer uma execução registrada como aprovada sem os quatro campos, o dossiê virou
  papelada e esta decisão perdeu a parte que ela tinha de verificável.
- Se o inconclusivo passar a ser o resultado normal por meses, a pergunta deixa de ser
  sobre o portão e passa a ser sobre o corpus: cinco projetos que ninguém consegue
  apontar não são mais corpus.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma. Refina o mecanismo de conformidade da
  [ADR 0012](0012-sara-e-corpus-coevoluem.md) §3, que continua valendo integralmente.
- Última alteração: 29 de agosto de 2026
