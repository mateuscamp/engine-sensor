# Padrões copiáveis AI-first

Os arquivos desta pasta são modelos pequenos, não uma biblioteca pública. Copie a
parte necessária para o projeto e dê nomes do domínio local.

## Fronteiras

- domínio recebe dados simples e devolve dados simples;
- o adapter da engine é o único lugar que traduz evento físico, nó, URL ou cena;
- relógio, RNG e efeitos externos entram por parâmetro;
- apresentação observa o resultado do domínio, mas o domínio não chama apresentação.

## RNG determinístico

Crie uma instância a partir da semente da partida e injete-a no domínio. Não misture o
gerador com `randf`, `math.random` ou outro estado global. Registre a semente na prova
de reprodução.

## Entrada normalizada

Mouse, toque, teclado e controle viram uma ação de domínio em um único adapter. O
domínio nunca recebe dois eventos físicos equivalentes nem consulta a engine.

## Animação

Um componente é proprietário de cada par alvo/propriedade. Outro fluxo pede uma
intenção a esse proprietário; não inicia um segundo Tween ou `animate` diretamente.

## Log estruturado

Emita uma linha JSON por fato verificável, com `evento`, semente/identidade e campos
de estado. Mensagem livre pode acompanhar o log para pessoas, mas não é a prova.

## Comandos reproduzíveis

Godot, ajustando a pasta pura depois de `--`:

```text
godot --headless --path . --script res://.sara/godot/portao_ai_first.gd -- game/domain
```

Defold/Lua, enumerando explicitamente os módulos puros:

```text
luajit .sara/defold/portao_ai_first.lua modules/*.lua
```

O portão estático comum é:

```text
sara check . --format json
```
