# ADR 0010 - Canal físico de entrada em Godot, sem exigir mapa de ações

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** regra de posse de entrada no adapter Godot

## Contexto

O `ROTEIRO.md` define o que o Sara bloqueia: dois donos para uma propriedade animada,
ou **dois canais físicos distintos chegando ao mesmo efeito de entrada**. A segunda
metade existia só no adapter Defold.

O adapter Godot casava `is_action_pressed("nome")` cruzado com a seção `[input]` do
`project.godot`. No Defold isso é seguro: o `game.input_binding` é obrigatório, então
sempre há um nome de ação para ancorar o canal. No Godot o mapa de ações é opcional, e
jogo de toque frequentemente não o usa — despacha `InputEvent` cru.

O porte do BomberBoom para Godot é assim. Medido na integração: 20 declarações de
animação e **zero de entrada**, contra 69 e 7 do original em Defold. Não por ausência
de entrada, mas porque a regra não tinha em que se agarrar. Metade da ferramenta era
cega — e desde 23/08/2026 o porte é o único projeto em desenvolvimento ativo, com os
outros três parados como corpus de regressão. A medição do Marco 6 concluiria sobre a
utilidade do Sara tendo exercitado metade dele.

E o que estava escondido nesse ponto cego era um defeito real, da mesma família que a
regressão histórica do Portão 0. Em `main/tabuleiro.gd`, `_unhandled_input` despacha
`InputEventScreenTouch` e `InputEventMouseButton` para o mesmo `_dedo`. Nada no
`project.godot` desligava `emulate_mouse_from_touch`, ninguém chamava
`set_input_as_handled`: no aparelho, um toque entregava os dois eventos.

O registro do estudo, `estudo/registros/defold.md`, descreve o mesmo defeito no
original: "Todo toque era entregue duas vezes no Android", `MOUSE_BUTTON_LEFT` e
`TOUCH_MULTI` aceitos pelo mesmo `on_input`, **duas bombas por toque**. O Estágio A
inteiro verificou toque no desktop, sob Xvfb, onde só o primeiro canal existe.

O porte reproduziu o defeito do original, em outra engine, com o mesmo ponto cego de
verificação. É a evidência mais direta que este projeto produziu de que a lacuna que
ele existe para cobrir é real.

## Opções consideradas

1. Não fazer: registrar o ponto cego no `COMPATIBILIDADE.md` e seguir.
2. Exigir que o projeto declare mapa de ações para ser verificável.
3. Derivar o canal da classe do evento testada no ramo, sem exigir mapa de ações.
4. Esperar o porte avançar e decidir com mais evidência.

A opção 2 é o Sara ditando arquitetura do jogo. Despachar `InputEvent` cru é
legítimo em Godot, e uma ferramenta de verificação que só funciona quando o projeto
adota a estrutura que ela prefere verifica pouco.

A opção 1 seria honesta e barata, mas deixaria sem medição metade da ferramenta
justamente no único projeto que vai se mexer.

## Decisão

Nós derivamos o canal físico da **classe do evento testada no ramo**, e o efeito da
**função chamada no corpo do ramo**. Sem exigir mapa de ações.

`InputEventScreenTouch` e `InputEventScreenDrag` são canal de toque;
`InputEventMouseButton` e `InputEventMouseMotion` são canal de mouse; o resto não
participa da regra. Toque e mouse chegando ao mesmo efeito é erro **no perfil
android**, e não existe no desktop — que é a mesma sensibilidade a perfil que o
adapter Defold já tinha, pelo mesmo motivo físico: no aparelho é o mesmo dedo.

`pointing/emulate_mouse_from_touch=false` no `project.godot` desfaz o conflito, e é a
única saída provável pelo texto. As outras duas saídas que o diagnóstico sugere —
consumir o evento, tratar um canal só — são reais mas não verificáveis por análise
estática, e por isso aparecem na remediação e não na regra.

## Consequências

### Positivas

- O eixo de entrada deixa de ser cego em jogo Godot de toque, que é a classe de jogo
  do único projeto ativo e do estudo inteiro.
- O Sara passa a reproduzir, em Godot, a regra que pegou a regressão histórica do
  Defold. A simetria entre os dois adapters deixa de ser parcial.
- O defeito encontrado no porte foi corrigido no mesmo dia, e a correção é uma linha
  de configuração porque o código já tratava os dois canais.

### Negativas

- A regra é sintática: ela vê a classe testada no `is`, não o fluxo real do evento.
  Despacho por variável intermediária, por `match`, ou por sinal de nó não é visto.
- Ela não prova dano, só duplicação. No porte a segunda entrega era neutralizada por
  uma guarda de estado, e o Sara bloqueou mesmo assim — porque a guarda é acidente de
  implementação e some quando alguém mexer nela sem saber. Bloquear aqui é a escolha
  do contrato, não um descuido.
- O vocabulário de eventos é fechado e cresce por decisão. Joypad, teclado e caneta
  não participam, porque não colapsam num único gesto no aparelho.
- Vale só para Godot. O Defold continua congelado pela ADR 0005.

## Conformidade

Fitness function automática, `godot_detects_touch_and_mouse_reaching_the_same_effect`
em `tests/cli.rs`, sobre três fixtures:

| fixture | perfil | espera |
|---|---|---|
| `godot_input_channel_red` | android | erro `SAR-OWN-002`, saída 1 |
| `godot_input_channel_red` | desktop | nenhum diagnóstico, saída 0 |
| `godot_input_channel_green` | android | nenhum: a emulação está desligada |
| `godot_input_channel_separate_green` | android | nenhum: canais em efeitos distintos |

As duas últimas são o que impede a regra de ser boa demais: tratar os dois canais não
é conflito, conflito é os dois caírem no mesmo efeito.

`tests/governanca.rs` fecha o vocabulário nos dois sentidos — as cinco classes de
evento e a chave de emulação estão em `CONSTRUCTS` e na tabela do
`COMPATIBILIDADE.md`, e divergir reprova.

## Critério de revisão

Se a medição Android mostrar que o Godot **não** entrega os dois eventos com a
emulação ligada, a regra está errada e sai. Essa medição continua tendo poder de veto
pelo `ROTEIRO.md`, e agora tem uma pergunta concreta para responder.

Se aparecer despacho de entrada por caminho que a regra não vê — sinal de nó, `match`,
variável intermediária — isso é regra ausente nova, e não motivo para afrouxar esta.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma
- Última alteração: 23 de agosto de 2026
