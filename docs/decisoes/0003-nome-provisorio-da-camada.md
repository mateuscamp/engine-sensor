# ADR 0003 - Nome provisório da camada

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do engine-sensor
**Escopo:** identidade interna do projeto e da CLI

## Contexto

O codinome anterior não pode continuar porque “Aurora Engine” já identifica outro
projeto. A entrega atual também não é uma engine: é uma camada estática de verificação
para Godot e Defold. Manter “Engine” no nome anteciparia uma decisão arquitetural que
os portões ainda não autorizaram.

## Decisão

- O projeto e a CLI passam a se chamar **engine-sensor**.
- engine-sensor é um nome provisório enquanto o produto for a opção B, a camada de verificação.
- O repositório canônico é `mateuscamp/engine-sensor`; o sufixo do repositório não muda
  a natureza atual do produto. *(O nome desta linha mudou duas vezes depois de escrita:
  em 30/08/2026 no GitHub, pela [ADR 0016 §6](0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md),
  e em 09/09/2026 no produto inteiro, pela [ADR 0018](0018-o-nome-sai-para-a-engine-e-este-produto-se-chama-engine-sensor.md),
  que substitui esta na parte que fixa o nome. A razão não foi a natureza do produto — foi
  o aparecimento de um segundo produto que herdou o nome antigo.)*
- Como `0.1.0` é interno e não possui consumidores públicos, o comando, a pasta local,
  a configuração e os identificadores mudam integralmente para `engine-sensor`, `.engine-sensor`,
  `engine-sensor.toml` e `ESN-*`. Não haverá alias de compatibilidade para o codinome anterior.
- Se o projeto virar engine, for publicado ou receber marca comercial, nome e marca
  exigirão uma nova decisão.

## Consequências

### Positivas

- a interface não carrega um nome já ocupado;
- o nome atual não promete uma engine antes da evidência;
- a troca completa agora evita duas nomenclaturas permanentes no kit e nos projetos.

### Negativas

- a instalação pessoal e o primeiro projeto integrado precisam de migração;
- registros históricos anteriores à decisão podem mencionar o codinome abandonado.

## Fitness function

Código, testes, documentação, binário, instalação pessoal e integração ativa não
podem usar o codinome anterior, exceto ao explicar esta migração ou ao ignorar a pasta
legada durante a varredura.
