# Contrato de compatibilidade 0.1.0

## Incluído

- Linux x86_64;
- Godot 4.7 com GDScript em arquivos `.gd` e `project.godot` textuais;
- Defold 1.13 com Lua 5.1/LuaJIT em arquivos `.lua`, `.script`, `.gui_script` e
  `.render_script`, mais `game.input_binding`;
- as construções da tabela abaixo, e nenhuma outra.

## Construções reconhecidas

Toda linha desta tabela é declarada por um adapter em `src/adapters/`, e toda
construção declarada por um adapter aparece aqui. `tests/governanca.rs` confere os dois
sentidos: o documento não promete o que o código não faz, nem cala o que ele faz.

| Engine | Eixo | Construção |
|---|---|---|
| Godot | animação | `tween_property` |
| Godot | animação | `kill` |
| Godot | animação | `pause` |
| Godot | animação | `play` |
| Godot | animação | `stop` |
| Godot | animação | `set_speed_scale` |
| Godot | entrada | `_input` |
| Godot | entrada | `_unhandled_input` |
| Godot | entrada | `_gui_input` |
| Godot | entrada | `set_input_as_handled` |
| Godot | entrada | `InputEventScreenTouch` |
| Godot | entrada | `InputEventScreenDrag` |
| Godot | entrada | `InputEventMouseButton` |
| Godot | entrada | `InputEventMouseMotion` |
| Godot | entrada | `emulate_mouse_from_touch` |
| Defold | animação | `go.animate` |
| Defold | animação | `gui.animate` |
| Defold | animação | `go.cancel_animations` |
| Defold | animação | `gui.cancel_animations` |
| Defold | entrada | `on_input` |

A entrada em Godot é reconhecida por dois caminhos independentes: por ação declarada
na seção `[input]` do `project.godot`, e — desde a ADR 0010 — pela classe do evento
testada no ramo, que é o que permite enxergar jogo de toque sem mapa de ações. Toque e
mouse chegando ao mesmo efeito é conflito no perfil android, porque lá um toque entrega
os dois eventos; `pointing/emulate_mouse_from_touch=false` desfaz isso e é a única saída
provável pelo texto.

O adapter Godot reconhece `tween_property` tanto encadeado quanto isolado, e emite a
operação com o nome público `Tween.tween_property`. O cancelamento é reconhecido tanto
por `alvo.kill()` direto quanto por chamada a um método auxiliar que encerra — um nível
de indireção, que é o padrão de dono centralizado (ADR 0009). Duas trajetórias só são
consideradas serializadas quando **as duas** encerram o mesmo alvo antes de começar. Pela ADR 0005 a lista do Defold
está congelada: ela não cresce enquanto o foco for Godot.

## Fora do contrato

- C#, GDExtension, extensões nativas Defold e código gerado;
- animações alteradas em runtime por reflexão ou nomes inteiramente dinâmicos;
- macOS, Windows e distribuição pública;
- qualquer inferência silenciosa quando o parser não entende um arquivo.

Construção relevante que não possa ser resolvida recebe `SAR-PARSE-001`. Erro de
sintaxe ou árvore incompleta encerra a execução com código 2.

Identificadores Unicode válidos do GDScript são normalizados apenas durante o parsing
porque a gramática comunitária aceita identificadores ASCII. A substituição preserva
o tamanho em bytes; extração e diagnósticos continuam usando o fonte original.
