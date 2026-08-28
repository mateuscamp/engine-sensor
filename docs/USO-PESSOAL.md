# Registro do Marco 6 - uso pessoal

> Este é o registro **do próprio Sara**. O modelo distribuído por `sara init` está em
> [`kit/USOS.md`](../kit/USOS.md); a instância criada em cada projeto integrado fica em
> `.sara/USOS.md` daquele projeto. Três arquivos, três papéis distintos.

Preencher uma linha para cada mudança em que animação ou entrada possa ter mudado.

**Critério de conclusão:** dez mudanças reais em projetos Godot.
**Critério de parada:** 20 de setembro de 2026.

O que vier primeiro encerra o marco. Chegar à data com menos de dez mudanças é um
resultado, não um atraso: significa que a ferramenta não está no caminho do trabalho
real, e o encerramento legítimo passa a ser manter só o kit ou congelar a ferramenta.

**A Sara muda durante o marco, e isso é o método — não um descuido.** Pela
[ADR 0012](decisoes/0012-sara-e-corpus-coevoluem.md), a obrigação deixou de ser "não
altere o instrumento" e passou a ser "não altere o instrumento sem deixar evidência". Por
isso a coluna **Sara** existe: ela diz qual instrumento respondeu a cada caso. Quando um
caso alterar a ferramenta, uma nota numerada abaixo da tabela registra o que faltava, o
que mudou e o efeito no corpus inteiro.

**E a série saiu mais controlada do que a ADR 0012 temia:** doze dos treze casos usaram o
mesmo binário, `f1f4d5f`. Nenhum commit tocou `src/` depois de 23/08. A liberdade de
mudar o instrumento durante o marco foi autorizada e quase não foi exercida — o que é um
resultado sobre o método, e está registrado como tal em vez de virar acaso.

A coluna que era "Regra ausente" virou **"Capacidade ausente"**: hoje é regra estática de
posse, amanhã pode ser observação de runtime, entrada, determinismo ou estado visual.

Pela [ADR 0005](decisoes/0005-foco-em-godot-com-defold-congelado.md), mudança em projeto
Defold não conta para a contagem. Nenhum uso registrado é Defold — os treze são do porte,
que é Godot —, então a regra não excluiu nada; ela vale para os que vierem.

**Estado atual: 13 de 10 mudanças. O critério de conclusão está atingido, e o portão do
Marco 6 está aberto** — vinte e três dias antes do critério de parada.

Atingir a contagem **não fecha o marco**: fecha a coleta de evidência. O portão pede uma
decisão entre manter privado, congelar ou propor nova ADR, e essa decisão é do
proprietário. Este arquivo agora carrega a evidência que ela precisa julgar, e não a
conclusão.

> ### A contradição do critério, resolvida em 28/08/2026
>
> O [`RESULTADO-0.1.0.md`](RESULTADO-0.1.0.md), de 23/08, exigia as dez mudanças **"em
> pelo menos dois projetos"**. As treze são todas do porte.
>
> **A frase foi corrigida, e a leitura tardia governa.** O [`ROTEIRO.md`](ROTEIRO.md)
> registrou em 24/08, de propósito e sabendo do risco, que *"as dez mudanças virão todas
> do porte, e isso é ao mesmo tempo a força e o risco do marco"* — depois de a ADR 0005
> deixar um único projeto Godot em desenvolvimento ativo. A exigência de espalhamento era
> contradição não riscada, não critério vivo.
>
> **O que ela protegia não sumiu: virou limitação declarada do julgamento.** Treze
> mudanças de um projeto só não são treze projetos. O portão julga uma série de um jogo, e
> isso pesa contra a generalização — a segunda das duas linhas de evidência que a
> [ADR 0012 §5](decisoes/0012-sara-e-corpus-coevoluem.md) mandou medir.

**Um único projeto em desenvolvimento ativo, e as treze mudanças vieram todas dele.**
Desde 24/08/2026 o porte do BomberBoom para Godot é o único que se mexe; Gods, Boomlitude
e MineBoom ficam parados. O risco de ter um alvo só estava registrado aqui para não ser
descoberto no fim — ele não se materializou como parada, mas continua valendo para a
leitura do portão: **treze mudanças de um projeto só não são treze projetos.** O que se
mediu foi a Sara acompanhando um jogo, não a Sara acompanhando um corpus.

A contrapartida é a que já estava escrita: o mesmo jogo existe nas duas engines, então
cada regra pôde ser conferida contra o original em Defold. Foi assim que a ADR 0010
nasceu.

| # | Data | **Sara** | Projeto | Mudança | Tempo | Conflito | Aviso útil/falso | Inspeção humana necessária | Capacidade ausente |
|---:|---|---|---|---|---:|---|---|---|---|
| 1 | 2026-08-23 | `2603b30` | porte BomberBoom (Godot) | bomba visual com Tween configurado em cadeia fluente | < 1 s | nenhum após correção | nenhum aviso; a primeira execução omitiu 2 declarações | sim, para comparar o inventário com o diff | parser perdia `tween_property` seguido de `set_trans`/`set_ease`; fixture adicionada |
| 2 | 2026-08-23 | `f1f4d5f` | porte BomberBoom (Godot) | `emulate_mouse_from_touch=false`: toque e mouse caíam no mesmo `_dedo` | < 1 s | erro comprovado, corrigido | 0 falsos | não, o diagnóstico bastou | nenhuma; a regra acabara de nascer pela ADR 0010 |
| 3 | 2026-08-24 | `f1f4d5f` | porte BomberBoom (Godot) | pavio que SOME em vez de virar carvão: o shader apaga do sprite a corda já queimada | < 1 s | nenhum; 22 declarações | 0 úteis / 0 falsos | sim — três momentos do pavio comparados em captura, porque o portão não vê fragmento | shader não entra no inventário de animação: a corda encurta por `set_shader_parameter`, não por Tween |
| 4 | 2026-08-24 | `f1f4d5f` | porte BomberBoom (Godot) | halo pulsante na gema de lvl 3 (D-034), fogo reduzido a meia célula, power-up acima da gema | < 1 s | nenhum; 24 declarações | 0 úteis / 0 falsos | sim — o halo herdou a escala do pai e sumiu atrás da gema; só a captura mostrou | nenhuma |
| 5 | 2026-08-24 | `f1f4d5f` | porte BomberBoom (Godot) | `_encenar` dividido em três e os enfeites da peça extraídos para `EnfeitesDaPeca` | < 1 s | nenhum; 24 declarações, as mesmas de antes | 0 úteis / 0 falsos | não — as declarações ficaram idênticas às do uso 4, e a suíte ganhou 5 casos que cobram o halo por portão em vez de olho | nenhuma; é o caso em que o inventário idêntico **é** a prova da refatoração |
| 6 | 2026-08-25 | `f1f4d5f` | porte BomberBoom (Godot) | barra de fúria: de `value = partida.furia`, um estado sem animação, para os TRÊS do original animados por `Tween` | < 1 s | nenhum; 25 declarações | **1 útil** / 0 falsos | sim — só a captura distingue barra parada de barra animando | nenhuma. O `SAR-PARSE-001` acusou `^"value"` como propriedade dinâmica que não consegue provar; trocado por `"value"` textual, o aviso sumiu. **A remediação sugerida estava certa** |
| 7 | 2026-08-25 | `f1f4d5f` | porte BomberBoom (Godot) | o ESTOURO ao armar: escala 1,10 × 3,4 com volta `OUTELASTIC` de 0,70 s, mais o clarão do trilho | < 1 s | nenhum; 26 declarações | 0 úteis / 0 falsos | sim, e foi decisiva — o defeito era um `Container` zerando o `scale` do filho, e só a sonda quadro a quadro o viu | posse de `scale` entre `Container` e `Tween`: o container reescreve a propriedade a cada layout, e a Sara não o modela como dono |
| 8 | 2026-08-25 | `f1f4d5f` | porte BomberBoom (Godot) | console de testes: cena própria que hospeda `main/tabuleiro.tscn` e abre um painel no **F1** | < 1 s | nenhum; 27 declarações | 0 úteis / 0 falsos | sim — a sonda `conferir_console.gd` pegou o painel mostrando a ficha de uma posição com o tabuleiro de outra | **tecla não é canal de entrada**: o `_unhandled_input[InputEventKey]` do F1 não virou declaração nenhuma. Sem risco aqui, mas um projeto de teclado ficaria sem cobertura de entrada e sem aviso |
| 9 | 2026-08-25 | `f1f4d5f` | porte BomberBoom (Godot) | a piscada do AUXILIAR (D-041): `DicasDoAuxiliar` desenha as conexões e anima `modulate:a` uma vez por assentamento | < 1 s | nenhum; 29 declarações | 0 úteis / 0 falsos | sim — só a captura mostra se a barra translúcida deixa a gema legível | nenhuma |
| 10 | 2026-08-25 | `f1f4d5f` | porte BomberBoom (Godot) | a LOJINHA do meta (D-023): cartão opcional que solta ao ser tocado de novo, e a marcação passa a sair de `set_meta` | < 1 s | nenhum; 29 declarações | 0 úteis / 0 falsos | sim — e a prova de cena pegou o que nenhum teste viu: o JSON do Godot devolve todo número como **float**, e o save real zerava o progresso | posse de estado de UI entre `Button.button_pressed` e a seção que é a fonte da verdade: a Sara modela animação, não seleção |
| 11 | 2026-08-25 | `f1f4d5f` | porte BomberBoom (Godot) | a CERIMÔNIA DA GARRAFA (D-009): um `Tween` encadeia escorrer → encher → estalar → sobrar, uma etapa por fragmento | < 1 s | nenhum; 31 declarações | 0 úteis / 0 falsos | sim, e decidiu duas coisas: o anel sumia sobre a gema dourada, e só a captura mostra o compasso da barra | encadeamento de `tween_callback().bind()` com `tween_property` no mesmo Tween: a Sara vê a propriedade, não a sequência |
| 12 | 2026-08-26 | `f1f4d5f` | porte BomberBoom (Godot) | brasas do estouro com `use_fixed_seed`/`seed` derivado da célula, e o portão de cena passando a exigir `--fixed-fps 60` | < 1 s | nenhum; 27 claims em 102 arquivos | 0 úteis / 0 falsos | sim, e ela **é** a medida: três rodadas do portão comparadas byte a byte, de 9 capturas instáveis para 0 | semente de `GPUParticles2D` não entra no inventário: a partícula anda na GPU, não por Tween, e a Sara não modela o relógio da rodada como dono |
| 13 | 2026-08-28 | `f1f4d5f` | porte BomberBoom (Godot) | a aranha que rouba a bomba (D-046/D-047): `RoboNaTela` vira subclasse de `BichoNaTela`, mais `AranhaNaTela`, `NinhoNaTela`, as duas entradas, e a teia que lentifica o pavio | < 1 s | nenhum; 302 claims em 1142 arquivos | 0 úteis / 0 falsos | sim, e ela decidiu o caso: dos nove defeitos, portão pegou um e o autor achou dois — jogando | três, e as três dentro da família que a Sara já modela: `pause()`/`play()`, `set_speed_scale` e profundidade por ordem de filho. Ver [CASO-DA-ARANHA.md](CASO-DA-ARANHA.md) |

O Sara pode permanecer privado ao final. Nova etapa pública exige nova decisão;
não é continuação automática deste registro.

## O uso 2 e sua procedência

O uso 2 não veio do trabalho normal: veio de uma regra que o próprio Sara acabara de
ganhar. Registrado assim de propósito, porque a diferença importa para julgar o marco.

A sequência foi: a integração do porte mostrou zero declaração de entrada contra sete
do original em Defold; a investigação achou a causa — o adapter Godot exigia mapa de
ações, e o porte despacha `InputEvent` cru; a [ADR 0010](decisoes/0010-canal-fisico-de-entrada-sem-mapa-de-acoes.md)
fez a regra enxergar canal por classe de evento; e a regra nova, na primeira execução,
achou em `main/tabuleiro.gd` toque e mouse caindo no mesmo `_dedo`.

**É o mesmo defeito que o original em Defold tinha**, e que está no `estudo/registros/defold.md`
como uma das duas regressões históricas do Portão 0: "Todo toque era entregue duas vezes
no Android", duas bombas por toque, invisível no desktop onde o Estágio A verificou tudo.
O porte reproduziu o defeito em outra engine, e reproduziu junto o ponto cego que o
escondia.

Não dava bomba dupla hoje: `_dedo` zera `_carregando_em` antes de `jogar`, e a guarda
`if alvo != Grade.FORA_DA_GRADE` engolia a segunda passada. Correto por propriedade da
máquina de estado, não por exclusão de canal. Corrigido com uma linha no `project.godot`,
porque o `if/elif` do `tabuleiro.gd` já tratava os dois canais e não precisava que o
motor fabricasse o segundo.

Uma fitness function que acha defeito no minuto em que entra apareceu três vezes neste
projeto: a F5 com as extensões, a F7 com as construções, e agora a ADR 0010 com o canal
físico. É o argumento mais forte a favor de escrever a regra antes de precisar dela.

## O uso 13, e as três capacidades que ele nomeou

A leitura inteira do caso está em [`CASO-DA-ARANHA.md`](CASO-DA-ARANHA.md). Aqui fica só o
que pertence ao marco.

**A Sara não achou nada, e não é silêncio de rotina.** Saída 0, 302 claims em 1142
arquivos, zero aviso. O caso teve nove defeitos, e a distribuição é o dado:

| quem achou | quantos |
|---|---:|
| portão (o de cena, um erro de parse) | 1 |
| instrumento escrito na hora e olhado por olho humano | 3 |
| teste da própria sessão, ou raciocínio ao escrever | 3 |
| **o autor, jogando** | **2** |

Os dois do autor eram a peça inteira: o mecanismo nunca se completava, em condição nenhuma,
e os nós ficavam presos na tela para sempre. Os outros sete eram engrenagem.

**As três capacidades ausentes são de animação, que é a família que a Sara já modela** — e
por isso valem mais que hipótese sobre eixo novo. Foram nomeadas pelo diário do próprio
porte, nas linhas 11, 12 e 13 de `.sara/USOS.md`:

1. **`pause()`/`play()` num Tween não entra no inventário.** A Sara vê a propriedade e o
   dono, não o fato de o relógio ter parado. Uma bomba presa para sempre, com o tween
   pausado, é indistinguível de uma que está queimando.
2. **`set_speed_scale` também não.** A Sara modela quem *anima* uma propriedade, não o
   relógio com que ela anima. E aqui isso não é detalhe: *"um pavio que passou a queimar na
   metade da velocidade é indistinguível de um que não passou — e essa é a regra inteira
   da peça."*
3. **Profundidade por ordem de filho.** `z_index` é relativo ao pai e `move_child` decide
   quem desenha na frente de quem; nenhum dos dois vira declaração. **Um sprite invisível
   passa por todos os portões**, e foi o que aconteceu com o fio de seda.

As três são estáticas, são de Godot, e cabem no recorte da
[ADR 0001](decisoes/0001-validar-mecanismos-antes-da-engine-completa.md). Nenhuma foi
confrontada com o corpus, então nenhuma está decidida: a
[ADR 0012 §3](decisoes/0012-sara-e-corpus-coevoluem.md) exige o confronto antes da
incorporação.

### De onde vieram os usos 3 a 12 — a transcrição, e o que ela custou de julgamento

Estas dez linhas não são trabalho novo: são mudanças reais de 24 a 26 de agosto que
estavam registradas só no diário do porte, em `.sara/USOS.md`, e nunca subiram para cá.
Pelo critério escrito no topo — *"uma linha para cada mudança em que animação ou entrada
possa ter mudado"* — elas sempre qualificaram. O proprietário decidiu em 28/08 que contam,
e a transcrição é esta.

**Três coisas precisaram de julgamento, e ficam declaradas em vez de escondidas:**

1. **O diário do porte tem 16 linhas físicas, não 13.** Duas sessões paralelas escreveram
   na mesma tabela e a numeração colidiu: há dois usos numerados 8, dois numerados 9, e as
   *brasas do estouro* aparecem **duas vezes, com texto idêntico**, numeradas 9 e 8.
   Contadas as distintas, são 15. Menos os usos 1 e 2, já registrados, e menos as três
   linhas da aranha, sobram exatamente dez.
2. **A aranha entrou como um uso, e o porte a registrou como três.** As linhas 11, 12 e 13
   de lá são três commits da mesma peça, na mesma sessão, sob um carimbo só. Contá-las
   1:1 como as outras daria 15 em vez de 13. **Escolhi o número menor porque ele é o que
   arrisca menos**: a contagem decide um portão, e nas duas leituras o portão está
   passado, então a ambiguidade não muda o desfecho — só a estatística. Se o proprietário
   preferir o 1:1, são duas linhas a acrescentar e nenhuma conclusão a rever.
3. **A ordem dentro de 25/08 segue a ordem do arquivo do porte**, não um carimbo de tempo.
   Seis mudanças caem no mesmo dia e o diário de lá não registra a hora.

**O que a transcrição não faz:** não recalcula nenhuma medição, não revê nenhuma
classificação de aviso, e não toca na previsão datada de 25/08 mais abaixo. Ela move para
cá o que já estava medido lá.

## Baselines que não contam como mudança

| Data | Projeto | Arquivos | Declarações | Erros | Avisos | Motivo de não contar |
|---|---|---:|---:|---:|---:|---|
| 2026-08-23 | porte BomberBoom (Godot) | 69 | 5 | 0 | 0 | fotografia posterior ao uso 1; não é uma segunda mudança |
| 2026-08-23 | BomberBoom (Defold) | 27 | 76 | 0 | 19 | integração do segundo projeto, sem mudança de jogo associada |
| 2026-08-23 | BomberBoom (Defold), após calibração | 27 | 76 | 0 | 12 | refinamento da mesma baseline; não é mudança de jogo |
| 2026-08-23 | Gods (Godot) | 450 | 67 | 0 | 1 -> 0 | integração do terceiro projeto Godot, sem mudança de jogo associada; o aviso era falso e virou a ADR 0009 |
| 2026-08-23 | Boomlitude (Godot) | 97 | 6 | 0 | 0 | integração do quarto projeto Godot, sem mudança de jogo associada |
| 2026-08-24 | Gods (Godot), após a ADR 0010 | 450 | 71 | 0 | 0 | rebaseline: o eixo de entrada passou a enxergar 4 declarações que eram invisíveis |
| 2026-08-24 | Boomlitude (Godot), após a ADR 0010 | 97 | 12 | 0 | 0 | rebaseline: 6 declarações de entrada invisíveis; o inventário dobrou |
| 2026-08-24 | porte BomberBoom (Godot), após a ADR 0010 | 76 | 22 | 0 | 0 | rebaseline no commit `303c061`; ver a atribuição abaixo |

## O que a ADR 0010 revelou nos projetos parados

A regra de canal físico foi escrita por causa do porte, mas o rebaseline mostrou que o
ponto cego não era só dele: o Gods tinha 4 declarações de entrada invisíveis e o
Boomlitude tinha 6 — nele, metade do inventário. Nenhuma produziu diagnóstico. O olho
abriu e não achou defeito nos parados, o que é o resultado que se quer de uma regra
nova: ela precisa enxergar mais sem passar a reclamar mais.

**O MineBoom não foi integrado, e isso é deliberado.** Ele não tem `.sara/` nem
`sara.toml`. Integrar só serve para projeto que vai gerar mudança real, e ele está
parado; como corpus de falso positivo ele já é lido direto pela biblioteca em
`tests/corpus.rs`, sem precisar de contrato instalado. Fica registrado para não parecer
esquecimento.

## Rebaseline do porte: separando o que é o jogo do que é a ferramenta

O porte saiu de 69 arquivos e 5 declarações para 76 e 22. O salto parece grande e é
tentador creditá-lo à ferramenta, mas as duas causas são distintas e misturá-las
corromperia a medição do marco.

**As 20 de animação são o jogo crescendo.** A baseline anterior foi tirada em
`70df43f`, às 16:48 de 23/08, no próprio commit do uso 1 — ou seja, já incluía a
correção da cadeia fluente do `tween_property`. Quatro arquivos entraram depois dela:
`explosao_na_tela.gd` (17:32, 2 declarações), `robo_na_tela.gd` (18:11, 3),
`reacao_da_peca.gd` e `powerup_na_tela.gd` (19:24, 6 e 5). Dezesseis das vinte vêm daí.
O Sara não passou a ver mais animação; passou a haver mais animação.

**As 2 de entrada são a ferramenta.** Elas não existiam até a
[ADR 0010](decisoes/0010-canal-fisico-de-entrada-sem-mapa-de-acoes.md): antes dela o eixo
de entrada exigia mapa de ações declarado no `project.godot`, e o porte despacha
`InputEvent` cru. A entrada do projeto inteiro era invisível, e o que estava escondido
nesse ponto cego era o defeito do uso 2.

A primeira versão desta nota atribuía o salto de animação à correção da cadeia fluente.
Estava errada, e o `git log` do porte desmentiu antes de a nota ser publicada. Fica
registrado porque a tentação é sistemática: numa medição que existe para julgar a
utilidade de uma ferramenta, todo número que cresce parece obra dela.

Medição tomada às 11:41 de 24/08, com o porte em `303c061`. O diário do próprio porte,
em `.sara/USOS.md`, é mantido pela sessão que trabalha lá.

## Baseline do Gods: o único aviso, classificado

`SAR-OWN-001` em `animation:godot:src/entities/card.gd:self:position`, apontando
`card.gd::set_elevated` (1040) e `card.gd::_on_selection_end` (1078).

**Classificação: falso.** Não há conflito em runtime. As seis animações de `position`
no arquivo seguem disciplina de dono centralizado: cada uma chama `_kill_active_tween()`
antes de criar a sua e guarda a nova em `_active_tween`, então existe um único Tween de
posição por construção. `_mouse_enter` e `_mouse_exit` ainda saem cedo quando
`_selected or _dragging or _block_hand_spring or _hand_target_active`.

**Regra ausente, em duas partes.** O adapter reconhecia cancelamento só como
`variavel.kill()` literal entre as duas linhas, e não seguia a indireção do método
auxiliar; e o aviso entre donos nunca consultava barreira nenhuma. O que decide o caso:
a remediação que o próprio `SAR-OWN-001` imprime é *"centralize o proprietário"*, e é
exatamente o que o `card.gd` faz. A ferramenta pedia o padrão que não sabia reconhecer.

**Consertado** pela [ADR 0009](decisoes/0009-baseline-em-projeto-real-expoe-regra-ausente.md),
que também ampliou a exceção da Fase 2 para baseline em projeto real. Duas trajetórias
passam a se serializar quando **as duas** encerram o mesmo alvo antes de começar,
seguindo um nível de indireção. Fixtures `godot_animation_centralized_owner_green` e
`godot_animation_uncancelled_owners_warn` — a segunda existe para reprovar uma regra
boa demais, e reprova mesmo: foi testada por mutação.

Nos cinco projetos do corpus, as declarações são idênticas antes e depois, e a única
mudança de diagnóstico é este aviso desaparecendo. Os 12 avisos do BomberBoom Defold
permanecem intactos.

Varredura de 450 arquivos em 681 ms, mediana de cinco execuções — a medição do
`RESULTADO-0.1.0.md` era 0,69 s, então a fronteira da Fase 1 não custou desempenho.

**Isto não conta como uso do Marco 6.** Baseline não é mudança real, e a ADR 0009 diz
isso explicitamente: a contagem não sobe por causa dela. O número corrente está no
cabeçalho, e só lá — repeti-lo aqui foi o que fez esta seção contradizer o topo do
arquivo por algumas horas.

## Observação: as onze pegadinhas do porte, e quantas a Sara pegaria

Não é uso do Marco 6 — não houve mudança real associada. É evidência sobre o **tamanho
do recorte** da ferramenta, colhida de `docs/PEGADINHAS.md` do porte em 25/08/2026, onde
o autor registrou onze defeitos pagos ao longo do porte.

**De onde vêm:**

| origem | quantas | quais |
|---|---:|---|
| Godot, semântica do motor | 6 | `--headless --script` trava para sempre se o script não compila; `ColorRect` engole todo toque pelo `mouse_filter` padrão; `--headless` não tem renderizador e não deixa capturar; caminho de arte como string literal que a tipagem estrita não confere; o motor reescreve o `project.godot` e apaga comentário; chamar antes do `_ready` mata todo `@onready` |
| ambiente, nenhuma engine | 3 | JDK embutido no editor fora do `PATH`; emulador com swiftshader mostrando tela cinza; `adb` mudando de porta a cada reconexão |
| ferramenta do próprio autor | 1 | o portão achava o defeito e morria antes de contar |
| **o porte, de fato** | **1** | gemas encavaladas: a cascata resolve num quadro só no domínio — divergência declarada do porte — e a cena reconstruía o compasso pela metade |

Uma em onze é atrito de porte. Fazer em Godot do zero teria poupado essa e cobrado as
outras dez igual.

**Seis das onze falham caladas.** As de número 3, 5, 6, 7, 10 e 11 trazem, escrito pelo
autor, "em silêncio", "nenhum erro", "não acusa nada" ou "sintoma: nenhum". Isso
contradiz o `RESULTADOS.md`, que concluiu que Godot falha "alto e opaco" e que quem falha
mudo é o Defold. O estudo já se protegia dessa contradição: uma execução por engine, e
"com n=1, um dia ruim do agente é indistinguível de propriedade da engine". Dias de porte
real são a amostra maior, e ela discorda.

**E a Sara não pegaria nenhuma das onze.** Nenhuma é conflito de posse: `mouse_filter` é
configuração de nó, arte errada é string literal válida, os dois relógios das gemas são
compasso de cena com um dono só. Ela vigia uma porta específica num corredor com seis
portas caladas.

As duas leituras são verdadeiras ao mesmo tempo, e o portão do Marco 6 vai ter que julgar
as duas juntas:

- **A favor da premissa.** Seis defeitos passaram por suíte verde e só apareceram quando
  uma pessoa olhou o aparelho. É exatamente a lacuna que o projeto existe para nomear.
- **Contra o escopo.** Ela cobre uma fatia estreita dessa lacuna, e a lista mostra o
  tamanho da fatia com números em vez de impressão.

**O contraponto honesto, para não pesar só de um lado:** o defeito de toque e mouse no
mesmo `_dedo` **não está nesta lista** — e a versão dele em Defold foi um defeito real
que chegou ao jogador, com duas bombas por toque. Ele não virou a pegadinha de número
doze porque a Sara o achou antes. Uma lista de pegadinhas só registra o que passou.

## O recorte é grande o bastante? — previsão registrada em 25/08/2026

> **Nota de 28/08/2026, acrescentada sem tocar no texto abaixo.** A previsão foi escrita
> quando o registro marcava duas mudanças e dizia "faltam oito". Hoje a contagem está em
> treze, e **a previsão passou a ser conferível** — que era exatamente o ponto de datá-la.
> O texto original fica intacto, porque previsão corrigida depois da evidência não prova
> nada. As onze mudanças que entraram desde então não produziram nenhum verdadeiro
> positivo bloqueante — a primeira das duas condições que a própria previsão listou como
> capazes de derrubá-la —, e produziram **um aviso útil** (uso 6, o `SAR-PARSE-001` da
> barra de fúria) e **sete capacidades ausentes nomeadas**. Quem julga o que isso faz com
> a previsão é o portão, não esta nota.

Isto **não é conclusão do marco**. O `RESULTADOS §4` proíbe pontuar antes de toda a
evidência comparável existir, e faltam oito mudanças reais. É uma **previsão datada**,
escrita agora justamente para poder ser conferida depois: previsão anotada antes da
evidência é testável; conclusão tirada cedo é contaminação. Se o portão discordar do que
está abaixo, o erro fica visível.

### A medição

A leitura crua é "zero de onze", e ela é enganosa. A pergunta certa é quantos defeitos
**só uma pessoa olhando o aparelho** podia achar, porque é essa a atenção escassa que o
projeto inteiro existe para economizar.

Das onze pegadinhas, quatro são dessa classe: a 3 (o toque não faz nada), a 5 (tela
cinza), a 6 ("quem viu foi o autor, olhando o jogo no aparelho") e a 7 ("jogando no
aparelho"). As outras sete se anunciaram sozinhas — travaram um comando, imprimiram erro,
quebraram um build.

Somando os dois defeitos históricos do BomberBoom Defold — a bomba dupla por toque e as
duas `gui.animate` que se cancelavam caladas, ambos sobreviventes de 107 asserções verdes
e de todo o Estágio A no desktop:

**Seis defeitos na classe "só olho humano acha". A Sara cobre dois.** Um terço, com
amostra de seis.

### O que puxa a favor

Os dois que ela cobre são os dois que **ninguém tinha achado**. As outras quatro custaram
tempo e foram encontradas; essas duas passaram por tudo que era automático. E o recorte
não foi escolhido no chute — foi desenhado olhando o que deu errado num jogo real.

### O que puxa contra

Quatro dos seis ela não vê, e não é imaturidade: `mouse_filter` é configuração de nó,
arte errada é string válida apontando para o arquivo errado, gemas encavaladas são dois
relógios com um dono só. Nenhum vira conflito de posse por mais que a ferramenta cresça.
Cobrir isso seria outra ferramenta, não uma versão melhor desta.

E há o silêncio dos números. Na data da ADR 0005, os 667 arquivos Godot do corpus
produziram zero erro e zero aviso. O `gods` tem 450 arquivos, 71 declarações e zero
diagnóstico. O `mineboom` tem zero declaração — sobre ele a ferramenta não tem o que
dizer.

### A previsão

**O recorte é pequeno demais para ser produto e grande o bastante para ser portão
privado.** Dois defeitos que ninguém mais achou, num jogo só, já pagaram o custo de um
binário. Não pagam uma engine, nem um lançamento, nem preço.

Se isso se confirmar, o desfecho do portão será manter privado — que é uma das três
saídas que o `RESULTADO-0.1.0.md` já autoriza, e não uma quarta inventada agora.

Convém notar, sem comemorar: é o que a ADR 0001 decidiu antes de qualquer medição, ao
mandar validar mecanismo antes de construir engine. A medição está caminhando para dar
razão a ela. Isso é boa notícia sobre o método e notícia morna sobre o produto — e as
duas coisas contam.

### O que derrubaria a previsão

- Uma das oito mudanças restantes produzir um verdadeiro positivo que teria chegado ao
  jogador. Isso mudaria a conta de dois para três num denominador que também cresce.
- A medição Android mostrar que a classe de posse é mais comum no aparelho do que o
  desktop deixa ver.
- O contrário também vale: as oito passarem sem nada útil. Aí o recorte não é pequeno
  demais para produto — é pequeno demais para portão, e o desfecho é congelar.

## Classificação da baseline Defold

- **7 falsos removidos:** laços com variáveis locais independentes, cancelamento que
  domina ramos, animação iniciada no callback de conclusão e ramo exclusivo de `init`.
- **11 úteis mantidos:** transições de ciclo de vida que substituem deliberadamente
  uma animação sem chamar `cancel_animations`; continuam avisos, nunca bloqueios.
- **1 útil mantido:** `PAVIO[n]`, alvo dinâmico limitado por tabela que ainda não pode
  ser resolvido estaticamente.
- **Taxa atual:** 0 aviso falso em 76 declarações classificadas. A amostra ainda é
  pequena e será recalculada em cada uso Defold.
