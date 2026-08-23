# ADR 0001 - Validar mecanismos antes de uma engine completa

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** estratégia de produto e sequência arquitetural

## Contexto

O bakeoff mostra que agentes conseguem construir o mesmo jogo em Godot, Unity e
Defold sem direção técnica. Mostra também que as três execuções precisaram criar
observabilidade, separar regras e controlar aleatoriedade. Os defeitos mais caros
ocorreram na integração entre motor, entrada, animação e plataforma e foram
silenciosos.

Os artigos derivam desses fatos um desenho de engine AI-first chamado Passo. O
desenho inclui passo determinístico, consulta de estado e geometria, posse explícita,
perfis de plataforma, projeto textual, cadeia offline e catálogo por intenção.

Construir uma engine completa testaria todos esses mecanismos ao mesmo tempo, pelo
maior custo e com pouca capacidade de atribuir sucesso ou falha a um mecanismo.
Também não existe ainda evidência de disposição a pagar.

## Opções consideradas

1. Construir imediatamente a engine Passo completa.
2. Construir um runtime sobre uma engine hospedeira.
3. Começar por ferramentas de verificação compatíveis com engines existentes.
4. Encerrar no corpus de artigos e usar somente convenções.

## Decisão

O Sara será conduzido, até nova decisão, como um programa de experimentos. A
sequência será:

1. linter de conflitos de posse em projetos existentes;
2. protocolo de consulta e estado de verificação;
3. perfil executável de plataforma;
4. SDK/runtime sobre uma engine hospedeira;
5. somente depois, reavaliação de uma engine completa.

Convenções e scripts existentes serão a linha de base. Cada passo só recebe
investimento depois de passar seu portão técnico. A primeira entrega será interna,
para uso do proprietário por Codex e Claude Code. Publicação, licença, produto pago
e engine própria não pertencem a esta decisão.

## Justificativa

- O linter ataca os dois defeitos mais caros medidos e custa dias, não anos.
- Adoção sem migração reduz o maior atrito comercial de uma engine nova.
- Experimentos isolados preservam causalidade: sabemos qual mecanismo gerou valor.
- Reusar renderização, áudio e exportação evita competir cedo com ecossistemas
  gratuitos e maduros.
- Se a hipótese comercial falhar, a ferramenta continua útil internamente ou aberta.

## Consequências

### Positivas

- menor capital e tempo em risco;
- primeira entrega potencialmente vendável mais cedo;
- evidência compatível com os projetos que originaram a tese;
- liberdade para parar, abrir o código ou permanecer ferramenta interna;
- arquitetura completa, se vier a existir, nasce de contratos já medidos.

### Negativas

- dependência temporária dos formatos e limites de Godot/Defold;
- experiência menos coerente do que uma engine desenhada de ponta a ponta;
- risco de o produto ficar conhecido apenas como linter;
- trabalho de integração que talvez seja descartado por um runtime posterior.

## Conformidade

Enquanto esta ADR estiver aceita:

- não iniciar editor, renderizador, áudio, física geral, loja de assets ou cadeia
  multiplataforma próprios;
- toda proposta de componente de engine deve apontar para o portão que ela testa;
- cada experimento deve registrar orçamento, critério de parada e evidência;
- nova fase só começa quando a anterior tiver resultado escrito;
- exceção exige uma ADR que substitua esta decisão.

## Critério de revisão

Revisar quando ocorrer o primeiro destes eventos:

- Portão 1 aprovado;
- cinco compras reais da ferramenta inicial;
- limitação do hospedeiro impedir a fitness function principal;
- concorrência ou mudança de plataforma alterar materialmente a tese;
- seis semanas de trabalho sem evidência nova sobre necessidade ou pagamento.

## Aprovação

Aceita pelo proprietário em 23 de agosto de 2026 ao autorizar a implementação do
plano por marcos.
