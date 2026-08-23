# ADR 0007 - `sara observe` nasce como binário separado

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** empacotamento do experimento do Marco 7

## Contexto

A propriedade mais forte e mais bem medida do `0.1.0` é ser um artefato independente:
3.836.984 bytes, código 0 em namespace Linux sem rede, sem runtime de linguagem
externo. Isso sustenta duas coisas registradas: a característica 5 do diagnóstico,
"possibilidade de posse", e o risco "ferramenta depende de rede ou runtime externo".

A [ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md) autoriza o spike de visão
instrumentada em Godot e registra corretamente a consequência: "a prova deixa de ser um
único binário operacional e passa a exigir Godot no ambiente". O registro era textual;
não havia decisão de empacotamento.

Se `observe` entrar no mesmo binário, `sara check` herda a dependência de uma
instalação de Godot. Nenhum teste atual reclamaria disso.

Aplicando os desintegradores de granularidade às duas funções:

| Desintegrador | `check` x `observe` |
|---|---|
| Escopo e função | análise estática de texto x instrumentação de cena em execução |
| Volatilidade | estável, com corpus e contrato congelado x experimental por desenho |
| Tolerância a falha | uma queda do Godot não pode derrubar o portão do agente |
| Extensibilidade | contrato estrito x interface provisória que vai mudar |

Os integradores existentes — `model`, esquema do relatório, vocabulário do CLI — pedem
biblioteca compartilhada, não binário compartilhado.

## Opções consideradas

1. Subcomando `sara observe` no mesmo binário.
2. Feature de Cargo desligada por padrão, um binário só.
3. Segundo binário `sara-observe` no mesmo workspace, compartilhando a biblioteca.
4. Adiar a decisão até depois do spike.

## Decisão

Nós adotaremos a opção 3, desde o primeiro commit do Marco 7.

- `sara-observe` é um segundo `[[bin]]` do mesmo crate, ou um segundo crate do mesmo
  workspace, compartilhando `model` e `report` pela biblioteca.
- O binário `sara` continua sendo um quantum independente: sem Godot, sem rede, sem
  runtime externo. O teste offline e a lista de dependências autorizadas aplicam-se a
  ele.
- `sara-observe` pode exigir Godot instalado. Essa exigência é dele, não do portão.
- O contrato estrito da [ADR 0006](0006-contrato-estrito-de-relatorio-e-codigos-de-saida.md)
  cobre o relatório de `sara check`. A evidência produzida por `observe` tem forma
  própria e provisória, e não entra naquele contrato enquanto o spike não terminar.

A opção 2 foi descartada porque um build de release com a feature ligada por engano
degrada a propriedade sem sinal. A opção 4 foi descartada porque decidir agora custa
quase nada e decidir depois custa reescrever empacotamento e refazer a medição offline.

## Consequências

### Positivas

- a propriedade medida do `0.1.0` fica protegida por construção, não por atenção;
- o spike pode falhar, ser descartado ou crescer sem contaminar o portão que já está
  em uso;
- a fronteira entre análise estática e instrumentação fica visível no repositório.

### Negativas

- dois binários para instalar e versionar, num projeto que hoje distribui um;
- alguma duplicação de opções de linha de comando entre os dois;
- se o `observe` for incorporado ao produto um dia, unificar custará uma decisão nova.

## Conformidade

`tests/governanca.rs::adr_0007_apenas_binarios_autorizados` reprova quando aparece um
`[[bin]]` fora da lista `sara` e `sara-observe`.

Quando `sara-observe` existir, a lista de dependências autorizadas passa a ser por
binário e o teste offline continua sendo executado contra `sara`. Enquanto ele não
existir, a lista única já garante que nenhuma dependência de instrumentação entrou.

## Critério de revisão

Revisar quando o Marco 7 terminar. Incorporar `observe` a um lançamento, oferecer
protocolo estável ou unificar os binários exige nova decisão.

## Notas

- Autor: proprietário do Sara
- Origem: achado A5 de `docs/AUDITORIA-ARQUITETURAL.md`
- Substitui: nenhuma. Complementa a ADR 0004, que permanece aceita
