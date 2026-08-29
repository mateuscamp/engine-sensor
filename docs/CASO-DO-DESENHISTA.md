# O caso do desenhista - a Sara viu a única parte que parece jogo

**Data do caso:** 29 de agosto de 2026
**Projeto:** porte do BomberBoom para Godot (`~/godot/bomberboom-gd`), branch `desenhista`,
PR #40, aberto no momento desta leitura
**Procedência:** o branch lido inteiro em 29/08/2026, o carimbo
`docs/carimbos/2026-08-29-desenhista.md` do porte, e a medição refeita aqui com o binário
de `dist/`. A moldura da §1 é do autor, dita durante esta leitura.

Isto **não é uma ADR** e não decide nada. É matéria-prima de uma, como o
[caso da aranha](CASO-DA-ARANHA.md): o caso escrito antes de a conclusão ser tirada, que a
[ADR 0012](decisoes/0012-sara-e-corpus-coevoluem.md) passou a exigir de toda dor local
candidata a virar capacidade.

**O que ele acrescenta ao caso da aranha.** Lá a Sara ficou verde sobre um mecanismo que
nunca funcionou, e a lacuna era de **domínio**: aritmética de durações, que nenhuma versão
melhor de um verificador de posse alcança. Aqui ela fica verde sobre uma ferramenta cujo
canal de entrada inteiro ela não enxerga — e entrada é **um dos dois eixos que ela declara
cobrir**.

---

## 1. O que é o desenhista, na moldura de quem o pediu

O desenhista é uma cena própria do porte onde o autor escolhe o andar, escolhe um símbolo
numa paleta e pinta as células com o dedo ou o mouse. Ao lado, um índice diz o que cada
sinal significa — e o índice não é uma lista copiada: a ferramenta monta um desenho inteiro
de cada caractere e pergunta à `Arena` se ele passa, então símbolo novo no domínio aparece
na paleta sozinho. O botão COPIAR devolve o bloco `PackedStringArray` pronto para colar em
`game/content/boca_da_mina.gd`.

A moldura é do autor, e é ela que faz o caso valer a leitura:

> achei que essa ferramenta era um meio termo entre prompt e interface gráfica de engine,
> facilitando comunicação de maneira simplificada

É uma descrição de **canal**, não de recurso. O prompt é texto: uma silhueta de andar
descrita em palavras se perde nas duas direções. A interface gráfica da engine é o extremo
oposto: geral demais, e operá-la é fazer o trabalho, não comunicá-lo. O desenhista fica no
meio — uma superfície pequena, com exatamente o vocabulário que o domínio aceita, que os
dois lados sabem escrever e ler.

**E o laço fechou durante esta leitura.** A bandeira `--desenho`, sendo escrita na sessão
do porte enquanto este documento era medido, abre no desenhista um desenho **proposto**,
vindo de um arquivo:

```text
godot --path . ferramentas/desenhista/desenhista.tscn -- --desenho /tmp/variante.txt
```

Com ela o canal passa a ter mão dupla: a agente propõe um desenho, o autor abre, corrige na
tela e devolve o texto. E o motivo de a ferramenta existir está escrito no passo 2 do carimbo
dela, antes de qualquer linha de código: hoje a silhueta nasce como texto dentro de
`game/content/boca_da_mina.gd`, e *"o autor só a vê quando alguém a desenha para ele numa
mensagem — foi assim que o andar 2 foi redesenhado em 29/08"*.

**Isto é a inversão do artigo 1.** [`O Agente Não Vê`](../artigos/1-o-agente-nao-ve.html)
trata da direção agente → autor, e é contra ela que este projeto construiu sondas, capturas
e portões. A direção autor → agente ganhou instrumento no mesmo dia, e por outro caminho: a
[ADR 0015](decisoes/0015-a-verdade-de-design-sao-tres-campos-no-carimbo.md) decidiu que a
verdade de design são três campos no carimbo — `ACONTECE`, `SUPONHO`, `FORA` —, e o carimbo
deste próprio caso os tem preenchidos. **São irmãos nascidos no mesmo dia, e a diferença
entre eles é o que este caso acrescenta:** para *intenção* a prosa serve, e a ADR 0015 é a
prova; para uma *forma* — a silhueta de um andar, onze linhas por nove colunas — a prosa é a
codificação errada, e nenhum campo de carimbo a conserta.

O desenhista é o primeiro artefato deste corpus que carrega intenção **como desenho**. Toda
ferramenta anterior — console, sondas, Sentinela — relatava; o carimbo declara, em texto.

---

## 2. O que a Sara viu

```text
git -C <worktree> archive 3ba88af | tar -x -C /tmp/desenhista-3ba88af
./dist/sara-linux-x86_64 check /tmp/desenhista-3ba88af
Sara 0.1.0 - 164 arquivo(s), 53 declaração(ões), 0 erro(s), 0 aviso(s)
```

Saída **0**. Nenhum diagnóstico.

**A medição está presa ao commit `3ba88af`, e por um motivo que é do caso:** a sessão do
porte estava escrevendo o `--desenho` no mesmo arquivo enquanto esta leitura corria, e entre
duas contagens minhas o `desenhista.gd` ganhou um sítio de sinal. Medir a árvore suja daria
um número que ninguém consegue reproduzir depois. Exportado o commit, os 164 arquivos são os
do projeto de verdade — sem o ruído de contagem que o `USO-PESSOAL.md` anotou para as
execuções feitas do checkout principal, que hospeda dez worktrees.

Das 53 declarações, **47 são de animação e 6 de entrada**. As 6 saem de dois lugares:

| origem | declarações |
|---|---:|
| `main/tabuleiro.gd::_unhandled_input`, ramos de toque e de mouse | 2 |
| `ferramentas/desenhista/prancha_na_tela.gd::_gui_input`, quatro ramos | 4 |

As quatro do desenhista são o que a linha 15 do diário do porte registra, e estão certas: o
`_gui_input` da prancha trata `InputEventMouseButton`, `InputEventScreenTouch`,
`InputEventMouseMotion` e `InputEventScreenDrag`, e os quatro caem em dois normalizadores —
`_dedo`, o aperto, e `_tocar`, a célula.

**E o desenhista não tem uma única animação.** Zero ocorrências de `Tween` em
`ferramentas/desenhista/` e `ferramentas/menu/`. O eixo em que a Sara é forte não tem nada
para medir aqui, e a capacidade mais nova dela — o relógio do Tween, `a3fe810` — não poderia
acrescentar nem uma declaração a este caso, o que torna irrelevante para ele o fato de o
binário de `dist/` estar em `f1f4d5f`.

Sobra a frase inteira do caso: **a Sara viu a única parte do desenhista que se comporta como
jogo** — um dedo numa superfície, com a posição dividida por 42 para virar célula. Todo o
resto da ferramenta é comando, e comando ela não vê.

---

## 3. O que ela não viu, medido

Contados todos os pontos de entrada do projeto — os quatro callbacks de entrada crua e todo
`connect` de sinal de widget (`pressed`, `toggled`, `item_selected`, `value_changed`,
`text_submitted`, `gui_input`, `button_up`, `button_down`, `text_changed`, `tab_changed`,
`confirmed`) em `main`, `game`, `ferramentas`, `tests` e `tools`:

| caminho de entrada | sítios | declarações |
|---|---:|---:|
| callback com classe de evento de ponteiro (`tabuleiro`, `prancha`) | 2 | **6** |
| callback que só testa `InputEventKey` (ESC do desenhista, F1 do console) | 2 | 0 |
| sinal de widget conectado | 29 | 0 |
| **total** | **33** | **6** |

**Dois dos trinta e três pontos de entrada do projeto produzem declaração.** Os 29 sinais de
widget se repartem assim:

| onde | sítios |
|---|---:|
| `ferramentas/console/painel_do_console.gd` e `console_de_testes.gd` | 16 |
| `ferramentas/desenhista/desenhista.gd` | 6 |
| `main/` | 4 |
| `ferramentas/menu/menu_do_jogo.gd` | 3 |

**Os quatro de `main/` não são ferramenta: são jogo que vai ao aparelho.** São o botão da
fúria no HUD (`_abrir.pressed` → `furia_pedida`), o CONTINUAR d'A PARADA, a escolha de cartão
da seção e o CONTINUAR da cerimônia da garrafa. `ferramentas/*` está no `exclude_filter` do
export e não embarca; `main/` embarca. O inventário de entrada do jogo publicado, portanto,
também está incompleto — não só o das ferramentas.

**E o silêncio é o dado.** Nenhum `SAR-PARSE-001`, nenhum aviso, saída 0. Pior: o
`_gui_input` **está** na tabela de construções reconhecidas do
[`COMPATIBILIDADE.md`](COMPATIBILIDADE.md), o que faz a interface parecer coberta. O contrato
inclui a porta que quase ninguém usa em UI Godot e cala sobre o sinal, que é a porta que todo
mundo usa.

---

## 4. A raiz: o modelo é o do DEDO, não o do COMANDO

A regra de entrada da Sara nasceu do uso 2, e nasceu certa. O defeito de origem era de
**canal físico**: com `emulate_mouse_from_touch` ligado, um toque entregava dois eventos e os
dois caíam no mesmo `_dedo`. A [ADR 0010](decisoes/0010-canal-fisico-de-entrada-sem-mapa-de-acoes.md)
fez a regra enxergar canal pela classe do evento testada no ramo, e é por isso que toda
declaração de entrada deste relatório sai com `profile: android` — é lá que o toque entrega
os dois.

*Por qual canal físico o evento chegou* é a pergunta exata para um jogo em que o dedo toca o
mundo. Numa interface de widget ela não tem resposta interessante: `Button.pressed` dispara
igual vindo de toque, de mouse, de teclado, de foco ou de `emit_signal`. O canal físico é
problema do motor. A pergunta que decide passa a ser **quem é dono deste comando** — e
nenhuma regra a faz.

**O registro já tinha dito isso três vezes, com três nomes diferentes**, e é a mesma
re-leitura que as quatro raízes fizeram com os defeitos da aranha: separar mal torna a lição
inútil, e achatar também.

| onde | como foi escrito | o que era |
|---|---|---|
| uso 8 (25/08) | "o Sara não modela TECLA como canal de entrada" (F1 do console) | um sintoma |
| uso 10 (25/08) | "posse de estado de UI entre `Button.button_pressed` e a seção que é a fonte da verdade" | o vizinho: posse de **estado** de widget |
| uso 15 (29/08) | "a mesma lacuna da linha 8, e agora ela custou" (ESC do desenhista) | o mesmo sintoma, de novo |

Os usos 8 e 15 são a mesma coisa dita duas vezes, e a segunda foi classificada como
reincidência da primeira — o que é honesto e, medido, é pequeno demais: **tecla são 2 dos 31
pontos calados; sinal de widget são 29.** O uso 10 é adjacente e não idêntico: ele fala de
quem é dono do estado de um `Button`, não de quem é dono do comando que ele dispara. As três
entradas apontam para o mesmo eixo que falta, e nenhuma das três o nomeia.

---

## 5. Por que isto pesa mais aqui do que pesava no console

O console tem 16 dos 29 sítios e é mais antigo. A diferença não é de tamanho, é de função.

**Ferramenta que relata erra para menos.** Se um botão do console mentir, o autor lê um
número errado sobre o jogo e o jogo continua o que era. **Ferramenta que especifica erra para
mais:** se um botão do desenhista mentir, o desenho que o autor pensa ter feito não é o
desenho que sai no `PackedStringArray` — e o que entra em `boca_da_mina.gd`, e portanto no
jogo, é o texto, não a tela. Com `--desenho` isso vale nas duas direções.

**O caso já tem um defeito exatamente dessa forma, e está no carimbo.** O conversor de
indentação trocou por TAB os espaços de dentro de quatro textos da tela, e o cabeçalho saiu
como `andar 2 —Cereja em Cadeia(hoje: com recorte)`. A suíte ficou verde nos 338 casos, a
Sara ficou verde, e quem viu foi o autor olhando a captura. Numa ferramenta que relata isso é
feiura. Num canal que carrega intenção, um rótulo que mente é defeito **na especificação** —
a mesma classe da "tela que nasceu errada" que criou a `sonda_de_tira` em 28/08, e a mesma
forma da pergunta 8: prova regressiva não pega o que nunca esteve certo.

Vale registrar o tamanho da camada: **51 dos 166 arquivos `.gd` da branch estão em
`ferramentas/`** — mais que os 24 de `main/`. Ela produz 4 das 53 declarações, e as 4 saem de
um arquivo só. (Os 164 varridos são esses 166 menos os 2 do portão em `.sara/`, que o scanner
pula junto com `.git`, `.godot`, `dist` e `target`. `.claude/` não está nessa lista, e é daí
que vem o ruído de contagem já anotado no `USO-PESSOAL.md`.)

---

## 6. O que este caso propõe, sem decidir

**A capacidade candidata é o sinal de widget como canal de entrada:** `X.pressed.connect(f)`
declara que `f` é dono do comando de `X`, com `bind` resolvido, do mesmo jeito que a ADR 0010
entrou — **só declarando, sem diagnóstico novo**, que é o que permite saber depois qual das
duas coisas produziu ruído. Tecla entra junto ou não entra: são 2 sítios contra 29, e a
justificativa dela é outra.

Ela **não** está decidida aqui. A [ADR 0012 §3](decisoes/0012-sara-e-corpus-coevoluem.md)
exige o confronto com o corpus antes da incorporação, e ele não foi feito.

**A previsão, escrita antes de medir**, porque depois vira racionalização: esta capacidade
**generaliza**, ao contrário do relógio do Tween, que entrou achando 3 declarações no porte e
**0** em Gods, Boomlitude, MineBoom e no Defold. Todo projeto Godot com menu tem botão ligado
por sinal.

### O denominador, medido depois da previsão

O confronto que a ADR 0012 §3 exige precisa da regra escrita, e ela não existe. O que dá para
medir agora é o **denominador**: quantos sítios da construção existem em cada projeto do
corpus, contados com o mesmo `grep` da §3, fora de `.claude/` e `.godot/`.

| projeto | arquivos `.gd` | sinais de widget | callbacks de entrada crua |
|---|---:|---:|---:|
| Gods | 452 | **42** | 7 |
| Boomlitude | 99 | **23** | 1 |
| porte BomberBoom, `main`, sem o desenhista | 158 | **20** | 2 |
| MineBoom | 51 | **4** | 1 |
| **total** | **760** | **89** | **11** |

**Oito vezes mais sítios de sinal do que de callback, e nenhum projeto com zero.** É o oposto
exato do que o relógio do Tween encontrou, e é a diferença que importa para a segunda linha
de evidência da [ADR 0012 §5](decisoes/0012-sara-e-corpus-coevoluem.md): lá a capacidade só
existia no projeto que a motivou; aqui a construção está nos quatro, inclusive nos três
parados desde 24/08.

**Duas ressalvas honestas, e as duas apertam o número:**

- **sítio não é declaração.** Isto conta onde a construção aparece, não o que uma regra
  conseguiria resolver. Dos 29 sítios do porte, 25 apontam para um método nomeado (16 com
  `bind`, 2 com `unbind`, 9 diretos) e 2 são funções anônimas — que é onde uma regra
  provavelmente pararia, como o `SAR-PARSE-001` já faz com alvo dinâmico;
- **o Defold não entra.** O original tem 4 `on_input` e 47 `gui.pick_node`: lá o botão mora
  **dentro** do callback que o adapter já reconhece, então a forma não é a mesma e o corpus
  congelado da [ADR 0005](decisoes/0005-foco-em-godot-com-defold-congelado.md) não confirma
  nem desmente esta previsão.

**A linha de contrato que falta, e que é escolha de escopo e não conserto:** hoje o
[`COMPATIBILIDADE.md`](COMPATIBILIDADE.md) lista `_gui_input` entre as construções
reconhecidas e não diz, em "Fora do contrato", que sinal de widget não é entrada. A redação
proposta é *"conexão de sinal de widget (`pressed`, `toggled`, `item_selected`) não é
reconhecida como canal de entrada; interface construída por botão não produz declaração"*.
Não foi aplicada: o contrato não promete o que o código não faz, e acrescentar um limite
explícito é decisão do proprietário.

---

## 7. O que este documento não conclui

- **Não reabre o Marco 6.** Ele encerrou por conclusão em 28/08 com treze mudanças, e a
  [ADR 0013](decisoes/0013-manter-a-sara-privada-ao-fim-do-marco-6.md) decidiu manter a Sara
  privada. Os usos 14 e 15 estão no diário do porte, que continua; a tabela fechada de
  [`USO-PESSOAL.md`](USO-PESSOAL.md) não é renumerada por este caso.
- **Não acusa a Sara de ter perdido um defeito.** Ela não perdeu: no desenhista não havia
  defeito da classe que ela procura, porque não há animação nenhuma. O que se mediu é o
  tamanho do que ela não olha, não um erro dela.
- **Não julga a sessão que construiu o desenhista.** Ela entregou com dois portões, conferiu
  os três injetando defeito, e nomeou sozinha a lacuna da tecla no diário. O que se aprende
  aqui é sobre **para onde o instrumento aponta**, não sobre quem o usou.
- **Não decide se `ferramentas/` deve ser varrido.** É a mesma família da anotação sobre
  `.claude/worktrees/` no `USO-PESSOAL.md`: escolha de contrato.
- **Não conclui sobre o recorte da Sara.** Alimenta a previsão de 25/08 em vez de a
  substituir, e desta vez do lado de dentro do eixo declarado — que é o que a distingue do
  caso da aranha.

---

## 8. Procedência

| o quê | onde |
|---|---|
| a sessão que construiu | "Ferramenta desenhista de andares", `bomberboom-gd`, 29/08/2026, PR #40 |
| o branch lido | `desenhista`, em `3ba88af`, mais o `--desenho` ainda não commitado |
| o carimbo, com a classificação feita na hora | `docs/carimbos/2026-08-29-desenhista.md`, no porte |
| a linha do diário do porte | `.sara/USOS.md`, linha 15 |
| a Sara usada | `f1f4d5f`, o binário de `dist/`, `sha256 1769280b…4751eb` |
| a medição | `git archive 3ba88af` exportado para `/tmp`, depois `sara check`: saída 0, 53 declarações em 164 arquivos, 0 diagnósticos |
| por que exportada | a sessão do porte editava `desenhista.gd` durante a leitura; a árvore suja deu 30 sítios de sinal contra os 29 do commit |
| o denominador do corpus | mesmo `grep`, em `~/godot/{gods,boomlitude,mineboom,bomberboom-gd}`, fora de `.claude/` e `.godot/` |
| as 6 declarações de entrada | `main/tabuleiro.gd:303,307` e `ferramentas/desenhista/prancha_na_tela.gd:93,96,99,102` |
| os 29 sinais de widget | `grep -rn -E "\.(pressed\|toggled\|item_selected\|…)\.connect\(" main game ferramentas tests tools` |
| os 2 callbacks calados | `ferramentas/console/console_de_testes.gd::_unhandled_input`, `ferramentas/desenhista/desenhista.gd::_unhandled_input` |
| a moldura da §1 | o autor, nesta leitura, 29/08/2026 |
