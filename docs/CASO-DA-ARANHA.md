# O caso da aranha - nove defeitos, e quem achou cada um

**Data do caso:** 28 de agosto de 2026
**Projeto:** porte do BomberBoom para Godot (`~/godot/bomberboom-gd`)
**Procedência:** a sessão "Conceito da aranha", de quase cinco horas, lida integralmente
em 28/08/2026. Os artefatos citados existem no repositório do porte e foram conferidos um
a um; as falas do autor e da agente estão no registro da sessão.

Isto **não é uma ADR** e não decide nada. É a matéria-prima de uma: o caso escrito antes
de a conclusão ser tirada, como o `RESULTADOS.md §4` exige e como a
[ADR 0012](decisoes/0012-sara-e-corpus-coevoluem.md) passou a obrigar para toda dor local
que se candidata a virar capacidade.

A pergunta que o organiza é a do autor, e é a pergunta central deste projeto: **o que
poderia ter sido visto antes de precisar dele.**

O caso está registrado como **uso 3 do Marco 6** em [`USO-PESSOAL.md`](USO-PESSOAL.md).

---

## 1. A sessão, em uma frase

Uma peça nova de jogo - a aranha que rouba a bomba - foi construída, testada com 303
casos verdes, aprovada pelo portão de cena, aprovada pela Sentinela, aprovada pelo
`sara check` com saída 0, carimbada pelo Carimbador, mergeada na `main` - e **não
funcionava**: nenhum roubo se completava, em nenhuma condição de jogo.

Quem descobriu foi o autor, jogando.

---

## 2. Os nove defeitos, e quem achou cada um

| # | defeito | quem achou |
|---:|---|---|
| 1 | 543 nós para três bichos: o nó era criado e nunca guardado | sonda de contagem escrita na hora; virou o portão `PresencaNaCena` |
| 2 | o fio de seda invisível - `z_index = -1` é relativo ao pai, e o pai mora em `$Gemas` | sonda quadro a quadro, escrita e jogada fora |
| 3 | a aranha desenhada um quadro na célula de destino antes de saltar para o topo | `ver_a_entrada.gd`, feito para outra coisa |
| 4 | dois travamentos sem saída (bomba presa ocupando lugar de simultânea) | raciocínio durante a escrita; nenhum instrumento |
| 5 | a ordem dentro de `avancar`: os 0,5 s de tecer eram consumidos em tempo nenhum | um caso de teste da própria agente |
| 6 | `main/bomba_na_tela.gd` com identificador não declarado - 303 casos verdes com o script quebrado | o portão de cena |
| 7 | o piloto automático era cego para a peça nova: 90% de mortalidade contra 10% | comparação com a run sem aranha |
| 8 | **o roubo nunca se completa: 0 de 36 encontros** | **o autor, jogando** |
| 9 | **os nós da bomba e da carregadora presos na tela para sempre** | **o autor, com uma captura** |

O padrão está na coluna da direita e a própria agente o nomeou no fecho:

> Tudo que **eu** achei era sobre o **mecanismo**. Tudo que **você** achou era sobre a
> **peça como se joga**.

Os sete primeiros são defeitos de engrenagem. Os dois últimos são a peça inteira, e são
os dois que os portões deixaram passar.

Vale registrar o que os portões fizeram, porque não fizeram nada de errado: **de nove
defeitos, portão pegou um** - o 6, e é o mais raso. Três dos nove foram achados por
gerar uma imagem e olhar. Dois foram achados pelo autor.

---

## 3. Por que os dois que importavam vazaram

Cinco mecanismos, e nenhum deles é descuido. Todos são efeito de instrumentos corretos
usados como foram desenhados.

### 3.1 O botão girado - e o hábito certo aplicado à pergunta errada

A regra do jogo é uma **corrida entre dois relógios**: o pavio da bomba queima enquanto a
aranha tece, desce e sobe. Os números da run:

| | segundos |
|---|---:|
| pavio | 2,20 |
| andar até a bomba | até 0,50 |
| tecer | 0,50 |
| a carregadora descer | 0,35 |
| subir, por célula | 0,20 |

Do meio do tabuleiro até a borda de cima são sete células: 1,40 s. Sobravam 0,85. **Não
era balanceamento apertado - era impossível**, e a conta cabe em duas linhas.

Nenhum instrumento a fez. Os 18 casos da suíte e a sonda que produziu a frase *"o roubo
funciona"* rodavam com `bomba.pavio = 999.0` - pavio infinito, para isolar as fases - ou
com `pavio = 0.01`, para a bomba estourar na hora. Está versionado e é conferível:
`tests/aranhas_spec.gd` em `07b452e`, linhas 113, 160 e 183 contra 141, 152, 201 e 243.

O detalhe que faz esta lição valer para outros projetos: **testar os dois extremos de um
parâmetro é o hábito correto, e é exatamente o errado para uma corrida.** Extremo isola,
e é para isso que serve quando se prova uma regra. Mas uma corrida entre dois relógios só
existe na faixa do meio: com 999 o roubo sempre vence, com 0,01 a bomba sempre vence, e
nos dois casos **não há corrida para observar**. O valor de produção, 2,20, não aparecia
em caso nenhum.

O conserto foi um caso - `O PAVIO QUEIMA DURANTE O ROUBO INTEIRO` - rodando em tiques de
1/60, como o jogo roda. Era o caso que decidia se a peça existe, e ele não existia.

### 3.2 O ponto cego declarado em prosa, e mergeado assim

O carimbo da tarefa, no repositório do porte em
`docs/carimbos/2026-08-28-conceito-da-aranha.md`, diz textualmente, no passo 4:

> **Deixado de fora, e por quê:** [...] a medição do roubo não foi feita (o andar 2 está
> para ser redesenhado, e medir contra um tabuleiro que vai mudar é gastar a conta duas
> vezes)

A lacuna foi vista, escrita, versionada e mergeada. A justificativa era sobre
**balanceamento** - quanto a aranha custa ao jogador - e a medição que faltava era outra:
a que decidia se o mecanismo **acontece**. As duas foram tratadas como a mesma coisa
porque nada distingue uma da outra num parágrafo.

**Nenhum portão lê prosa.** Para o CI, lacuna declarada e lacuna fechada são idênticas.

### 3.3 A pergunta certa, feita, não respondida, e implementada como se respondida

Ao receber o conceito novo, a agente devolveu cinco perguntas em aberto. A terceira era:

> em que altura ela deixa de fazer qualquer coisa?

O autor respondeu uma - *"sim, o pavio continua queimando"* - e mandou implementar. A
agente implementou, e escolheu sozinha "sobe até a borda de cima do tabuleiro". Foi
exatamente essa escolha que tornou o roubo impossível. Ela reconheceu depois:

> "Sobe até o topo" foi leitura minha, não sua palavra.

A correção do autor custou seis palavras: *"teto é sair da tela mesmo"*, refinada em
seguida para *"talvez sair do tabuleiro, não da tela"*. Uma frase, depois de uma hora de
construção em cima do palpite.

O defeito de processo não é ter chutado - é que **o chute não ficou marcado como o que
sustentava a peça**. Pergunta aberta que vira escolha silenciosa perde a única
propriedade útil que tinha: ser visível.

### 3.4 A referência que nasce com o defeito

A Sentinela compara 25 telas gravadas. Ela não viu, e **não podia**: não há entre as 25
nenhum instante de aranha-com-bomba, e mesmo que houvesse, a referência teria sido
gravada já contendo o defeito.

Isto não é falha da Sentinela - é a definição dela. Referência pega tela que **estava
certa e ficou errada**. Esta nasceu errada. A [ADR 0012](decisoes/0012-sara-e-corpus-coevoluem.md)
já registrava a distinção entre prova por referência e prova por afirmação; a aranha é a
confirmação empírica dela, e acrescenta o corolário: **nenhuma das duas pega o que
ninguém pensou em afirmar** - e o que ninguém pensou em afirmar, aqui, era a peça
inteira.

### 3.5 O carimbo retrospectivo não pode provar o que o carimbo existe para provar

O Carimbador - o instrumento que separa "o pedido estava errado" de "a agente entendeu
outra coisa" de "a construção não cumpriu" - **nasceu no meio desta sessão**, com a
aranha já construída. O carimbo da aranha é retrospectivo e diz isso de si, no topo:

> reconstrução sai sempre coerente com o que aconteceu, que é justamente o defeito que o
> carimbo existe para evitar.

Salvou-se um fragmento genuíno, e ele é o mais valioso do documento: quando o autor
perguntou *"só para eu ver se você entendeu meu pedido"*, a agente escreveu **as sete
coisas que eram leitura dela e não palavra dele**, antes de qualquer mudança. Essa lista
existia na cabeça dela desde o começo e foi publicada só quando perguntada - três horas
depois. É o argumento inteiro do Carimbador, demonstrado pela ausência.

E o carimbo **comparou e passou**, o que é a falha mais instrutiva das três: ele confronta
o pedido com a entrega *como a agente a observou*, e ela observou pelo mesmo instrumento
furado da §3.1.

---

## 4. Como cada um poderia ter sido evitado

| mecanismo | o que teria bastado | custo |
|---|---|---|
| §3.1 botão girado | um caso que roda o encontro com o valor de produção, em tiques de 1/60 | um caso |
| §3.2 lacuna em prosa | "deixado de fora" ser campo que um portão lê, e não parágrafo | um formato |
| §3.3 pergunta não respondida | pergunta sem resposta vira **suposição declarada**, escrita onde o número mora | uma linha |
| §3.4 referência | nada. Referência não pega o que nasceu errado, e isso é definição | - |
| §3.5 carimbo no fecho | o carimbo escrito na abertura - que é o que a skill já manda fazer | uma resposta |

A linha do meio é a que mais paga: das cinco, a §3.3 é a única em que o autor teria
gastado **uma frase** para evitar **uma hora**, e a informação necessária já estava
escrita - só não estava marcada como pendente.

---

## 5. O que a Sara poderia ter visto sozinha

Seis candidatos, em dois grupos muito diferentes. Nenhum está decidido: pela
[ADR 0012 §3](decisoes/0012-sara-e-corpus-coevoluem.md), capacidade generalizável só entra
depois de confrontada com o corpus, e nenhum deles foi.

### 5.1 As três que o próprio porte nomeou - e são da família que a Sara já modela

Estas valem mais que as outras três, e a diferença é de natureza: **não pedem eixo novo.**
São todas de posse de animação em Godot, que é exatamente o que a Sara faz hoje. Foram
escritas pela sessão, na hora, nas linhas 11 a 13 do `.sara/USOS.md` do porte.

| # | o que a Sara não vê | por que dói |
|---:|---|---|
| A1 | **`pause()` / `play()` num Tween** | ela vê a propriedade e o dono, não o fato de o relógio ter parado. Uma bomba presa para sempre, com o tween pausado, é indistinguível de uma que está queimando |
| A2 | **`set_speed_scale` num Tween** | ela modela quem *anima* a propriedade, não o relógio com que anima. Aqui não é detalhe: *"um pavio que passou a queimar na metade da velocidade é indistinguível de um que não passou - e essa é a regra inteira da peça"* |
| A3 | **profundidade por ordem de filho** | `z_index` é relativo ao pai e `move_child` decide quem desenha na frente de quem. Nenhum vira declaração, e **um sprite invisível passa por todos os portões** - foi o que aconteceu com o fio de seda (defeito 2) |

A A2 é a mais forte das seis. O `set_speed_scale` **é** a regra da peça depois do conserto:
a teia não para o pavio, ela o lentifica. Quer dizer que a Sara varreu 1142 arquivos, contou
302 declarações e deu saída 0 sobre um mecanismo cuja regra central mora numa chamada de
Tween que ela não modela. Isso é um limite nomeado do inventário, não um limite do método.

O que as três têm em comum, e que sugere a forma da regra: as três são **modificações do
relógio de um Tween** ou **da ordem de desenho**, e nenhuma modifica a propriedade animada.
A Sara declara alvo, propriedade e dono; nenhuma das três aparece nessas três coordenadas.

### 5.2 As três que pedem eixo novo

O que segue é hipótese com discriminador e com risco escrito - não proposta de regra.

### C1 - parâmetro de tempo que nenhum caso exercita no valor de produção

**O sinal.** Um campo numérico que o jogo alimenta com um valor de produção, e que o
corpus de testes só atribui em valores de outra ordem de grandeza. Aqui: produção 2,20;
testes 999,0 e 0,01.

**O discriminador, e ele é o que separa a regra útil da regra barulhenta.** Teste que
sobrescreve um parâmetro é prática correta e sozinho não diz nada - avisar sobre isso
seria ruído em todo projeto do corpus. O sinal é a conjunção: *todo* caso que exercita o
campo o sobrescreve, **e** nenhum usa o valor de produção, **e** o campo é consumido no
mesmo passo de tempo que outro campo do mesmo tipo. É a forma da
[ADR 0009](decisoes/0009-baseline-em-projeto-real-expoe-regra-ausente.md): exigir os dois
lados é o que impede a regra de calar ou de gritar.

**O que ela custa.** Um eixo novo - cobertura de parâmetro -, que não é extensão da regra
de posse. Continua estático, offline e sem runtime, então cabe na
[ADR 0001](decisoes/0001-validar-mecanismos-antes-da-engine-completa.md); mas exige ler
`tests/` e correlacionar valores entre código de produção e código de teste, o que hoje o
scanner não faz.

**O risco.** Alto de falso positivo, e honestamente maior que o das regras atuais: a
noção de "valor de produção" é clara neste jogo e pode não ser em outro. Se o confronto
com o corpus produzir aviso falso no `gods` ou no `boomlitude`, a hipótese cai - e cair é
um resultado.

### C2 - script que nenhum caminho de teste carrega

**O sinal.** 303 casos passaram com `main/bomba_na_tela.gd` quebrado, porque nenhum teste
carrega aquele script. Isso não é defeito da aranha: vale para qualquer script de `main/`,
e é ponto cego de cobertura declarável sem executar nada.

**O que a Sara já tem para isso.** O inventário. A fitness function do Marco 2 é "nenhum
arquivo relevante é omitido", e ela já percorre o projeto inteiro. Dizer quais scripts
nenhum teste alcança é aritmética sobre um inventário que já existe.

**O limite honesto, e ele importa.** A Sara **não teria pego o erro em si**. `_tranco` não
declarado é erro semântico, não sintático: o tree-sitter analisa o arquivo sem reclamar, e
a Sara não resolve identificadores nem mantém tabela de símbolos. O que ela poderia
relatar é o **fato de cobertura** - "este script não é carregado por caminho de teste
nenhum" -, que é o que teria feito alguém olhar. Confundir as duas coisas seria prometer
o que a ferramenta não faz.

**O risco.** Em Godot muito script é carregado pelo motor via `.tscn`, não por `preload`
em teste. Sem ler as cenas, a regra acusaria `main/` inteiro. É trabalho real, e é a razão
de isto ser hipótese.

### C3 - desenho condicionado a evento, com estado que muda sem evento

**O sinal.** O defeito 9 tem causa nomeável: o redesenho da cena estava preso a *"só
quando há evento"*, e a bomba roubada, com raio 0, não produz evento nenhum. Os nós
ficavam para sempre. O conserto foi fazer a cena **seguir o estado**, como o resto do
tabuleiro.

Isso tem a forma de um problema de posse - a presença na cena passa a ter duas fontes de
verdade, o fluxo de eventos e o estado, e elas divergem -, que é a família que a Sara já
conhece.

**É a mais fraca das três, e fica registrada como fraca.** Não sei dizer se é expressável
estaticamente, e não vou fingir que sei. Fica anotada porque a causa é nomeável e porque
o porte já a transformou em portão local (`PresencaNaCena`), o que a torna candidata a
generalização - o caminho que a ADR 0012 declarou: dor local, hipótese, confronto,
incorporação ou recusa.

---

## 6. O que a Sara não veria, e a quem pertence

O defeito decisivo - 0 de 36 - é **aritmética sobre durações num domínio que a Sara não
modela**: 2,20 s de pavio contra 0,50 + 0,35 + 1,40 de roubo. Nenhum analisador estático
sabe que esses números correm um contra o outro, e nenhuma versão melhor de um verificador
de posse vai saber.

Isto é honesto e é o mesmo veredito que o `USO-PESSOAL.md` já registrou para as onze
pegadinhas do porte: **a Sara vigia uma porta específica num corredor com várias portas
caladas.** O caso da aranha não muda o tamanho da porta. Ele acrescenta uma medição ao
denominador: mais dois defeitos da classe "só olho humano acha", e a Sara não cobre
nenhum dos dois.

O que o caso sugere é que a peça que falta **para este defeito** não é uma regra estática
melhor - as três da §5.1 são regras estáticas e nenhuma delas o pegaria. É a **verdade de
design declarada** que a ADR 0012 nomeou e explicitamente deixou sem formato, sem lugar e
sem dono. As §3.2 e §3.3 são as duas faces dela: uma lacuna declarada que nenhum portão
lê, e uma suposição que sustenta a peça e não está escrita em lugar nenhum que se confira.

---

## 7. O achado que atinge uma decisão já aceita

A [ADR 0004](decisoes/0004-spike-de-visao-instrumentada-em-godot.md) autoriza, depois do
Marco 6, o spike de visão instrumentada. Suas fitness functions, no cenário mínimo:

> três regressões visuais injetadas e suas versões corrigidas
> [...] as três regressões injetadas são detectadas e as três versões corrigidas passam
> [...] o diagnóstico combinado aponta o nó ou propriedade causal nas três regressões

**Todo o critério de aceitação tem forma de regressão** - algo que estava certo e ficou
errado. É a mesma forma da Sentinela, e é o ponto cego da §3.4 escrito dentro de uma
decisão que ainda não foi executada.

Aplicado a este caso: o spike, exatamente como está especificado, teria **passado** num
mecanismo que nunca funcionou. Nenhuma das sete fitness functions pergunta se a peça
alguma vez esteve certa.

Não estou propondo mudar a ADR 0004 - ela já está restringida pela
[ADR 0011](decisoes/0011-marco-7-exige-comparacao-com-ferramenta-existente.md), que exige
comparação com ferramenta existente antes de o Marco 7 começar. O que este caso acrescenta
é uma **segunda pergunta para o mesmo momento**: se o spike mede apenas a capacidade de
reencontrar defeito conhecido, ele mede a metade da lacuna que este projeto existe para
nomear. Fica escrito agora, antes de o spike ser construído, porque depois vira
racionalização.

---

## 8. O que este documento não conclui

- **Não decide capacidade nenhuma.** A1, A2 e A3 são limites nomeados do inventário; C1, C2
  e C3 são hipóteses sobre eixo novo. Nenhuma das seis foi confrontada com o corpus, e a
  ADR 0012 §3 exige o confronto antes da incorporação.
- **Não resolve a discrepância do diário.** O caso entrou como uso 3 do Marco 6, mas o
  diário do próprio porte tem 13 linhas contra as 3 daqui, e as duas leituras possíveis
  mudam o desfecho do portão. Está registrada como pergunta em
  [`USO-PESSOAL.md`](USO-PESSOAL.md), onde o número mora.
- **Não conclui sobre o recorte da Sara.** A previsão datada de 25/08 continua de pé e
  este caso a alimenta em vez de a substituir: dois defeitos novos na classe "só olho
  humano acha", nenhum coberto.
- **Não julga a sessão.** Ela entregou a peça funcionando, com 305 casos, portão verde e a
  regra do autor medida em três configurações. O que se aprende aqui não é sobre
  competência - é sobre **quais instrumentos existiam e o que cada um, por construção, não
  podia ver.**

---

## 9. Procedência

| o quê | onde |
|---|---|
| a sessão | "Conceito da aranha", `bomberboom-gd`, 28/08/2026 |
| o carimbo, com a classificação das diferenças feita na hora | `docs/carimbos/2026-08-28-conceito-da-aranha.md`, no porte |
| o botão girado, antes do conserto | `tests/aranhas_spec.gd` em `07b452e`, linhas 113, 160, 183 |
| o caso que faltava, depois do conserto | `tests/aranhas_spec.gd:105` - `O PAVIO QUEIMA DURANTE O ROUBO INTEIRO` |
| o teto que fez o roubo existir | `tests/aranhas_spec.gd:213`, e o commit `28330eb` |
| a medição 0/36 e 12/36 | saída do `conta_temporaria.gd`, citada no fecho da sessão |
| as três capacidades ausentes, nomeadas na hora | `.sara/USOS.md` do porte, linhas 11, 12 e 13 |
| a Sara usada | `f1f4d5f`; `sara check` no porte com saída 0, 302 claims em 1142 arquivos |
