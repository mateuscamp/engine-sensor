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

Pela [ADR 0005](decisoes/0005-foco-em-godot-com-defold-congelado.md), mudança em projeto
Defold não conta para a contagem. Nenhum uso registrado até aqui é Defold — os dois são
do porte, que é Godot —, então a regra ainda não excluiu nada; ela vale para os que
vierem.

**Estado atual:** 2 de 10 mudanças; prazo aberto até 20/09/2026.

**Um único projeto em desenvolvimento ativo.** Desde 24/08/2026, o porte do BomberBoom
para Godot é o único que se mexe; Gods, Boomlitude e MineBoom ficam parados. As dez
mudanças, portanto, virão todas do porte — e ele é o melhor caso possível para isto: o
mesmo jogo existe nas duas engines, então cada regra do Sara pode ser conferida contra o
original em Defold. Foi assim que a ADR 0010 nasceu.

O risco de ter um alvo só está registrado aqui para não ser descoberto no fim: se o
porte parar, o marco para junto, e a data decide. Três projetos parados não substituem
um em movimento — eles são corpus de falso positivo, e é isso que continuam sendo.

| # | Data | Projeto | Mudança | Tempo | Conflito | Aviso útil/falso | Inspeção humana necessária | Regra ausente |
|---:|---|---|---|---:|---|---|---|---|
| 1 | 2026-08-23 | porte BomberBoom (Godot) | bomba visual com Tween configurado em cadeia fluente | < 1 s | nenhum após correção | nenhum aviso; a primeira execução omitiu 2 declarações | sim, para comparar o inventário com o diff | parser perdia `tween_property` seguido de `set_trans`/`set_ease`; fixture adicionada |
| 2 | 2026-08-23 | porte BomberBoom (Godot) | `emulate_mouse_from_touch=false`: toque e mouse caíam no mesmo `_dedo` | < 1 s | erro comprovado, corrigido | 0 falsos | não, o diagnóstico bastou | nenhuma; a regra acabara de nascer pela ADR 0010 |
| 3 | | | | | | | | |
| 4 | | | | | | | | |
| 5 | | | | | | | | |
| 6 | | | | | | | | |
| 7 | | | | | | | | |
| 8 | | | | | | | | |
| 9 | | | | | | | | |
| 10 | | | | | | | | |

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

## Classificação da baseline Defold

- **7 falsos removidos:** laços com variáveis locais independentes, cancelamento que
  domina ramos, animação iniciada no callback de conclusão e ramo exclusivo de `init`.
- **11 úteis mantidos:** transições de ciclo de vida que substituem deliberadamente
  uma animação sem chamar `cancel_animations`; continuam avisos, nunca bloqueios.
- **1 útil mantido:** `PAVIO[n]`, alvo dinâmico limitado por tabela que ainda não pode
  ser resolvido estaticamente.
- **Taxa atual:** 0 aviso falso em 76 declarações classificadas. A amostra ainda é
  pequena e será recalculada em cada uso Defold.
