# ADR 0006 - Contrato estrito de relatório e códigos de saída

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** interface consumida por agentes

## Contexto

`sara check --format json` produz um relatório com `schema_version` e os códigos de
saída 0, 1 e 2 têm significado definido. Codex e Claude Code consomem os dois através
de `.sara/CONTRATO.md` e dos fragmentos instalados em cada projeto. Na prática isso já
é um contrato público, com consumidores ativos em dois projetos.

Não havia decisão escrita. Sem ela não existe regra de quebra: qualquer campo podia
sumir, mudar de nome ou de tipo sem que nada avisasse, e o número da versão era um
campo sem política.

Contratos frouxos toleram evolução e adiam a descoberta do erro para o consumidor.
Contratos estritos quebram alto e cedo e custam evolução. Quando o consumidor é um
agente, a alternativa à quebra alta é o agente adivinhar, e adivinhação silenciosa é
exatamente a classe de defeito que o Sara existe para encontrar.

## Opções consideradas

1. Contrato frouxo: campo novo é compatível, o consumidor ignora o que não conhece.
2. Contrato semântico: só remoção, renomeação ou mudança de tipo sobem a versão.
3. Contrato estrito: qualquer mudança de forma sobe a versão.
4. Não decidir, mantendo o estado atual.

## Decisão

Nós adotaremos a opção 3.

- O relatório JSON e os códigos de saída formam um contrato **estrito**.
- **Qualquer mudança de forma sobe `REPORT_SCHEMA_VERSION`**: campo acrescentado,
  removido, renomeado, com tipo diferente, ou que mude de obrigatório para opcional.
  Acrescentar campo não é compatível neste contrato.
- Mudança de **valor** dentro da forma existente — uma explicação melhor, uma regra
  nova produzindo diagnóstico do mesmo formato — não sobe a versão.
- Os códigos de saída são fixos: `0` nenhum conflito comprovado, `1` conflito
  comprovado, `2` a ferramenta não conseguiu provar que analisou o projeto inteiro.
  Nenhum outro código pode ser introduzido sem ADR que substitua esta.
- O consumidor pode confiar na forma exata. É esse o ponto: agente que não precisa
  adivinhar não erra em silêncio.

## Consequências

### Positivas

- o agente pode validar a forma e falhar alto quando ela mudar, em vez de interpretar;
- a versão passa a significar alguma coisa, e o kit pode exigir uma versão mínima;
- a regra é simples o bastante para ser lembrada e verificada por um teste.

### Negativas

- a versão vai subir com frequência, inclusive por acréscimos inofensivos;
- cada versão nova exige atualizar o contrato distribuído pelo `init` nos projetos já
  integrados;
- um consumidor externo, se existir um dia, quebra a cada acréscimo. Aceitável enquanto
  o lançamento for interno; reavaliar se houver publicação.

## Conformidade

`tests/governanca.rs::adr_0006_forma_do_relatorio_json_esta_congelada` extrai o
conjunto de caminhos de chave do JSON produzido em duas fixtures e compara com a lista
declarada no próprio teste. Acrescentar, remover ou renomear um campo reprova com a
instrução de subir `REPORT_SCHEMA_VERSION` junto.

`tests/governanca.rs::adr_0006_codigos_de_saida_continuam_exercitados` reprova quando
algum dos três cenários de código de saída sai de `tests/cli.rs`.

## Critério de revisão

Revisar quando ocorrer o primeiro destes eventos:

- existir consumidor fora do controle do proprietário;
- a frequência de subida de versão passar a atrapalhar mais do que a previsibilidade
  ajuda;
- o Marco 7 introduzir um segundo artefato de evidência com forma própria.

## Notas

- Autor: proprietário do Sara
- Origem: achado A6 de `docs/AUDITORIA-ARQUITETURAL.md`
- Substitui: nenhuma
