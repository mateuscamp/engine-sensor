# ADR 0015 - A verdade de design são três campos no carimbo, e um deles bloqueia

**Status:** Aceita
**Data:** 28 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** método. Decide o formato, o lugar e o dono da **verdade de design declarada**,
que a [ADR 0012](0012-sara-e-corpus-coevoluem.md) nomeou e deixou sem os três, e que a
[ADR 0014 §3](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md) registrou como
pré-condição da pergunta 7. **Não altera a Sara**, nem a ADR 0004, nem a 0011.

## Contexto

Duas ADRs nomearam a verdade de design e nenhuma ficou com ela.

A [ADR 0012](0012-sara-e-corpus-coevoluem.md), na seção *"A verdade de design"*, escreveu
que **o agente precisa de uma verdade de design declarada para ter com o que comparar** —
sem ela, um agente que executa conclui *"rodou, logo funcionou"* — e fechou dizendo que
decidir formato, lugar e dono *"exige decisão própria"*. A
[ADR 0014 §3](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md) voltou a ela por
outro caminho, registrou-a como pré-condição da única pergunta que sobreviveu ao
cancelamento do Marco 7, e também não a decidiu.

Uma lacuna declarada em prosa, versionada, e que nenhum portão lê. É a mesma forma do
defeito que o [caso da aranha](../CASO-DA-ARANHA.md) documenta, um andar acima: em vez de
num carimbo, no registro de decisões deste projeto.

### A evidência, e ela é de 28/08/2026

O caso da aranha: uma peça de jogo construída, **303 casos verdes**, portão de cena verde,
Sentinela verde, `sara check` com saída 0, carimbada e mergeada — e o roubo **não se
completava em condição nenhuma**, 0 de 36 encontros. Dos nove defeitos, **portão pegou um**,
e os dois que decidiam se a peça existia foram achados pelo autor jogando.

Dois dos cinco mecanismos daquele caso são exatamente o assunto desta ADR.

**§3.2 — a lacuna declarada em prosa, e mergeada assim.** O carimbo da tarefa, no passo 4,
diz textualmente: *"a medição do roubo não foi feita (o andar 2 está para ser redesenhado, e
medir contra um tabuleiro que vai mudar é gastar a conta duas vezes)"*. A justificativa é
sobre **balanceamento** — quanto a aranha custa ao jogador. A medição que faltava era outra:
a que decidia se o mecanismo **acontece**. As duas foram tratadas como a mesma coisa porque
**nada distingue uma da outra num parágrafo**. Para o CI, lacuna declarada e lacuna fechada
são idênticas.

**§3.3 — a pergunta aberta que virou escolha silenciosa.** A agente devolveu cinco perguntas
ao receber o conceito; a terceira era *"em que altura ela deixa de fazer qualquer coisa?"*.
Ela não foi respondida, e a agente escolheu sozinha *"sobe até a borda de cima do
tabuleiro"* — a escolha que tornou o roubo impossível. A correção do autor custou seis
palavras, depois de uma hora de construção em cima do palpite. O defeito de processo não foi
chutar: foi **o chute não ficar marcado como o que sustentava a peça**.

### Por que uma oitava fitness function não se sustenta sozinha

A [ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md) fixou sete fitness functions e
**todas têm forma de regressão** — algo que estava certo e ficou errado. O spike, exatamente
como especificado, teria passado na aranha. O reparo óbvio seria acrescentar um oitavo item:
*"o instrumento detecta uma peça que nunca esteve certa"*. Ele não se sustenta, e o porquê é
o conteúdo desta decisão:

- **prova por referência** — as 25 capturas da Sentinela — pega tela que estava certa e ficou
  errada. Não pega o que nasceu errado, porque **não há referência correta com que comparar**.
  Isso não é falha da Sentinela: é a definição dela;
- **prova por afirmação** pega tela que nasceu errada, mas só o que alguém pensou em afirmar.
  No caso da aranha, ninguém pensou em afirmar que o roubo se completa;
- logo o oitavo item exige uma **terceira coisa**: um critério de aceitação declarado **antes**
  da construção, contra o qual a execução possa ser conferida. Sem ela, o oitavo item é uma
  frase que ninguém sabe medir.

### A forma barata que já roda, e onde ela para

A skill **Carimbador**, em `bomberboom-gd/.claude/skills/carimbador`, já faz quase tudo. O
passo 2 (*"Conceito entendido"*) **congela** e pede *"Critério de pronto — a frase que decide
se acabou. Uma só, verificável. Nada de 'está funcionando'"*; o passo 4 declara o que ficou de
fora; e a skill manda anunciar o passo 2 na primeira resposta, em até seis linhas, porque **é
aí que erro de pedido custa uma frase**.

Já existe portão, e ele já grava: `bomberboom-gd/tests/carimbos_spec.gd` escreve
`docs/carimbos/registro.txt`, uma linha por carimbo, com o veredito de ordem tirado do git
(`antes`, `junto`, `fecho`, `sozinho`). E ele declara no próprio cabeçalho o que é e o que
não é: *"Ele checa PRESENCA, nunca prosa [...] Se o carimbo e BOM, quem diz e quem le."*

O elo que falta é um só: **o passo 2 carrega parágrafo, e portão não lê parágrafo.**

### A objeção, e ela é forte

O passo 2 da aranha **tinha** critério de pronto, e ele foi cumprido: *"a aranha e o ninho
existem no domínio com casos próprios, aparecem na tela pelo caminho de produção, e o custo
da aranha no andar em que ela estreia está medido em número"*. Dezoito casos da aranha dentro
dos 303, arte provisória com portão, captura da tela, 100 runs medidos. Um campo conferível
por máquina com esse mesmo conteúdo teria passado igual, porque o critério falava de
**artefatos existirem** e não de o **mecanismo acontecer**. **Formato não faz ninguém pensar
na coisa certa.**

A objeção está certa, e a resposta é mais estreita que ela. **Ninguém precisava pensar: já
estava escrito.** O carimbo dizia que a medição do roubo não fora feita. O que falhou não foi
a percepção — foi a lacuna percebida não ter consequência. O trabalho do formato não é
produzir a ideia; é tornar **bloqueante o que já foi escrito**. Essa afirmação tem
exatamente o tamanho da evidência, e nada além dele.

## Opções consideradas

1. **Não fazer.** A verdade de design continua sem formato, lugar e dono, como as ADRs 0012
   e 0014 a deixaram.
2. **A Sara passa a ler a verdade de design** — um arquivo declarado no projeto que o scanner
   lê e confronta com o código.
3. **Documento próprio neste repositório** — um `docs/VERDADE-DE-DESIGN.md`, com o critério
   por peça.
4. **Três campos no passo 2 do carimbo, no repositório de quem constrói, e um deles bloqueia
   o fecho.**

### Por que não a 1

Ela tem o melhor argumento de todos, e ele é de escopo: depois das
[ADRs 0013](0013-manter-a-sara-privada-ao-fim-do-marco-6.md) e
[0014](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md) **este projeto não tem
experimento aberto**; a pergunta 7 pode nunca ser retomada; e decidir formato para um
documento que mora em outro repositório é legislar onde o `cargo test` daqui não enxerga.

O que a derruba é o custo, e ele está escrito: a ADR 0014 §3 fez da verdade de design a
**pré-condição** da única pergunta que sobrou. Deixá-la em aberto deixa a pergunta 7 sem
como ser respondida, e repete um andar acima o defeito da §3.2 — lacuna declarada em prosa,
que nenhum portão lê. Não decidir aqui é o caso da aranha aplicado às decisões deste projeto.

### Por que não a 2

Ela contraria a [ADR 0001](0001-validar-mecanismos-antes-da-engine-completa.md) e a
[ADR 0014 §2](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md), que escreveu que
nada ali autoriza binário, daemon, addon, autoload, rede ou dependência. Seria eixo novo — ler
declaração e correlacioná-la com execução —, e pela [ADR 0012 §3](0012-sara-e-corpus-coevoluem.md)
capacidade só entra depois de confrontada com o corpus, que para esta não existe.

E ela não resolveria o caso que a motivou. O defeito decisivo da aranha é **aritmética sobre
durações** — 2,20 s de pavio contra 0,50 + 0,35 + 1,40 de roubo — num domínio que a Sara não
modela, e o [caso §6](../CASO-DA-ARANHA.md) já registrou que nenhuma versão melhor de um
verificador de posse vai saber disso. Uma Sara que lesse a verdade de design continuaria sem
conferi-la.

### Por que não a 3

A verdade de design é sobre a **peça que está sendo construída**, não sobre a Sara. Escrita
aqui, ela seria escrita depois, por quem escreve ADR, num momento em que a peça já existe —
que é o defeito da [§3.5](../CASO-DA-ARANHA.md): o carimbo da aranha foi retrospectivo e diz
de si mesmo que *"reconstrução sai sempre coerente com o que aconteceu, que é justamente o
defeito que o carimbo existe para evitar"*.

## Decisão

Nós adotamos a opção 4.

### 1. O formato: três campos, com nome, no passo 2

O passo 2 do carimbo passa a carregar um bloco **Verdade de design** com três campos de nome
fixo. Cada um existe porque um mecanismo medido do caso da aranha o exige, e nenhum foi
inventado por simetria:

| campo | o que ele carrega | de onde ele vem |
|---|---|---|
| `ACONTECE` | o evento observável que decide se o mecanismo existe, o caso que o observa (`arquivo:linha` ou nome) e **o valor de produção** com que esse caso roda | [§3.1](../CASO-DA-ARANHA.md) |
| `SUPONHO` | uma linha por pergunta que o autor não respondeu e o agente decidiu sozinho, com o `arquivo:linha` onde a escolha mora | [§3.3](../CASO-DA-ARANHA.md) |
| `FORA` | uma linha por medição ou verificação não feita, cada uma com **classe**: `bloqueia` ou `adia` | [§3.2](../CASO-DA-ARANHA.md) |

**`FORA` aparece duas vezes: no passo 2 e de novo no passo 4**, e o portão lê os dois. No
passo 2 vai o que já se sabia que não seria medido; no passo 4, o que a construção
descobriu — e é lá que estava o da aranha. Ler só o passo 2 deixaria passar exatamente o
caso que originou esta decisão. *(Corrigido em 28/08/2026, ao implementar: o texto original
desta seção falava só do passo 2.)*

**`ACONTECE` exige o valor de produção porque extremo isola, e isolar é o hábito certo
aplicado à pergunta errada.** Os 18 casos da aranha rodavam com `pavio = 999.0` ou
`pavio = 0.01`; com 999 o roubo sempre vence, com 0,01 a bomba sempre vence, e **nos dois
casos não há corrida para observar**. O valor de produção, 2,20, não aparecia em caso nenhum.
O conserto foi um caso — `O PAVIO QUEIMA DURANTE O ROUBO INTEIRO` — rodando em tiques de
1/60, como o jogo roda.

**`SUPONHO` tem prova de que funciona quando é acionado.** Quando o autor perguntou *"só para
eu ver se você entendeu meu pedido"*, a agente escreveu **as sete coisas que eram leitura dela
e não palavra dele**, antes de qualquer mudança. Essa lista existia desde o começo e foi
publicada só quando perguntada, três horas depois. O campo torna a pergunta permanente.

**`FORA` tem exatamente duas classes, e a fronteira entre elas é a pergunta que o parágrafo
da aranha não separou:** `bloqueia` é a medição que decide se o mecanismo **acontece**;
`adia` é balanceamento, estética ou número a ajustar. A medição do roubo era `bloqueia` e foi
justificada como `adia`.

### 2. O lugar: o carimbo da tarefa, no repositório de quem constrói

`docs/carimbos/AAAA-MM-DD-<branch>.md`, onde os carimbos já moram — **não neste repositório**.
Esta ADR decide o formato e guarda a razão, porque o método mora aqui; o instrumento mora no
`bomberboom-gd`, porque é lá que a peça é concebida e é lá que o custo de escrever é de uma
frase em vez de uma tarde.

### 3. O dono: quem abre a tarefa escreve, quem pediu corrige numa frase

O agente escreve os três campos **antes de abrir qualquer arquivo do jogo**, e eles vão no
anúncio do passo 2, na primeira resposta, dentro das seis linhas que a skill já pede. O autor
corrige por uma frase — e a §3.3 mediu quanto vale essa frase: **seis palavras contra uma hora
de construção**.

**Ninguém neste repositório é dono da verdade de design de uma peça do jogo.** Fingir o
contrário produziria documento escrito depois, que é a opção 3 recusada acima.

**O custo, dos dois lados.** Escrever: três campos numa resposta que a skill já obriga, e uma
palavra — `bloqueia` ou `adia` — por item deixado de fora. Não escrever: está medido no caso
da aranha, e é o preço que esta decisão compra. Uma hora de construção sobre um palpite
([§3.3](../CASO-DA-ARANHA.md)); e uma peça que nunca funcionou, aprovada por 303 casos, pelo
portão de cena, pela Sentinela e pelo `sara check`, e mergeada com a lacuna decisiva declarada
em prosa no próprio carimbo ([§3.2](../CASO-DA-ARANHA.md)).

### 4. Só uma coisa bloqueia, e ela é estreita

**Um item de `FORA` que não esteja marcado `adia` impede o carimbo de ir a `FECHADO`.** Ele
barra o **fecho da tarefa**, não a escrita de código: o carimbo `ABERTO` continua verde, que é
o estado normal de trabalho em andamento e que o portão de carimbos já trata assim de propósito.

**Item sem classe barra**, e é a regra que faz a classificação valer: o padrão é o lado
seguro, e dizer `adia` vira ato explícito, escrito, datado e com dono. *(Acrescentado em
28/08/2026, ao implementar: sem isso, deixar de classificar seria mais barato que
classificar, e a palavra vira opcional.)*

Tudo o mais é gravado, não barrado. **Ausência do bloco inteiro é gravada**: o portão escreve
`com` ou `sem` numa **coluna própria** do `registro.txt`, e o número responde no tempo — que é
a regra que aquele arquivo já estabeleceu para si: *"Um carimbo escrito no fecho e um
acidente; tres em cinco e um processo que nao esta funcionando."*

A coluna é própria, e não um quinto veredito ao lado de `antes`, `junto`, `fecho` e
`sozinho`, porque **ordem e presença são duas medições**: um carimbo pode nascer `antes` e
não trazer verdade nenhuma, e uma coluna só o obrigaria a declarar uma das duas.
*(Corrigido em 28/08/2026, ao implementar: o texto original desta seção dizia "ao lado de".)*

**O portão lê campo, não parágrafo, e não julga o que está escrito.** Isso não é limitação
descoberta agora: é o que o `carimbos_spec.gd` já declara de si mesmo. Os três campos não
mudam o que o portão julga; mudam **o que precisa estar presente**.

### 5. A oitava fitness function não entra na ADR 0004, e esta é a redação exata

**Não acrescentamos o oitavo item à ADR 0004.** Duas razões, e a segunda vale mais que a
primeira:

- a ADR 0004 foi **cancelada** pela ADR 0014 em 28/08/2026, que declarou que o texto dela
  *"fica intacto, como registro do que foi decidido em 23/08 e do que a comparação encontrou
  depois"*. Acrescentar critério de aceitação a um experimento cancelado é escrever critério
  que ninguém vai rodar — a §3.2 mais uma vez;
- **ele não é um oitavo item da mesma espécie.** As sete medem um **instrumento**. O oitavo
  pergunta **contra o que** o instrumento é conferido. A ADR 0014 §3 já o pôs no lugar certo:
  pré-condição da pergunta 7, e não item ao lado dela.

A redação exata, que passa a valer como pré-condição da pergunta 7 e mora nesta ADR:

> **Pergunta 8 — a prova contra uma peça que nunca esteve certa.** Dado um mecanismo cuja
> verdade de design foi declarada **antes** da construção, no formato da §1 desta ADR, e que
> não cumpre o campo `ACONTECE` em condição nenhuma, a prova passa se a evidência combinada
> nomear **qual campo da verdade de design não é cumprido** — sem dispor de nenhuma execução
> anterior da mesma peça em que ele fosse cumprido, isto é, **sem referência**.
>
> Enquanto a pergunta 8 não tiver resposta, a pergunta 7 mede a capacidade de reencontrar
> defeito conhecido, e apenas isso.

O caso para ela **já existe e não precisa ser injetado**: a aranha no commit `07b452e`, com
0 de 36 encontros e nenhum commit anterior em que o roubo se completasse. A referência para
ela só pôde ser construída **depois** do conserto — que é precisamente o argumento.

### 6. O que isto não autoriza

Nada aqui muda a Sara, acrescenta eixo, arquivo lido pelo scanner, campo de relatório,
dependência, binário, daemon, addon, autoload ou rede. A
[ADR 0001](0001-validar-mecanismos-antes-da-engine-completa.md) e a
[ADR 0013 §2](0013-manter-a-sara-privada-ao-fim-do-marco-6.md) continuam inteiras, e a lista
adiada do [`ROTEIRO.md`](../ROTEIRO.md#o-que-fica-adiado) não encurta em uma linha.

## Consequências

### Positivas

- A dívida que duas ADRs nomearam e nenhuma assumiu fecha, com formato, lugar e dono
  escritos.
- O custo é três campos num instrumento que já roda e uma palavra por item deixado de fora.
  Nenhum instrumento novo, nenhum binário, nenhuma dependência.
- O que bloqueia é estreito e atribuível: não é *"existe lacuna"*, é *"a lacuna foi
  classificada como decidindo se o mecanismo acontece"*.
- A pergunta 7 ganha pré-condição com redação exata, em vez de uma nota dizendo que tem uma.
- A ADR 0004 fica intacta como registro histórico, e agora com portão que confere isso.

### Negativas

- **Decidido aqui, conferido lá.** O formato governa um documento em outro repositório, cujo
  portão o `cargo test` deste não enxerga — e continua não enxergando. Quem ler esta ADR
  procurando o argumento contra ela deve ler esta linha primeiro.

  *O risco de ela ficar no papel foi fechado no mesmo dia: o portão passou a ler os três
  campos em `bomberboom-gd`, verificado injetando o defeito nas cinco combinações, e o
  primeiro carimbo no formato novo é o daquela própria tarefa. Mas isto é registro de um
  fato, não conserto da negativa: as duas metades continuam em repositórios diferentes, e
  nada aqui reprova se aquele portão for removido.*
- **O portão lê a classe, não a verdade.** Uma palavra — `adia` no lugar de `bloqueia` — passa
  qualquer coisa. A falha se desloca de *"ninguém escreveu"* para *"alguém escreveu a palavra
  errada"*. A segunda é visível e atribuível; não é impedida.
- **O formato não produz o pensamento.** O passo 2 da aranha tinha critério de pronto, ele foi
  cumprido, e a peça não funcionava. Só vira bloqueante o que alguém já pensou. O veredito do
  [caso §6](../CASO-DA-ARANHA.md) continua de pé: o defeito decisivo é aritmética sobre
  durações, e formato nenhum o pega.
- **Cerimônia.** A própria skill registra que *"cerimônia repetida em tarefa pequena é cara"* e
  que *"um carimbo que ninguém lê ensina a não ler carimbo"*. Três campos são três chances a
  mais disso. O critério de revisão abaixo é datado por causa disto.
- **Generalização sem medida.** A verdade de design nasceu de um caso, num projeto, como tudo
  o mais do Marco 6. A ADR 0013 já registra essa limitação para a série inteira, e ela vale
  aqui sem desconto.

## Conformidade

**Fitness function automática, `adr_0015_as_sete_fitness_functions_da_0004_continuam_sete` em
`tests/governanca.rs`:** ela lê a seção `## Fitness functions` da
[ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md), conta os itens numerados e reprova
se não forem sete. É a conformidade da §5 — o oitavo critério não é parafusado num experimento
cancelado —, e ao mesmo tempo o mecanismo que faltava à promessa da ADR 0014 de manter o texto
de 23/08 **intacto**. Registro que se edita depois deixa de ser registro.

**Declarado manual, porque não há como automatizar daqui sem fingir:**

- **que os três campos são escritos, e que um item sem `adia` impede o fecho.** Isso mora em
  `bomberboom-gd/tests/carimbos_spec.gd`, e o CI deste repositório não vê aquele. A evidência
  que fica é o `docs/carimbos/registro.txt` de lá, que já grava uma linha por carimbo e passa a
  gravar a coluna `verdade`;
- **que um `bloqueia` não foi classificado `adia` para passar.** Nenhum portão lê a verdade de
  uma classe. O que existe é rastro: a classe fica escrita, datada e versionada, e quem lê o
  carimbo depois vê quem escreveu qual palavra.

Nenhuma fitness function foi escrita para esses dois. Pelo precedente da
[ADR 0012](0012-sara-e-corpus-coevoluem.md), manual declarado vale mais que cobertura fingida.

## Critério de revisão

- **Data: 20 de setembro de 2026** — a mesma da [ADR 0013](0013-manter-a-sara-privada-ao-fim-do-marco-6.md).
  O portão foi escrito em 28/08, então o que a data pergunta deixou de ser *"foi
  implementado?"* e passou a ser **quantos carimbos passaram pela coluna `verdade`, e quantos
  vieram `com`**. Se em 20/09 a série ainda for curta, ou majoritariamente `sem`, o formato
  não pegou e a revisão é entre insistir e revogar. Hoje ela tem **um carimbo, `com`** — o da
  própria tarefa que a implementou, o que é o ponto de partida de uma série e não uma série.
- **A série tem uma exclusão conhecida, medida em 29/08/2026.** O portão só grava a linha do
  `registro.txt` quando a branch tem commit **fora de `docs/carimbos`**, pela razão que ele
  declara de si: *"uma branch so com o carimbo ainda pode virar `antes`; grava-la cedo seria
  carimbar de errado uma coisa que ainda vai acontecer"*. A consequência é que **tarefa cujo
  produto é só documento não entra na coluna**, ainda que traga os três campos — e o segundo
  carimbo no formato novo, o da conferência de 29/08, é exatamente um desses. Quem fizer a
  conta em 20/09 conta os carimbos que **passaram pelo portão**, e não os que existem; se a
  série vier curta, esta exclusão é a primeira coisa a descontar antes de concluir que o
  formato não pegou.
- **O laço tem um elo fora dele, medido em 29/08/2026.** Os três campos e a frase de correção
  conferem a **releitura** do pedido, e não o pedido. Quando ele chega como artefato produzido
  por uma ferramenta — o desenho do [desenhista](../CASO-DO-DESENHISTA.md) é o primeiro —, o
  elo `tela → texto emitido` fica fora do laço: releitura fiel e correção fiel a um texto que
  não é o que o autor viu. Não muda formato nem dono. Em 20/09 isto acrescenta uma pergunta à
  conta: quantos pedidos chegaram por esse caminho, e se algum chegou torto — **zero até
  aqui**, com dois quase, o `unexpand` e as aspas da NOTA. *(Acrescentado em 29/08/2026, a
  pedido do proprietário, depois da leitura do caso do desenhista.)*
- **Contrária:** se um carimbo fechar com todos os itens de `FORA` classificados `adia` e a
  peça não funcionar, a classe não fez o trabalho e o formato cai. Cair é um resultado.
- **A favor:** se um item `bloqueia` segurar um merge ao menos uma vez, o mecanismo se pagou —
  é exatamente o merge da aranha que não devia ter acontecido.
- **Cerimônia:** se os campos voltarem vazios, ou copiados entre carimbos, esta decisão
  produziu forma e não informação, e é revogada em vez de ampliada.
- **Se a pergunta 7 for retomada**, a redação da pergunta 8 é conferida contra o que se souber
  então. A ADR 0014 já registrou que comparação envelhece, e envelheceu em dois dias.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma. **Decide o que a [ADR 0012](0012-sara-e-corpus-coevoluem.md) nomeou e
  deixou sem formato, lugar e dono, e o que a
  [ADR 0014 §3](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md) registrou como
  pré-condição da pergunta 7.** A decisão de nenhuma das duas é alterada: cada uma recebeu
  **só um ponteiro** para esta, no parágrafo onde deixou a pendência. A
  [ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md) e a
  [ADR 0011](0011-marco-7-exige-comparacao-com-ferramenta-existente.md) não foram tocadas, e
  a fitness function acima é o que garante que a 0004 continue assim.
- Evidência: [`CASO-DA-ARANHA.md`](../CASO-DA-ARANHA.md), §§3.1, 3.2, 3.3, 3.4, 3.5, 4, 6 e 7.
- Instrumento citado: skill `carimbador` e `tests/carimbos_spec.gd`, ambos em
  `bomberboom-gd`, lidos em 28/08/2026; carimbo
  `docs/carimbos/2026-08-28-conceito-da-aranha.md` do mesmo repositório.
- **Implementada em 28/08/2026**, em `bomberboom-gd`, na branch `verdade-de-design`: os três
  campos no `modelo.md` e na skill, o portão lendo-os em `tests/carimbos_spec.gd`, a coluna
  `verdade` no `registro.txt`, e 307 casos com 0 falhas. Conferido injetando o defeito em
  cinco combinações. **Duas correções a esta ADR saíram de lá e estão marcadas no texto**: o
  `FORA` do passo 4 e a coluna própria. O primeiro carimbo no formato novo é o daquela
  tarefa, e ele registra que três rodadas saíram verdes com o portão desligado antes de o
  erro aparecer — o caso da aranha em miniatura, dentro da tarefa que existe para
  consertá-lo. **Mergeada na `main` daquele repositório em 29/08/2026, pelo PR #33.**
  A branch continua existindo, e nomeá-la sem o merge já custou uma leitura: em 29/08 uma
  conferência independente rodou os `grep` desta seção contra uma cópia local da `main` três
  merges atrasada, não achou nenhuma das duas peças e concluiu que a "Conformidade" acima
  prometia a mais. As peças estavam lá. **Instrumento citado por branch se lê como
  instrumento possivelmente encalhado** — é o registro que faltava, e é barato.
- Última alteração: 29 de agosto de 2026
