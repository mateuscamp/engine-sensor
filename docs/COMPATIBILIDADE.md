# Contrato de compatibilidade 0.1.0

## Incluído

- Linux x86_64;
- Godot 4.7 com GDScript e `project.godot` textuais;
- Defold 1.13 com Lua 5.1/LuaJIT, `.script`, `.gui_script`, `.lua` e
  `game.input_binding`;
- `Tween.tween_property`, `go.animate`, `gui.animate`, cancelamento explícito e
  os caminhos de entrada descritos na documentação das duas engines.

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
