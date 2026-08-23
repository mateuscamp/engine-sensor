# ADR 0008 - Gramáticas tree-sitter comunitárias, fixadas por versão exata

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** análise sintática de GDScript e Lua

## Contexto

O Sara não interpreta texto por expressão regular: ele analisa a árvore sintática.
Nem Godot nem Defold publicam uma gramática oficial para consumo externo, então as
gramáticas usadas são comunitárias, fixadas por versão exata no manifesto e no
lockfile: `tree-sitter =0.26.13`, `tree-sitter-gdscript =6.1.0`, `tree-sitter-lua
=0.5.0`.

Isso é uma decisão de framework segundo a própria caixa de decisão do kit: estrutura o
projeto inteiro e é altamente invasiva. Estava tomada de fato e não estava registrada.

A consequência já se materializou uma vez. A gramática de GDScript recusou
identificadores Unicode válidos encontrados no projeto Gods. O adapter passou a criar
uma forma ASCII de mesmo tamanho em bytes apenas para a análise, mantendo o texto
original nas localizações e diagnósticos. O defeito era da gramática; a correção teve
de ser do Sara.

## Opções consideradas

1. Escrever analisadores próprios para GDScript e Lua.
2. Usar expressões regulares em vez de árvore sintática.
3. Usar gramáticas tree-sitter comunitárias, fixadas por versão exata.
4. Vendorizar as gramáticas dentro do repositório.

## Decisão

Nós adotaremos a opção 3.

- As três dependências ficam fixadas com `=` no manifesto, e não com faixa de versão.
  Atualizar qualquer uma delas é uma mudança que precisa passar pelo corpus inteiro
  antes de ser aceita.
- **Um defeito da gramática vira um defeito do Sara.** Não há repasse de culpa: se a
  gramática não entende uma construção válida, ou o adapter contorna com uma
  transformação registrada e reversível, ou o arquivo recebe `SAR-PARSE-001`.
- **Nenhuma omissão silenciosa.** Todo arquivo relevante é aceito ou recusado
  explicitamente. Erro de sintaxe ou árvore incompleta encerra a execução com código 2.
- Contorno de defeito de gramática precisa preservar a origem: a transformação vale
  para a análise, nunca para o que é mostrado ao usuário ou ao agente.
- A opção 4 fica disponível como saída se a gramática for abandonada ou divergir do
  GDScript real; ela não é o estado inicial porque duplicaria manutenção sem
  necessidade demonstrada.

## Consequências

### Positivas

- análise estrutural em vez de textual, que é o que permite distinguir sequência de
  concorrência;
- custo de dias em vez de meses, compatível com o orçamento do Portão 0;
- versão fixa impede que uma atualização silenciosa mude o resultado do verificador.

### Negativas

- o Sara herda os limites de gramáticas que não controla;
- acompanhar a evolução do GDScript depende de terceiros;
- a transformação ASCII é dívida técnica visível: ela existe para contornar um defeito
  externo e precisa ser removida se a gramática for corrigida.

## Conformidade

- `tests/governanca.rs::adr_0001_nao_admite_dependencia_fora_da_lista_autorizada`
  garante que nenhuma gramática nova entra sem decisão.
- `tools/check_corpus.sh` executa os cinco projetos reais: todo arquivo relevante
  precisa ser aceito ou recusado explicitamente, sem omissão.
- Fixação por versão exata é verificável no `Cargo.toml` e no `Cargo.lock`, ambos
  versionados.

## Critério de revisão

Revisar quando ocorrer o primeiro destes eventos:

- um corpus novo apresentar omissão silenciosa;
- a gramática de GDScript ficar sem manutenção por um ciclo de versão do Godot;
- a transformação ASCII deixar de ser necessária, o que permite removê-la;
- Godot publicar gramática oficial consumível.

## Notas

- Autor: proprietário do Sara
- Origem: seção 5 do plano em `docs/AUDITORIA-ARQUITETURAL.md`
- Substitui: nenhuma
