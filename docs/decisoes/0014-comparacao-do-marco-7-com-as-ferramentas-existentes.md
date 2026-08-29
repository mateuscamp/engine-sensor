# ADR 0014 - A comparação do Marco 7: cancelar o spike e preservar a pergunta 7

**Status:** Aceita
**Data:** 28 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** pré-condição exigida pela [ADR 0011](0011-marco-7-exige-comparacao-com-ferramenta-existente.md);
conclui sobre o spike autorizado pela [ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md)

## Contexto

A [ADR 0011](0011-marco-7-exige-comparacao-com-ferramenta-existente.md) proibiu o Marco 7
de começar sem uma ADR que confrontasse o spike da ADR 0004 contra as ferramentas
existentes, item a item contra as sete fitness functions daquela ADR. Esta é essa ADR.

Ela é escrita agora porque a pré-condição de tempo caiu: o
[Marco 6 encerrou em 28/08/2026](0013-manter-a-sara-privada-ao-fim-do-marco-6.md), por
conclusão.

### O que existe, com data, versão e maturidade observadas em 28/08/2026

| ferramenta | maturidade | o que entrega | o que exige do jogo |
|---|---|---|---|
| [`aigengame/godot-agent`](https://github.com/aigengame/godot-agent) (`gda`) | **v0.12.0, 27/08/2026**; 459 commits; CI e ADRs numeradas | dois modos: *headless* sem instalar nada, e *live* por daemon — árvore de cena viva, ler e escrever propriedade, simular entrada, capturar tela, amostrar desempenho | modo live: addon + autoload; `game call` exige o jogo declarar `GDA_CALLABLE` no próprio script |
| [`satelliteoflove/godot-mcp`](https://github.com/satelliteoflove/godot-mcp) | 309 commits, 148 estrelas, ativo | estado vivo em JSON, **playtesting determinístico com relógio congelado e passo de quadro**, injeção de entrada, cenário por GDScript, captura, perfil por quadro | addon + autoload + Node.js 20+ + Godot 4.5+, WebSocket pelo protocolo de depuração |
| [`defold/extension-automation-bridge`](https://github.com/defold/extension-automation-bridge) | oficial da Defold; 92 commits, 32 estrelas | consultas de cena e elemento, entrada FIFO com recibo, capturas, vídeo nativo, canal Lua para eventos e anotação semântica | extensão nativa; a ponte HTTP só existe em build de depuração |

A ponte da Defold não é candidata do spike, que é só Godot. Ela entra como evidência de
direção: **quem faz as engines está fechando esta lacuna**, e a ADR 0011 já registrou o
que isso significa.

### A evidência que decide esta ADR não é de leitura: é medida, neste projeto

Em 26 e 27 de agosto de 2026 o `gda 0.11.0` foi instalado no porte do BomberBoom e
**medido contra as sondas próprias do projeto**, em três investigações reais. O registro
está em `docs/COMPARACAO.md` daquele repositório, na seção *"O `godot-agent` (gda) medido
contra as sondas"*. É evidência de primeira mão, com comando e número, e vale mais que
qualquer leitura de README.

| investigação | `gda` | sonda do projeto | quem acertou |
|---|---|---|---|
| (a) `Container` zera o `scale` da barra de fúria | ~2,0 s de comandos | 45 linhas, 0,74 s | **só a sonda** |
| (b) o HUD só acordava com evento | 4,5–6,2 s por ciclo | 48 linhas, 3,7 s | os dois |
| (c) célula com gema e sem nó | **não fecha** | 62 s | **só a sonda** |

**O achado (a) é o que governa esta decisão, e ele não é sobre custo.** O `gda` leu a
barra com `perf monitor --property … --frames 30` e respondeu que a escala **sobrevive**
ao relayout. O defeito estava lá. A causa: em Godot 4.7, `Control.scale` e
`offset_transform_scale` são propriedades diferentes, e **só a segunda aparece em
`get_property_list()`**. O jogo escreve `scale`; o `gda` recusa `scale` com
`live_unknown_property` e lê `offset_transform_scale`, que fica em `(1,1)` estourando ou
não. O valor que ele devolveu era o que ele mesmo havia escrito.

Medido depois: a família `offset_transform_*` **não desenha**. Ou seja, ele não leu a
propriedade errada — leu uma propriedade **ligada em nada**, e reportou verde, em JSON
estruturado, com 30 quadros amostrados. Descobrir que estava errado custou dez rodadas de
comando e três sondas.

**E isto é exatamente o defeito do [caso da aranha](../CASO-DA-ARANHA.md), do outro lado
do balcão.** Lá, todo instrumento da agente rodava com `pavio = 999.0` para isolar as
fases, e era esse botão girado que escondia a falha; a suíte deu 303 casos verdes sobre um
mecanismo que nunca funcionou. Aqui, o instrumento lê a propriedade que o motor deixa ler,
e é essa leitura que esconde a falha. **É a mesma classe: instrumento cego medindo a
própria cegueira, e reportando verde.** Uma ferramenta que responde errado sem avisar é
pior que ferramenta ausente, porque a ausência não é acreditada.

### Um achado que a própria ADR 0011 previu, e que se realizou em dois dias

A ADR 0011 registrou como consequência negativa que *"a comparação envelhece"*. Envelheceu
**em dois dias**. A medição de 26/08 concluiu, com razão para a versão medida, que *"o gda
não chama método — toda sonda deste repo começa em `tabuleiro.call(…)`; não há verbo para
isso"*. A **v0.12.0, de 27/08**, declara uma superfície `game call` somente-leitura, e a
mesma versão traz recibo de captura ligando sessão, cena, estado e hash da saída, e captura
de tela sincronizada por predicado com eventos de entrada atômicos.

Isso é a unidade de evidência da ADR 0004 — `imagem + estado semântico + sequência de
entradas + instante + logs` — **com recibo, entregue por terceiros, na mesma semana em que
este marco fechou.** A limitação estrutural que a medição registrou caiu antes de a ADR de
comparação ser escrita.

O ecossistema se move mais rápido que o ciclo de decisão deste projeto. Isso é um dado
sobre a decisão, e não um detalhe dela.

## As sete fitness functions da ADR 0004, item a item

| # | fitness function da ADR 0004 | as ferramentas existentes | evidência |
|---:|---|---|---|
| 1 | um agente inicia a prova com um comando e conclui sem uma pessoa olhar a cena | **satisfaz** | modo headless do `gda`, sem editor e sem instalar nada; instalação do modo live é reversível (`git diff` vazio após `daemon uninstall`, conferido) |
| 2 | três regressões injetadas detectadas, e as versões corrigidas passam | **não se aplica** | isto é o experimento, não a ferramenta. Elas entregam o material para fazê-lo |
| 3 | o diagnóstico combinado aponta o nó ou propriedade causal | **NÃO satisfaz, e está medido** | na investigação (a) o `gda` apontou que não havia defeito. Localizar causa é precisamente onde ele falhou, e falhou em silêncio |
| 4 | dez repetições produzem estado e pixels idênticos, ou a variação é identificada | **satisfaz** | o `godot-mcp` entrega relógio congelado e passo de quadro explicitamente |
| 5 | manifesto, imagens, estado, entradas e logs reproduzem a execução sem arquivo oculto | **satisfaz, e além** | `gda` v0.12.0: recibo de captura ligando sessão, cena, estado e hash da saída |
| 6 | execução mediana abaixo de 30 s no cenário mínimo | **satisfaz, com ressalva medida** | 190 ms por chamada (10 leituras, 179–208 ms). A ressalva: 190 ms são ~11 quadros a 60 fps, e em fenômeno transiente **a latência é da ordem do fenômeno** — ele vê o estado parado, não a descida |
| 7 | imagem mais estado diagnostica melhor que imagem isolada, em ao menos um caso definido | **ninguém responde** | é a única das sete que produz conhecimento em vez de infraestrutura, e continua sem resposta pública |

**Seis das sete são infraestrutura, e cinco delas já estão prontas.** A que falha, a 3, não
falha por imaturidade: falha porque uma ferramenta genérica lê a lista de propriedades que
o motor publica, e essa lista mentiu. A sétima é a pergunta.

## Opções consideradas

São as três que a ADR 0011 mandou escolher entre.

1. **Construir** o spike como a ADR 0004 o escreveu.
2. **Adotar** uma ferramenta existente e medir com ela.
3. **Cancelar** o spike.

### Por que não construir

Construir gastaria as horas do Marco 7 em cinco fitness functions que já existem prontas,
mantidas por quem tem mais braço, com recibo e hash que a ADR 0004 nem pediu. Sobrariam as
duas que interessam — a 3 e a 7 —, e a 3 **não se resolve construindo**: ela falha pela
mesma razão em qualquer instrumento genérico que leia a lista de propriedades do motor.

E há o argumento do [caso da aranha](../CASO-DA-ARANHA.md), que vale contra construir mais
que qualquer comparação de ferramenta: **as sete fitness functions da ADR 0004 têm todas
forma de regressão** — algo que estava certo e ficou errado. A aranha nunca esteve certa, e
o spike, exatamente como especificado, teria passado nela. Construir o experimento como
está escrito custaria as horas e responderia a pergunta errada.

### Por que não adotar

O acoplamento é o problema, e ele é escrito, não estimado:

- **modo live do `gda`:** addon `res://addons/gda_harness/` (1.754 linhas) mais seção
  `[autoload]` no `project.godot`. O `CLAUDE.md` do porte proíbe os dois por escrito. A
  instalação é reversível e o crédito é real — mas reversível não é ausente;
- **`godot-mcp`:** addon, autoload, **Node.js 20+** e WebSocket em processo de fundo;
- **`game call`:** exige o jogo declarar `GDA_CALLABLE` no próprio script. É intrusão
  menor que um autoload e continua sendo modificação do jogo para servir ao instrumento;
- **plataforma:** o modo live do `gda` é macOS e Linux. **Nenhuma delas vai ao Android**, e
  a medição em aparelho é o veto que o `RESULTADO-0.1.0.md` mantém sobre toda conclusão de
  foco. A ferramenta não alcança o lugar onde a evidência que falta seria colhida.

Contra a [ADR 0007](0007-observe-como-binario-separado.md): ela separou `sara-observe` em
binário próprio justamente para que o quantum offline do `sara` não herdasse dependência de
ambiente. Adotar não fere a ADR 0007 — o acoplamento ficaria do lado certo da fronteira —,
mas **esvazia o motivo dela**: não haveria segundo binário nenhum, e sim um daemon de
terceiros.

E adotar carrega o achado (a) para dentro do portão. Uma ferramenta que responde errado sem
avisar, no caso mais difícil dos três, não pode ser a base de um portão que substitui o olho
humano. Esse é o modo de falha mais caro que existe para este projeto — é o que o
`RESULTADOS.md` chama de *"rodou, logo funcionou"*, com JSON bonito por cima.

### Por que cancelar, e o que cancelar não pode levar junto

Cancelar é a única das três que não gasta hora em infraestrutura pronta nem importa
cegueira medida. O risco dela é jogar fora a pergunta 7 junto com o spike — e a ADR 0011 já
tinha nomeado exatamente esse risco ao recusar a opção "cancelar o Marco 7".

## Decisão

Nós **cancelamos o spike da ADR 0004 e o Marco 7 como estavam especificados**, e
**preservamos a pergunta 7** como experimento próprio, muito menor.

### 1. O Marco 7 não acontece

O segundo binário `sara-observe` não nasce. As fitness functions 1, 2, 4, 5 e 6 da ADR 0004
deixam de ser trabalho deste projeto: elas estão entregues por terceiros, e refazê-las seria
construir o que já existe — que é literalmente o erro que a ADR 0011 existe para impedir.

A [ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md) passa a **Substituída por 0014**.
A [ADR 0011](0011-marco-7-exige-comparacao-com-ferramenta-existente.md) cumpriu o papel e se
encerra junto: o freio funcionou, e funcionou barato — custou uma ADR e nenhuma hora de
implementação.

### 2. A pergunta 7 sobrevive, e sozinha ela não precisa de binário

*Imagem mais estado produz diagnóstico mais preciso que imagem isolada?* Continua sem
resposta pública, continua sendo a única das sete que produz conhecimento, e **não exige
runtime novo**: exige um caso definido antes, duas evidências e uma comparação.

O projeto já tem os dois lados montados no porte — a Sentinela dá imagem com referência, e
as sondas dão estado no mesmo processo. Responder a pergunta é escolher um caso e medir, não
construir uma ferramenta.

Isso **não** é autorização de escopo: nada aqui autoriza binário, daemon, addon, autoload,
rede ou dependência. Se responder a pergunta exigir qualquer um desses, ela não é respondida
e o resultado é esse.

### 3. A pergunta 8 fica registrada como pré-condição da 7

O caso da aranha mostrou que uma prova de forma regressiva passa numa peça que nunca esteve
certa. Antes de a pergunta 7 valer alguma coisa, é preciso saber contra o que a evidência
combinada é conferida — e a resposta hoje é *"contra uma referência que só existe se a peça
já esteve certa"*. Isso é a **verdade de design declarada**, que a
[ADR 0012](0012-sara-e-corpus-coevoluem.md) nomeou e deixou sem formato, lugar e dono.

Esta ADR não a decide. Registra que ela é pré-condição da pergunta 7, e não o contrário.

Quem a decide é a [ADR 0015](0015-a-verdade-de-design-sao-tres-campos-no-carimbo.md), do
mesmo dia, que fixa formato, lugar e dono e escreve a **redação exata da pergunta 8**. Ela
mantém esta decisão inteira: a pergunta 8 não vira oitava fitness function da ADR 0004 —
acrescentar critério a um experimento cancelado seria escrever critério que ninguém roda, e o
texto de 23/08 fica intacto como esta ADR prometeu.

### 4. O que fica escrito sobre as ferramentas, para não ser redescoberto

- **Elas resolvem infraestrutura e não resolvem cegueira.** A lista de propriedades que o
  motor publica pode não conter a propriedade que o jogo anima, e nesse caso a ferramenta
  genérica responde verde. Medido em Godot 4.7 com `Control.scale`.
- **Instrumento com latência da ordem do fenômeno mede o estado, não a transição.** 190 ms
  são onze quadros.
- **Nenhuma vai ao Android.** O veto do aparelho continua sem instrumento.

## Consequências

### Positivas

- O Marco 7 não gasta uma hora, e a decisão de não gastá-la está apoiada em medição de
  primeira mão no projeto alvo, não em leitura de README.
- O freio da ADR 0011 se prova barato e eficaz: uma ADR impediu um experimento redundante
  cuja redundância só apareceria no fim.
- A pergunta que interessa fica isolada do trabalho que outros já fizeram, e fica menor.
- É coerente com a [ADR 0013](0013-manter-a-sara-privada-ao-fim-do-marco-6.md): manter
  privado e construir um segundo binário ao mesmo tempo seria decisão contra decisão.

### Negativas

- **A ADR 0004 era a única frente autorizada de crescimento, e ela morre aqui.** Depois
  desta decisão e da 0013, o projeto não tem experimento aberto — só o método da ADR 0012
  para capacidade nova e as perguntas que sobraram. Quem quiser ler isto como o projeto
  chegando ao fim tem base para fazê-lo, e esse é o custo honesto de não fingir trabalho.
- **A comparação envelhece, e já envelheceu uma vez durante a própria escrita.** Uma
  limitação medida em 26/08 caiu em 27/08. Se a pergunta 7 for retomada daqui a meses, esta
  ADR é história e não instrumento.
- Cancelar com base em medição de **uma** ferramenta é uma amostra de um. O `godot-mcp` e as
  outras duas não foram instaladas nem medidas: foram lidas. O achado (a) é estrutural — a
  lista de propriedades do motor é a mesma para todas —, mas isso é raciocínio, não medição.
- A lacuna que este projeto nomeia está sendo fechada por quem faz as engines. A ADR 0011 já
  disse isto e vale repetir: **valida a tese e reduz o espaço do produto ao mesmo tempo.**

## Conformidade

A fitness function `adr_0011_observe_exige_adr_de_comparacao`, em `tests/governanca.rs`,
continua valendo sem alteração e passa a ser satisfeita por este documento — ela procura uma
ADR de comparação em `docs/decisoes/`, e agora existe uma.

O freio efetivo contra o spike renascer é a lista de binários autorizados da
[ADR 0007](0007-observe-como-binario-separado.md), conferida por
`adr_0007_apenas_binarios_autorizados`: `sara-observe` não está nela, e acrescentá-lo exige
uma ADR que substitua esta.

Não há fitness function automática para "a pergunta 7 não virou projeto de infraestrutura".
Isso é conformidade manual, e fica declarada como manual.

## Critério de revisão

- **Se a pergunta 7 for respondida** — em qualquer direção —, esta ADR não muda; o que muda
  é o que se sabe. Registre a resposta onde a evidência mora.
- **Se as ferramentas resolverem a cegueira do achado (a)** — lendo propriedade que
  `get_property_list()` não publica —, a opção "adotar" volta à mesa com evidência que hoje
  não existe, e esta decisão é revista.
- **Se alguma delas alcançar o Android**, o veto do aparelho ganha instrumento e a
  comparação inteira precisa ser refeita: seria a primeira vez que uma ferramenta externa
  chega onde a evidência que falta a este projeto é colhida.
- **Se o projeto for retomado depois de meses parado**, esta ADR é história. Refazer a
  comparação com os dados daquele momento é obrigatório, e a ADR 0011 já dizia por quê.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma. **Cancela a [ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md)
  e encerra a [ADR 0011](0011-marco-7-exige-comparacao-com-ferramenta-existente.md)**, que
  a exigiu.
- Evidência de primeira mão: `docs/COMPARACAO.md` do porte do BomberBoom, seção *"O
  `godot-agent` (gda) medido contra as sondas — 26/08/2026"*, e `docs/PEGADINHAS.md`,
  pegadinhas 13 e 14, do mesmo repositório.
- Fontes consultadas em 28/08/2026: `github.com/aigengame/godot-agent` (v0.12.0, 27/08/2026,
  459 commits), `github.com/satelliteoflove/godot-mcp` (309 commits, 148 estrelas),
  `github.com/defold/extension-automation-bridge` (92 commits, 32 estrelas)
- Última alteração: 28 de agosto de 2026
