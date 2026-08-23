# ADR 0002 - Lançamento interno e foco adaptativo de engine

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** primeira versão do verificador

## Contexto

O proprietário quer o Sara para os próprios jogos antes de avaliar qualquer
produto público. Godot fecha hoje uma parte maior do ciclo de cena sem atenção
humana; Defold, porém, contém os dois defeitos históricos que dão ao linter seu
teste causal mais forte.

Codex e Claude Code serão os consumidores imediatos. Os dois precisam encontrar o
mesmo contrato no projeto sem manter duas cópias divergentes das regras.

## Decisão

- Sara é o nome provisório da camada e `0.1.0` é um lançamento interno.
- A versão inicial verifica somente posse de animação e de entrada.
- Conflito comprovado bloqueia; ambiguidade produz aviso.
- O núcleo é independente de engine. Defold prova as regras primeiro e Godot é a
  hipótese inicial de foco, sujeita à matriz de decisão registrada no roteiro.
- `.sara/CONTRATO.md` é a fonte canônica para agentes. `AGENTS.md` e `CLAUDE.md`
  recebem fragmentos curtos que apontam para ela e para `sara check`.
- `sara init` cria material copiável, mas nunca sobrescreve arquivos de instrução
  existentes.

## Consequências

### Positivas

- o resultado continua valioso mesmo sem lançamento público;
- Codex e Claude usam o mesmo portão e o mesmo vocabulário;
- a engine prioritária pode mudar por evidência sem reescrever o núcleo;
- escopo estreito permite atribuir acerto ou falha às duas regras testadas.

### Negativas

- o adapter Defold recebe trabalho antes da engine hoje favorita;
- a análise estática precisa declarar quando não consegue provar sobreposição;
- duas integrações de agente ainda precisam ser instaladas em cada projeto.

## Conformidade

- nenhum serviço remoto, telemetria ou atualização automática;
- nenhuma edição automática de `AGENTS.md` ou `CLAUDE.md`;
- nenhuma expansão para determinismo, consulta, runtime ou monetização sem nova ADR;
- mudança de engine prioritária exige atualizar a matriz com evidência reproduzível.
