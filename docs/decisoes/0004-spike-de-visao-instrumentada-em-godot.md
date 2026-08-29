# ADR 0004 - Spike de visão instrumentada em Godot

**Status:** **Substituída por [0014](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md)**
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** experimento posterior ao Marco 6
**Restringida por:** [ADR 0011](0011-marco-7-exige-comparacao-com-ferramenta-existente.md) — o
Marco 7 não começa sem uma ADR que compare este spike contra as ferramentas existentes.
Esta ADR não lista opções consideradas, e a ferramenta existente era alternativa viva.

> ## ⚠️ O spike foi CANCELADO em 28/08/2026
>
> A comparação que a ADR 0011 exigiu foi feita e concluiu por **cancelar**. Cinco das sete
> fitness functions abaixo já estavam entregues por ferramentas de terceiros, com recibo e
> hash que esta ADR nem pediu; a sexta — apontar o nó ou propriedade causal — **falhou
> quando medida**, e falhou em silêncio; e a sétima, a única que produz conhecimento, foi
> preservada como pergunta e não precisa de binário nenhum.
>
> Há ainda um defeito no critério de aceitação desta ADR que só apareceu depois: **as sete
> fitness functions têm todas forma de regressão** — algo que estava certo e ficou errado.
> Ver o [caso da aranha](../CASO-DA-ARANHA.md), que é uma peça que nunca esteve certa e
> pela qual este spike teria passado.
>
> O texto abaixo fica **intacto**, como registro do que foi decidido em 23/08 e do que a
> comparação encontrou depois. Ver a
> [ADR 0014](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md).

## Contexto

O verificador estático reduz conflitos que um agente não consegue perceber antes de
executar o jogo, mas não fecha o ciclo central de desenvolvimento: executar uma cena,
observar seu resultado e relacionar um defeito visual ao estado que o causou. Uma
captura isolada mostra sintomas, porém não distingue com segurança posição,
visibilidade, ordem de desenho, propriedade animada, estado do nó ou entrada que levou
ao quadro.

Os critérios arquiteturais do Sara já priorizam verificabilidade autônoma e
observabilidade consultável. É preciso testar se uma engine hospedeira consegue expor
essas duas características sem construir um runtime próprio.

## Decisão

Após o Marco 6, o Sara fará um spike somente em Godot para produzir uma prova de cena
instrumentada. A unidade de evidência será:

```text
imagem + estado semântico + sequência de entradas + instante + logs
```

O experimento terá uma interface provisória equivalente a:

```text
sara observe CAMINHO --scenario NOME
```

Cada execução deverá criar um diretório autocontido com manifesto, imagens PNG em
checkpoints, estado consultável da cena, rastro de entradas e log da engine. Codex ou
Claude Code serão os consumidores das evidências; o Sara não incorporará modelo de
visão, serviço remoto nem credencial de provedor.

O spike poderá executar uma instalação existente do Godot e instalar material local
de prova no projeto. Isso é uma exceção experimental ao caráter puramente estático da
versão `0.1.0`, não uma mudança retroativa no contrato desse lançamento.

Defold, Android, API pública, plugin de editor, SDK e runtime ficam fora do spike. A
prova em aparelho continua sendo um portão posterior com poder de veto.

## Cenário mínimo

- uma cena real de projeto pessoal;
- semente, viewport, perfil e sequência de entradas declarados;
- três checkpoints visuais;
- três regressões visuais injetadas e suas versões corrigidas;
- captura do nó e das propriedades relevantes em cada checkpoint;
- execução sem abrir ou operar o editor manualmente;
- comparação explícita entre evidência somente visual e evidência visual mais estado.

## Fitness functions

O spike passa somente se:

1. um agente inicia a prova com um comando e conclui a avaliação sem uma pessoa olhar
   a cena;
2. as três regressões injetadas são detectadas e as três versões corrigidas passam;
3. o diagnóstico combinado aponta o nó ou propriedade causal nas três regressões;
4. dez repetições no mesmo ambiente produzem estado final e pixels idênticos, ou uma
   fonte de variação é identificada e eliminada;
5. manifesto, imagens, estado, entradas e logs permitem reproduzir a execução sem
   arquivo local oculto;
6. a execução mediana fica abaixo de 30 segundos no cenário mínimo;
7. imagem mais estado produz diagnóstico mais preciso que imagem isolada em pelo
   menos um caso previamente definido.

Falha em determinismo, dependência de inspeção humana ou incapacidade de localizar a
causa encerra o spike sem autorizar SDK ou runtime.

## Consequências

### Positivas

- testa diretamente a maior deficiência de um agente em projetos de jogos;
- reaproveita renderização, exportação e ciclo de cena do Godot;
- mantém pixels e estado ligados por um manifesto reproduzível;
- preserva independência de Codex, Claude ou outro modelo multimodal.

### Negativas

- a prova deixa de ser um único binário operacional e passa a exigir Godot no ambiente;
- instrumentação pode alterar tempo, árvore ou comportamento da cena;
- referências visuais podem ficar frágeis diante de fonte, GPU e plataforma;
- sucesso no desktop não prova fidelidade Android nem viabilidade em Defold.

## Limites

Esta ADR autoriza somente o spike. Incorporar `sara observe` a um lançamento interno,
suportar outra engine ou aparelho e oferecer protocolo estável exige os resultados do
experimento e uma nova decisão.
