# Premissa do produto

**Versão:** 1.1 · **Status:** fechada para as execuções
**Anterior:** `PREMISSA-v1.0.md` — a §11 mudou; ver o registro de mudanças no fim
**Material compartilhado, idêntico para todas as engines.**

Este documento descreve **o que o jogo faz**, do ponto de vista de quem o joga e
de quem o observa de fora. Ele não descreve como construir nada. Escolher as
abstrações, as estruturas e as ferramentas é responsabilidade de quem
implementa.

---

## 1. O que é

Um jogo mobile de quebra-cabeça, jogado **em retrato**, inteiramente por toque,
num tabuleiro retangular em grade.

**Não há personagem.** O jogador não move ninguém pelo tabuleiro. Ele escolhe
uma célula, e é ali que a ação acontece.

O jogo cruza duas ideias: a bomba que explode em cruz e atinge o que está na
linha e na coluna, e a combinação de peças iguais que reage em cadeia.

---

## 2. O tabuleiro

O tabuleiro tem **9 colunas por 11 linhas**. Toda célula guarda uma **gema** de
um entre **cinco tipos**: rubi, topázio, esmeralda, ametista e safira.

O tabuleiro começa cheio e **nunca esvazia**: sempre que peças saem, o que
sobrou cai e peças novas entram por cima.

---

## 3. Plantar e explodir

**Tocar numa célula planta uma bomba exatamente ali.** Não há arrasto, não há
gesto contínuo, não há seleção prévia: um toque, uma bomba.

A bomba tem **pavio** — espera um instante e então detona.

A explosão sai em **cruz**: quatro braços nas direções ortogonais, cada braço
alcançando um número fixo de células a partir da bomba. A chama não anda na
diagonal.

Cada fase dá ao jogador um **orçamento fechado de bombas**. Há também um **teto
de bombas existindo ao mesmo tempo**.

---

## 4. Grupos e detonação

Um **grupo** é o conjunto de gemas do mesmo tipo ligadas entre si por adjacência
ortogonal — vizinhança lateral e vertical, nunca diagonal.

Um grupo de **três ou mais** gemas está **instável**.

- Quando a chama alcança uma gema que faz parte de um grupo instável, **o grupo
  inteiro detona junto**, não só a gema atingida.
- Quando a chama alcança uma gema que **não** está num grupo de três ou mais,
  apenas aquela gema é atingida. Não há combo e não há cadeia.

**Nada no tabuleiro marca os grupos.** Não há halo, contorno, brilho ou
qualquer indicação de quais gemas formam grupo. Enxergar onde a cruz pega três
do mesmo tipo é metade do quebra-cabeça, e entregar isso pronto destrói o jogo.

---

## 5. A cadeia

Quando um grupo detona, todo grupo instável que estiver **ortogonalmente vizinho
ao grupo que acabou de sair** se acende e detona na onda seguinte. Essa onda
acende as próximas, e assim por diante.

O **multiplicador de combo sobe a cada elo da cadeia** — a cada onda —, e não a
cada gema destruída. Encadear vale mais do que estourar muito de uma vez.

A cadeia termina quando uma onda não acende nenhuma outra.

---

## 6. Riqueza e durabilidade — são a mesma coisa

Toda gema está num de **três níveis**: 1, 2 ou 3.

**Cada explosão que atinge uma gema leva um nível dela.** A de nível 3 vira 2, a
de 2 vira 1, e a de nível 1 sai do tabuleiro. Ou seja: **uma gema de nível 2
exige duas explosões para sair, e uma de nível 3 exige três.**

**Isso vale inclusive dentro de um grupo que detona.** Quando um grupo de cinco
gemas explode, as gemas de nível 1 saem, mas uma gema de nível 3 que estava no
grupo **não sai** — ela vira nível 2 e permanece no tabuleiro, no lugar onde
estava. Esta é a **única exceção** à regra de que o grupo inteiro detona.

**A perda de nível precisa ser visível.** O jogador vê três gemas virarem duas.
Ele não deve precisar deduzir a durabilidade por tentativa: a peça mostra
quantos golpes ainda aguenta.

Cada nível removido paga **1 de ouro**. O ouro é uma moeda separada da
pontuação: ele mede quanta pedra preciosa foi quebrada, não quão bem se jogou.
Uma gema de nível 3 paga 3 de ouro ao longo dos três golpes que ela leva —
inclusive nos golpes que não a destruíram.

A maior parte do tabuleiro é de nível 1. Gema rica é achado.

---

## 7. Queda e reposição

Quando a cadeia inteira termina — e só então —, o que sobrou **cai** para
preencher os vazios abaixo, e gemas novas **entram por cima** até o tabuleiro
ficar cheio de novo.

A queda e a reposição podem criar grupos novos. Isso é esperado e não dispara
detonação sozinho: o tabuleiro apenas fica pronto para a próxima bomba.

---

## 8. Fase, meta e derrota

Cada fase declara **metas** e um **orçamento de bombas**. As metas são do tipo:

- estourar N gemas de um tipo específico;
- fazer N combos.

A fase é vencida quando todas as metas fecham. É perdida quando as bombas acabam
com meta em aberto.

**A fase N é sempre o mesmo tabuleiro inicial.** Perder e tentar de novo devolve
o mesmo problema, para que a fase possa ser aprendida em vez de sorteada. Só o
tabuleiro inicial é fixo; o que entra por cima na reposição, não.

---

## 9. Run e progressão

Perder uma fase **encerra a run** e devolve o jogador ao começo.

Ao fim de uma run, alguma coisa **permanece** com o jogador e afeta a run
seguinte, de modo que recomeçar não é repetir. O jogador precisa **perceber**
essa diferença sem que ninguém explique.

O ouro pertence à run e não sobrevive a ela.

---

## 10. Monetização

O jogo é distribuído gratuitamente e a comunicação é transparente desde o
primeiro contato.

A demonstração precisa entregar o ciclo completo, não só a interação imediata:

> interação → melhoria → consequência perceptível → fim da run → progressão
> persistente → nova run diferente

Uma derrota muito precoce, antes de o jogador conhecer uma melhoria e perceber
alguma consequência dela, **não** conta como run demonstrativa. Um jogador ainda
aprendendo pode precisar de mais tentativas; ambos devem conhecer as mesmas
ideias.

A barreira aparece **somente em um ponto natural e seguro** — o retorno depois
de uma run, ou o encerramento do conteúdo demonstrativo. Ela **nunca** interrompe
uma partida em andamento, e o jogo não pressiona repetidamente durante a
demonstração.

O progresso obtido na demonstração é preservado. A continuação é apresentada
como continuação, nunca como recomeço.

---

## 11. A arte das gemas

A peça jogável é uma **gema solta**. Não há placa de rocha, pedestal, aro
metálico nem moldura: a gema é a peça.

### A silhueta identifica o tipo

**Cada tipo tem silhueta própria, reconhecível pela forma antes da cor.** Não
vale desenhar cinco recortes iguais e apenas recolorir — a forma é o primeiro
canal de leitura e a cor é a confirmação.

O que identifica o tipo **não é o contorno exato, e sim a orientação e as
pontas**. O contorno muda com o nível; a orientação, nunca.

| Tipo | Cor-base | Nível 1 | Como engorda até o nível 3 |
|---|---|---|---|
| Rubi | vermelho `#FF294D` | losango / pipa vertical, lados retos, **ponta embaixo** | perde a ponta de cima, arredonda e vira **pera de ponta para baixo** |
| Topázio | amarelo `#FFC71A` | triângulo largo, base embaixo, lados retos | os lados arqueiam para fora, o triângulo fica estufado |
| Esmeralda | verde `#29FF6B` | retângulo vertical de cantos arredondados | os quatro cantos viram chanfro em degraus, no corte esmeralda clássico |
| Ametista | roxo `#CC47FF` | gota, **ponta em cima**, base redonda | fica mais bojuda; continua gota |
| Safira | azul `#3370FF` | círculo | continua círculo |

**A gema engorda com o nível.** Os lados, retos e magros no nível 1, arqueiam
para fora no 2 e mais ainda no 3, e os cantos arredondam ou se chanfram junto.
Isso é uma pista de nível por si só, e das que melhor sobrevivem no tamanho
pequeno: magra, cheia, estufada.

**O que nunca muda é para onde a gema aponta.** O rubi aponta para baixo e a
ametista aponta para cima. É só isso que impede que as duas se confundam quando
ambas estão bojudas no nível 3 — e é por isso que a orientação é inegociável,
mesmo com o contorno livre para engordar.

### O acabamento identifica o nível

**A riqueza é o acabamento de uma pedra só, nunca a repetição de várias.**

- **Nível 1 — simples.** Superfície de leitura chapada, que **não cintila**: ou um
  domo liso, ou poucos planos grandes de corte chanfrado. Nunca facetamento fino.
  Um único ponto de brilho especular no alto à esquerda. Aro claro fino.
- **Nível 2 — facetada.** A superfície se quebra em muitas facetas triangulares,
  com claros e escuros da própria cor. A pedra passa a cintilar, mas só na própria
  matiz. Aro claro mais espesso.
- **Nível 3 — irisada.** Facetamento denso irradiando de um centro luminoso, com
  dispersão em arco-íris sobre a cor base: reflexos amarelos, verdes e magenta
  dentro da pedra. Aro claro grosso e luminoso.

O aro engrossa a cada nível, e essa é a pista que melhor sobrevive quando a gema
é vista pequena.

### O que não pode acontecer

- **Nenhuma cópia, nenhuma repetição de peças, nenhum vão interno.** Dois ou três
  corpos pequenos dentro de uma célula deixam buracos entre eles, fazem o
  tabuleiro parecer esburacado e — pior — fazem o nível rico parecer *menos*
  pedra que o nível pobre.
- **Nenhuma variação de tamanho entre os níveis.** A gema ocupa a mesma área da
  célula nos três, com o mesmo fundo visível ao redor. O que muda é o que
  acontece dentro da silhueta, nunca o quanto ela preenche.

### O que a arte precisa entregar

Numa captura do jogo no tamanho real de tela do aparelho, um observador que
nunca viu o jogo deve conseguir, **sem ler código**:

1. nomear qual dos cinco tipos ocupa cada célula, guiando-se pela forma;
2. dadas três gemas do mesmo tipo em níveis diferentes, ordená-las da mais pobre
   para a mais rica e dizer qual é 1, 2 e 3.

Isso é o piso. Acabamento acima disso é bem-vindo e não é exigido.

## 12. Comportamento em aparelho

- O jogo roda em **retrato**, num aparelho real.
- O tabuleiro e os controles precisam funcionar corretamente em **pelo menos
  duas proporções de tela diferentes**, incluindo uma bem mais alta que 16:9.
- O toque precisa acertar a célula que o jogador vê sob o dedo, em qualquer
  proporção suportada.
- Sair do jogo e voltar não pode corromper nem perder o que devia persistir.

---

## 13. Parâmetros iniciais

Pontos de partida, não regras. Ajuste o que o balanceamento pedir e registre o
que mudou e por quê.

| Parâmetro | Valor inicial |
|---|---|
| Tabuleiro | 9 × 11 |
| Tipos de gema | 5 |
| Mínimo para grupo | 3 |
| Alcance da chama | 2 células por braço |
| Bombas simultâneas | 2 |
| Proporção de gemas nível 1 | ~70% |
| Fases | 3 |

A exigência real é mais simples que os números: **cada fase precisa ser
vencível e precisa ser perdível.** Uma fase que se ganha sozinha e uma fase
impossível falham igualmente.

---

## Registro de mudanças

### v1.1a — 2026-08-22

A silhueta não é rígida entre os níveis. O que identifica o tipo é a
**orientação e as pontas**, não o contorno: o rubi aponta para baixo e engorda
até virar pera; a ametista aponta para cima e continua gota. A gema **engorda**
com o nível — lados retos no 1 arqueiam para fora no 2 e no 3 —, o que dá uma
terceira pista de nível, legível no tamanho pequeno.

Precisão no nível 1: dizia "lisa, superfície contínua, sem facetas", o que
descrevia só parte da direção. O nível 1 admite tanto domo liso quanto poucos
planos grandes de corte chanfrado; o que o define é **não cintilar**. O nível 2
ganhou a distinção explícita de cintilar apenas na própria matiz, contra o
arco-íris que é exclusivo do nível 3.

### v1.1 — 2026-08-22

**§11 reescrita.** A direção anterior expressava a riqueza repetindo e fundindo
cópias do mesmo recorte: nível 2 eram duas cópias a ~80% da escala, nível 3 eram
três a ~70%.

Ela foi recusada depois de vista rodando. As cópias deixavam vãos visíveis entre
si, o tabuleiro lia como esburacado, e a redução de escala fazia a gema rica
ocupar *menos* célula que a pobre — o oposto do que a riqueza deveria comunicar.

A direção nova mantém uma pedra só, do mesmo tamanho nos três níveis, e move a
riqueza para o acabamento da superfície: lisa, facetada, irisada, com o aro
engrossando. A silhueta passa a ser explicitamente constante entre os níveis.

**Efeito na aceitação:** o critério de contagem de cópias deixa de existir e é
substituído por ordenação dos três níveis. Ver `ESPECIFICACAO.md`.

### v1.0 — 2026-08-21

Versão inicial, preservada em `PREMISSA-v1.0.md`.
