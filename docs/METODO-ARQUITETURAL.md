# Método arquitetural do Sara

**Versão:** 0.1
**Data:** 23 de agosto de 2026
**Escopo:** decidir se existe produto, qual produto existe e somente então qual
arquitetura ele merece.

## 1. O que é requisito e o que é referência

Este projeto mantém quatro tipos de informação separados:

1. **Objetivo do proprietário** - decidir a necessidade, a viabilidade técnica e a
   viabilidade econômica de uma ferramenta ou engine de jogos AI-first; testar o
   itch.io como primeiro canal; considerar outros modelos de receita, uso próprio e
   código aberto.
2. **Evidência local** - o estudo comparativo, seus registros e os três artigos deste
   repositório. Eles descrevem problemas observados e hipóteses de mecanismo.
3. **Referências arquiteturais** - *Fundamentals of Software Architecture* e
   *Software Architecture: The Hard Parts*. Os livros fornecem método de decisão;
   não fornecem requisitos para o Sara e não determinam que uma engine deva ser
   construída.
4. **Evidência de mercado** - regras atuais dos canais, concorrentes, entrevistas,
   uso real, conversão e pagamentos. Ela deve ser datada porque muda.

Uma afirmação só vira requisito do produto quando puder ser rastreada ao objetivo do
proprietário ou a um experimento aceito. Uma recomendação dos livros continua sendo
uma referência, e uma hipótese dos artigos continua sendo uma hipótese, até ser
testada.

## 2. Princípios do processo

- Não buscar a arquitetura "melhor" em abstrato. Buscar a combinação menos ruim de
  trade-offs para um contexto explícito.
- Registrar o **porquê** antes do **como**.
- Traduzir objetivos de negócio em características arquiteturais mensuráveis.
- Comparar opções equivalentes e cobrir o espaço de decisão sem misturar produto,
  canal e implementação na mesma alternativa.
- Modelar cenários reais do domínio antes de atribuir notas.
- Adiar decisões irreversíveis até o último momento responsável, sem usar isso como
  desculpa para paralisia.
- Trocar regras lembradas por mecanismos executáveis quando o custo se justificar.
- Tratar prova de utilidade técnica e prova de disposição a pagar como experimentos
  diferentes.
- Preservar decisões substituídas. Documento histórico recebe status; não é apagado.

## 3. Ciclo de decisão

Cada decisão arquitetural relevante percorre este ciclo:

1. **Contexto e motor de negócio** - quem sofre o problema, qual resultado importa e
   qual restrição não pode ser ignorada.
2. **Opções** - alternativas mutuamente comparáveis, incluindo "não construir" e
   "usar ferramenta existente".
3. **Características prioritárias** - no máximo sete realmente determinantes. Tentar
   maximizar todas dilui a decisão.
4. **Dimensões entrelaçadas** - identificar o que muda junto. Exemplo: mais
   determinismo pode custar desempenho; mais controle pode custar portabilidade.
5. **Cenários** - exercitar as alternativas em casos representativos, sobretudo nos
   casos que mais tensionam o desenho.
6. **Trade-offs** - resumir vantagens, desvantagens e consequências no contexto do
   Sara.
7. **Experimento refutável** - menor protótipo capaz de derrubar a hipótese.
8. **Fitness functions** - transformar a característica em uma verificação objetiva,
   automática quando possível.
9. **Risco** - impacto primeiro, probabilidade depois; pontuação de 1 a 9 e mitigação
   com custo explícito.
10. **ADR** - registrar contexto, decisão, consequências e como conferir
    conformidade.

## 4. Artefatos vivos

| Artefato | Pergunta que responde | Quando muda |
|---|---|---|
| `RESULTADOS.md` | O que o bakeoff mediu? | Só com nova medição comparável |
| `docs/DIAGNOSTICO-INICIAL.md` | O que a evidência permite concluir hoje? | A cada portão de decisão |
| `docs/decisoes/` | Por que escolhemos este caminho? | Nova ADR; a anterior é substituída, não reescrita |
| Registro de riscos | O que pode invalidar o projeto? | Quando impacto, probabilidade ou mitigação mudam |
| Fitness functions | A arquitetura ainda preserva o que importa? | Continuamente |
| Evidência econômica | Alguém usa, volta e paga? | Por coorte e por canal |

## 5. Regra de leveza

O próprio corpus do Sara mostra que especificação pesada não protege contra uma
decisão de produto errada. Por isso:

- uma ADR deve tratar uma decisão e caber, em geral, em uma ou duas páginas;
- um experimento deve ter uma hipótese principal e um critério de parada;
- informação operacional pertence ao teste ou ao código, não a uma segunda fonte de
  verdade em prosa;
- documento deixa claro quando descreve estado atual, proposta ou histórico.

## 6. Critério para chamar algo de "AI-first"

Neste projeto, AI-first não significa colocar um modelo dentro da engine. Significa
que um agente de código consegue:

1. descobrir capacidades por texto;
2. alterar o projeto sem depender de um editor visual;
3. executar o comportamento relevante;
4. observar estado, geometria e efeitos;
5. reproduzir um defeito;
6. obter uma falha explícita quando viola um contrato;
7. verificar o resultado sem consumir atenção humana, salvo nos critérios
   deliberadamente humanos, como qualidade artística e diversão.

Se uma solução não melhora esse ciclo completo, ela pode usar IA, mas não resolve o
problema investigado pelo Sara.

## 7. Fontes metodológicas

- Mark Richards e Neal Ford, *Fundamentals of Software Architecture: An
  Engineering Approach*. Em especial: pensamento arquitetural e trade-offs;
  identificação, medição e governo de características; fitness functions; ADRs; e
  análise de risco.
- Neal Ford, Mark Richards, Pramod Sadalage e Zhamak Dehghani, *Software
  Architecture: The Hard Parts*. Em especial: ausência de "melhores práticas"
  universais; análise de acoplamentos entre dimensões; listas MECE; contexto;
  modelagem de cenários do domínio; e síntese do resultado decisório.
