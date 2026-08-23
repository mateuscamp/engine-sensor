# Diagnóstico inicial de necessidade e viabilidade

**Status:** hipótese de investimento, não autorização para construir a engine
**Versão:** 0.1
**Data-base:** 23 de agosto de 2026

## Resposta curta

Existe evidência suficiente de uma **necessidade técnica**: agentes conseguem escrever
jogos nas engines atuais, mas ainda têm dificuldade para provar sozinhos que o jogo
completo funciona, principalmente na fronteira entre regra, cena, tempo, entrada e
plataforma.

Ainda não existe evidência suficiente de que a resposta deva ser uma **engine
completa**, nem de que haja demanda paga suficiente para sustentá-la. A primeira tese
de produto deve ser menor:

> Uma camada de verificação AI-first para engines existentes, começando pelos
> conflitos de posse e pelo ciclo executar-observar-reproduzir.

O itch.io é um canal tecnicamente adequado para vender uma ferramenta em acesso
antecipado. Ele não é, por si só, evidência de demanda. A viabilidade econômica deve
ser decidida por pagamentos e retenção, não por visualizações, estrelas ou interesse
verbal.

## 1. O que a evidência local sustenta

### O problema existe

- Godot, Unity e Defold fecharam o mesmo jogo, no mesmo aparelho, sem receber
  direção técnica. Logo, o problema principal **não** é incapacidade do agente de usar
  uma engine tradicional.
- Três execuções independentes separaram regras do motor, criaram um canal de
  narração e reimplementaram o gerador aleatório. Essa convergência aponta para
  lacunas de verificabilidade, observabilidade e determinismo.
- No Defold, 107 asserções ficaram verdes enquanto o Android entregava o mesmo toque
  por dois canais. O defeito estava na integração que os testes de regra não
  exercitavam.
- Os defeitos mais caros foram conflitos silenciosos de posse: duas animações tocando
  a mesma propriedade e dois caminhos tratando a mesma entrada.
- O porte histórico Defold-Godot e o bakeoff, feitos com meses de distância, apontaram
  para o mesmo fator: o valor econômico está no ciclo que o agente fecha sem pedir que
  uma pessoa olhe.

### O que ela não sustenta

- Uma execução por engine não estima taxa de defeito nem produtividade média.
- O estudo não compara uma engine AI-first com as engines atuais.
- Não há entrevista, usuário externo, preço testado, conversão ou retenção.
- Não foi provado que perfis declarativos reproduzem diferenças reais de plataforma.
- Não foi provado que a regra de um dono por propriedade continua tolerável em jogos
  maiores.

## 2. Sinais externos atuais

Dois sinais reforçam a existência do problema, sem provar mercado:

- O [GameCraft-Bench](https://arxiv.org/abs/2606.17861), publicado em junho de
  2026, avalia 140 tarefas em Godot e relata 41,46% para o agente mais forte. A
  avaliação enfatiza execução jogável, completude do artefato e verificação
  interativa; os modelos ainda sofrem com conteúdo suficiente, feedback funcional e
  apresentação coerente.
- O projeto aberto [Arcane](https://github.com/jsvd/arcane) se apresenta como engine
  2D code-first, test-native e agent-native, com estado consultável e lógica headless.
  Isso mostra que a categoria começou a existir e também elimina qualquer hipótese de
  que o Sara esteja sozinho. Em 23 de agosto de 2026, o repositório ainda exibia
  adoção pública inicial, com quatro estrelas e nenhuma release listada.

O primeiro sinal evidencia capacidade ainda insuficiente. O segundo evidencia
concorrência e timing. Nenhum deles mede disposição a pagar.

## 3. Motores de negócio

Em ordem de prioridade para este projeto:

1. **Economia de atenção humana** - reduzir verificações que somente o proprietário
   consegue fazer.
2. **Tempo até evidência confiável** - não apenas tempo até compilar ou executar.
3. **Posse e continuidade** - build e verificação não podem depender de serviço remoto
   indispensável.
4. **Superfície de manutenção pequena** - um projeto solo não consegue sustentar um
   ecossistema comparável ao de engines maduras.
5. **Adoção progressiva** - entregar valor sem exigir que o usuário migre o jogo antes
   de confiar na ferramenta.
6. **Receita compatível com suporte** - compradores adicionais não podem aumentar o
   trabalho humano mais rápido que a receita.
7. **Liberdade de saída** - se o mercado não pagar, o resultado deve continuar útil
   internamente ou como software aberto.

## 4. Contexto inicial do produto

**Sara** é o nome provisório da camada de verificação e do projeto de decisão enquanto
ele não for uma engine. **Passo** é o nome usado nos artigos para um desenho
arquitetural específico. Se a evolução chegar a uma engine, ou se houver publicação,
nome comercial, marca e relação entre os dois serão decididos separadamente.

O recorte inicial deve ser estreito:

- jogos 2D;
- puzzle, tático, roguelike, estratégia por turno e jogos com simulação discreta;
- agente de código como usuário primário da ferramenta;
- pessoa como decisora de produto, estética e diversão;
- Godot como primeiro hospedeiro provável, sem excluir um linter que também leia
  projetos Defold.

Mundo aberto, física contínua complexa, renderização de ponta e uma loja de assets não
fazem parte da hipótese inicial.

## 5. Opções de produto

As opções abaixo representam graus crescentes de propriedade sobre a pilha. Canal de
venda e licença são decisões separadas.

| Opção | Valor que pode provar | Principal vantagem | Principal custo ou risco | Decisão agora |
|---|---|---|---|---|
| A. Convenções e scripts nos projetos atuais | Se disciplina já resolve o problema | Quase nenhum investimento | Regra esquecível; pouco produto vendável | Controle do experimento |
| B. CLI/linter de verificação para Godot e Defold | Posse, contratos, evidência e estado verificável | Valor imediato, sem migração | APIs e formatos de duas engines | **Construir primeiro** |
| C. SDK/plugin pago sobre uma engine hospedeira | Ciclo determinístico e consultável ponta a ponta | Reusa render, áudio e exportação maduros | Limites e mudanças do hospedeiro | Só após B |
| D. Runtime AI-first com apresentação hospedeira | Se a arquitetura Passo funciona no jogo inteiro | Mais controle sem reescrever toda a plataforma | Integração complexa; dois modelos mentais | Só após portão técnico |
| E. Engine completa | Controle de toda a experiência | Diferenciação máxima | Anos de escopo, suporte multiplataforma e concorrência gratuita | **Não iniciar** |

Recomendação: manter A como linha de base, validar B, testar monetização em C e só
então reavaliar D. A opção E precisa de evidência nova; não é a continuação automática
das anteriores.

## 6. Características arquiteturais prioritárias

| Prioridade | Característica | Definição operacional inicial |
|---|---|---|
| 1 | Verificabilidade autônoma | O agente executa, observa e decide se um critério passou sem intervenção humana |
| 2 | Determinismo | Mesmo estado, entrada, semente e perfil produzem o mesmo próximo estado |
| 3 | Observabilidade consultável | Estado e geometria são consultáveis por contrato estável, não inferidos apenas de pixels ou logs livres |
| 4 | Falha explícita | Conflito de posse, nome, contrato ou referência impede a carga e nomeia os dois sítios envolvidos |
| 5 | Possibilidade de posse | Build e verificação essenciais funcionam localmente e podem ser versionados |
| 6 | Evolutividade textual | Fonte de verdade é pequena, decomponível, pesquisável e preserva proveniência |
| 7 | Fidelidade de plataforma | Diferenças semânticas conhecidas do alvo podem ser executadas antes do aparelho físico |

Desempenho, usabilidade humana, extensibilidade e portabilidade continuam
importantes, mas não podem ser maximizados às custas das sete características que
definem a tese.

## 7. Dimensões entrelaçadas e trade-offs

| Escolha que melhora | Efeito colateral provável | Pergunta que o experimento deve responder |
|---|---|---|
| Estado imutável e rebobinável | Alocação, cache e desempenho | Diário de mutações preserva a prova com custo menor? |
| Um dono por propriedade | Menos composição e mais declaração | Em projeto maior, quantos conflitos reais e falsos positivos aparecem? |
| Perfis de plataforma no desktop | Manutenção de uma simulação e falsa segurança | O perfil pega defeitos antes encontrados só no aparelho sem criar confiança indevida? |
| Projeto sem arquivos-sombra | Renomeações e referências ficam dependentes de caminho | Mudança atômica por ferramenta é suficiente em projetos longos? |
| Hospedar em Godot | Exportação e ecossistema prontos | O hospedeiro permite avançar tempo e consultar geometria sem caminhos paralelos? |
| Cadeia hermética e offline | Binário maior e atualização de segurança mais difícil | Qual conjunto mínimo precisa estar empacotado para evitar bloqueio? |
| Código aberto | Adoção, confiança e contribuição | Receita de conveniência e suporte paga a manutenção? |
| Produto fechado | Captura direta de receita | A barreira reduz adoção antes de existir confiança? |

## 8. Fitness functions iniciais

Estes limiares são propostas. Devem ser fixados antes de executar o experimento que
irão julgar.

| Característica | Verificação objetiva proposta |
|---|---|
| Posse | O linter encontra os dois conflitos históricos conhecidos; cada diagnóstico traz recurso, donos e localização |
| Precisão do linter | No corpus de validação, no máximo um falso positivo por 100 recursos declarados |
| Determinismo | 1.000 repetições da mesma sequência terminam com o mesmo hash; comparação inclui dois sistemas operacionais quando possível |
| Perfil Android | O caso de toque duplicado falha no desktop sob o perfil Android |
| Consulta | Varredura das 99 células e sua geometria termina em menos de um segundo na máquina de referência |
| Separação do núcleo | Testes de regra não importam janela, GPU, relógio ou API do renderizador |
| Variante de loja | Pacote de entrega não contém comandos, prefixos nem endpoints exclusivos de verificação |
| Continuidade | O manifesto informa quais provas continuam válidas para a revisão atual e invalida as afetadas por mudança |
| Reprodutibilidade | Clone limpo executa teste e gera pacote seguindo um comando documentado, sem arquivo local oculto |
| Autonomia | Em tarefa padronizada, o agente fecha o ciclo sem direção técnica e anexa evidência reproduzível |

## 9. Viabilidade econômica no itch.io

### O canal permite o experimento

As regras atuais do itch.io suportam produto pago, doação, preço mínimo com
pagamento acima do mínimo, arquivos com faixas de preço, acesso antecipado, pré-venda,
metas, promoções e bundles. O criador escolhe de 0% a 100% de participação para a
plataforma; o padrão mostrado na documentação é 10%. Taxas do processador são
tipicamente US$ 0,30 mais 2,9% por transação. No modo de pagamentos coletados pelo
itch.io, a plataforma atua como merchant of record e cuida de VAT e chargebacks.

Fontes: [pagamentos e repasses](https://itch.io/docs/creators/payments),
[precificação](https://itch.io/docs/creators/pricing) e
[visão geral para criadores](https://itch.io/docs/general/about).

Isso torna o itch.io bom para um primeiro teste de ferramenta independente e acesso
antecipado. A taxa fixa torna preços muito baixos ineficientes; a própria documentação
recomenda pelo menos US$ 2.

### Economia unitária ilustrativa

Antes de impostos, reembolsos e suporte, usando participação de 10% para o itch.io e
a taxa típica documentada:

`receita líquida aproximada = preço - 10% - 2,9% - US$ 0,30`

| Preço testado | Líquido aproximado | Compradores para recuperar US$ 5 mil | Para recuperar US$ 10 mil |
|---:|---:|---:|---:|
| US$ 15 | US$ 12,77 | 392 | 784 |
| US$ 29 | US$ 24,96 | 201 | 401 |
| US$ 49 | US$ 42,38 | 118 | 236 |

Os valores de US$ 5 mil e US$ 10 mil são referências de recuperação, não estimativas
do custo real. O projeto deve registrar horas, custo de agente, infraestrutura e suporte
para substituir essas referências pelo custo de oportunidade do proprietário.

### Retenção para vendedor brasileiro

Há uma variável material antes de tratar os valores acima como receita disponível. No
modo `Collected by itch.io`, vendedores fora dos Estados Unidos passam por entrevista
fiscal. A documentação do itch.io informa retenção padrão de 30% quando não há número
fiscal válido ou benefício de tratado. A
[lista vigente do IRS](https://www.irs.gov/businesses/international-businesses/united-states-income-tax-treaties-a-to-z)
não inclui o Brasil entre os países com tratado de imposto de renda em vigor.

Portanto, se o vendedor for pessoa ou entidade brasileira e usar esse modo, o plano
deve carregar **30% de retenção americana como pior caso até o onboarding mostrar a
taxa aplicável**. Isso não substitui orientação contábil sobre a natureza da receita,
tributação brasileira ou eventual compensação.

Aplicando esse pior caso apenas para mostrar a sensibilidade da conta:

| Preço testado | Líquido após taxas e retenção ilustrativa de 30% | Compradores para US$ 5 mil | Para US$ 10 mil |
|---:|---:|---:|---:|
| US$ 15 | US$ 8,94 | 560 | 1.120 |
| US$ 29 | US$ 17,47 | 287 | 573 |
| US$ 49 | US$ 29,67 | 169 | 338 |

Antes da primeira venda, é necessário concluir a entrevista fiscal, confirmar o meio
de repasse disponível e registrar a taxa exibida na conta. O modo de pagamento direto
transfere obrigações de merchant of record, VAT e chargeback ao vendedor; não deve
ser escolhido somente para evitar retenção sem revisão profissional.

### O que vender primeiro

Não vender "mais uma engine". Vender um resultado observável:

> Encontre conflitos silenciosos e produza uma prova reproduzível do seu jogo antes
> de pedir que uma pessoa rode e olhe.

Uma escada de oferta possível:

1. demonstração e diagnóstico gratuito sobre um projeto pequeno;
2. acesso antecipado pago, com licença perpétua para a versão adquirida;
3. edição profissional com integração de CI, histórico de evidência e perfis;
4. serviço de implantação ou auditoria para equipes.

US$ 29 é um bom **primeiro teste**, não uma conclusão de preço: é alto o bastante
para que o pagamento tenha sinal e baixo o bastante para uma ferramenta indie em
acesso antecipado. Outra coorte deve testar preço diferente; não se muda preço no
meio da mesma coorte.

### Outros formatos de monetização

Em provável ordem de receita inicial:

1. **Auditoria e integração pagas** - aplicar o verificador a projetos existentes. É
   pouco escalável, mas aprende diretamente com casos reais e pode financiar o
   produto.
2. **Ferramenta paga no itch.io** - licença única e acesso antecipado. Menor operação
   comercial, bom para validar disposição a pagar.
3. **Núcleo aberto + conveniência paga** - CLI e contratos abertos; relatório visual,
   integração de CI, pacotes verificados ou suporte como oferta paga.
4. **Patrocínio e doações** - complemento adequado se o núcleo virar bem público;
   não deve ser a única hipótese de sustentabilidade.
5. **Verificação hospedada** - receita recorrente por execução, histórico e matriz de
   plataformas. Só faz sentido depois de provar uso frequente; introduz justamente a
   dependência remota que o produto critica, então o modo local deve continuar
   existindo.
6. **Licença e suporte para equipes** - contrato anual, SLA, integração privada e
   perfis internos. É um mercado posterior, não a primeira versão.

Se o mercado não pagar, há duas saídas que preservam valor: ferramenta interna para
produzir os próprios jogos com menos atenção humana, ou projeto aberto que acumula
um corpus público de defeitos e melhora o ecossistema.

## 10. Portões de decisão

### Portão 0 - posse

**Orçamento:** no máximo 40 horas de implementação antes da primeira avaliação.

Passa se:

- detecta os dois defeitos históricos de posse;
- explica o conflito com localizações úteis;
- mantém o limite de falsos positivos no corpus;
- encontra pelo menos um caso novo ou evita regressão real em projeto ativo.

Se falhar, publicar a regra como aprendizado e não construir a engine.

### Portão 1 - ciclo fechado

Passa se:

- a varredura de 99 células cai de 6 min 24 s para menos de um segundo;
- o perfil Android reproduz o toque duplicado no desktop;
- três tarefas padronizadas são concluídas por agente sem direção técnica humana;
- a prova produzida por uma implementação independente concorda com o jogo.

Se posse funcionar, mas este portão falhar, o produto continua sendo linter, não
runtime.

### Portão 2 - uso externo

Passa se pelo menos cinco usuários externos aplicam a ferramenta em dez projetos,
três voltam a usá-la numa segunda semana e o suporte mediano fica abaixo de 30
minutos por usuário ativo por semana.

### Portão 3 - pagamento

Antes de ampliar o runtime:

- página clara e demonstração reproduzível;
- dez entrevistas com usuários qualificados;
- pelo menos cinco compras ou pré-compras reais na faixa testada;
- registro separado de origem, ativação, retorno, reembolso e tempo de suporte.

Interesse verbal e cadastro gratuito não substituem pagamento. Cinco vendas também
não provam escala; apenas autorizam a próxima coorte.

## 11. Registro inicial de riscos

Pontuação = impacto (1-3) x probabilidade (1-3). Tecnologia desconhecida começa em
risco alto até o experimento reduzir a incerteza.

| Risco | I | P | Nota | Mitigação imediata |
|---|---:|---:|---:|---|
| Não existir disposição a pagar | 3 | 3 | 9 | Pré-venda antes do runtime completo |
| Escopo crescer até uma engine generalista | 3 | 3 | 9 | Portões e proibição explícita de E antes de nova ADR |
| O verificador produzir prova falsa | 3 | 3 | 9 | Oráculo independente e testes do próprio verificador |
| Perfis criarem falsa confiança de plataforma | 3 | 3 | 9 | Manter aparelho no portão; publicar cobertura e lacunas do perfil |
| Suporte multiplataforma consumir o projeto solo | 3 | 3 | 9 | Um hospedeiro e um recorte de gênero no início |
| Regra de posse gerar cerimônia excessiva | 2 | 2 | 4 | Medir falsos positivos e permitir composição explícita, não silêncio |
| Dependência do hospedeiro limitar o avanço por quadro | 3 | 2 | 6 | Spike antes de desenhar SDK público |
| Concorrente aberto ocupar a categoria | 2 | 3 | 6 | Diferenciar por evidência medida e interoperabilidade, não pelo rótulo AI-first |
| Código aberto não capturar receita | 2 | 2 | 4 | Separar adoção de oferta paga; medir conversão para conveniência e suporte |
| Produto fechado bloquear adoção | 2 | 2 | 4 | Demo funcional, licença de avaliação e experimento de open core |
| Nome comercial conflitar ou confundir Sara/Passo | 2 | 2 | 4 | Pesquisa de marca e decisão separada antes de publicar |
| Retenção e tributação tornarem o preço inviável no Brasil | 3 | 2 | 6 | Onboarding fiscal antes da pré-venda; modelar preço líquido com contador |

## 12. Decisão provisória

O projeto deve avançar como **descoberta de produto baseada em ferramentas de
verificação**, não como construção de engine completa. O primeiro investimento é o
linter de posse. O primeiro candidato pago é a camada de verificação/SDK sobre uma
engine existente. O runtime Passo permanece uma hipótese condicionada.

Resultados possíveis são todos válidos:

- **técnica e comercialmente viável:** produto pago, inicialmente no itch.io;
- **útil, mas sem mercado suficiente:** ferramenta interna;
- **útil para muitos, difícil de capturar receita:** núcleo aberto com serviços e
  patrocínio;
- **hipótese técnica refutada:** encerrar sem carregar o custo de uma engine.

A decisão está formalizada como proposta em
[`decisoes/0001-validar-mecanismos-antes-da-engine-completa.md`](decisoes/0001-validar-mecanismos-antes-da-engine-completa.md).
