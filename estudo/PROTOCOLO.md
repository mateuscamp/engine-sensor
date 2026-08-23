# Protocolo do experimento: engines para desenvolvimento AI-first

**Versão:** 2.0 · **Substitui:** `PROTOCOLO-v1.md`
**Objeto do experimento:** Defold, Godot e Unity
**Produto comum:** jogo mobile de tabuleiro em grade, onde o jogador dispara
explosões que detonam grupos conexos do mesmo tipo, com reação em cadeia, queda
e reposição, progressão em runs e monetização. A premissa completa está em
`shared/PREMISSA.md`.

## 1. Objetivo

Comparar a experiência completa de um agente de IA ao desenvolver o mesmo produto
nas três engines, desde um projeto vazio até uma versão executável, verificável e
empacotada para mobile.

O experimento deve medir o que acontece na prática, incluindo:

- quanto o agente entende naturalmente de cada engine;
- quanto consegue operar, observar e verificar sem intervenção humana;
- quanto depende de orientação específica sobre recursos da engine;
- quanto atrito existe na preparação, execução, depuração e geração do build;
- quão acessíveis são os recursos necessários para um jogo mobile monetizado;
- quanto custa, em tempo e em tokens, chegar ao mesmo resultado.

Este protocolo não escolhe uma engine e não presume que permanecer no Defold ou
migrar seja a resposta correta.

## 2. Princípios

### 2.1 Mesmo objetivo, melhores ferramentas disponíveis

As três execuções recebem o mesmo objetivo de produto e os mesmos critérios de
aceitação. Não é necessário restringi-las ao mesmo conjunto técnico.

Cada engine pode usar suas melhores ferramentas oficiais de edição, automação,
linha de comando, testes, inspeção e integração com agentes. A qualidade dessas
ferramentas faz parte do que está sendo avaliado.

Equilíbrio de ferramentas significa igualdade de acesso e de oportunidade, não
obrigar todas as engines a trabalhar pelo menor denominador comum. Uma engine que
oferece de fábrica algo que a outra exige plugin tem uma vantagem real de
produto, e o experimento existe para medi-la, não para neutralizá-la.

### 2.2 Nenhuma orientação de implementação

O protocolo, a premissa e a especificação descrevem resultados observáveis, não o
caminho técnico para alcançá-los.

Não devem indicar:

- APIs;
- classes ou tipos de nós;
- arquitetura;
- padrões de projeto;
- estrutura de arquivos;
- plugins, pacotes ou SDKs;
- sistema de UI, input, câmera, física ou renderização;
- técnica de produção de imagem;
- fórmulas ou estratégias de conversão de coordenadas.

Descobrir as abstrações apropriadas é responsabilidade do agente e parte central
do experimento.

Direção de produto **não** é orientação de implementação. Dizer que a safira é um
brilhante circular azul é requisito de produto; dizer como desenhá-la é proibido.

### 2.3 A experiência do agente é o objeto de avaliação

Uma solução final correta não apaga o caminho necessário para chegar até ela.
Tentativas quebradas, escolhas inadequadas, pedidos de ajuda, correções humanas e
dificuldade de verificar o resultado também são dados do experimento.

### 2.4 Evidência antes de pontuação

Os resultados brutos serão registrados antes da criação de pesos ou de uma nota
final. Isso evita ajustar a régua para favorecer uma conclusão já desejada.

## 3. Organização do diretório

```text
engine-agent-bakeoff/
├── PROTOCOLO.md            este documento (v2)
├── PROTOCOLO-v1.md         versão anterior, preservada
├── ESPECIFICACAO.md        matriz de aceitação dos Estágios A e B
├── shared/
│   ├── PREMISSA.md         o produto, em resultados observáveis
│   └── PROMPT-BASE.md      o prompt inicial, idêntico exceto engine e caminho
├── defold-test/
├── defold-test-treinado/
├── godot-test/
├── unity-test/
└── results/
    ├── defold-test/   godot-test/   unity-test/   defold-test-treinado/
```

`shared/` contém material idêntico e neutro em relação às engines. `results/`
guarda evidências e medições, sem código compartilhado entre soluções.

## 4. Isolamento entre as execuções

Cada engine é trabalhada num chat independente, com contexto novo. Um agente pode
ler o protocolo, a especificação e os materiais compartilhados. Não pode ler o
código, os documentos internos, o histórico ou os resultados das outras
implementações antes de concluir a própria execução. Código, testes e soluções
técnicas não podem ser copiados entre engines.

**O isolamento é garantido por mecanismo, não por convenção.** Cada execução roda
com bloqueio de leitura sobre:

- `~/defold/bomberboom` e qualquer outro projeto anterior do mesmo produto;
- os diretórios das outras execuções;
- os resultados já depositados em `results/`.

O bloqueio é **cirúrgico**: o resto do sistema permanece acessível, porque as
engines precisam ler suas próprias instalações, caches e ferramentas, e um
bloqueio amplo criaria atrito artificial que depois entraria nos números como se
fosse defeito da engine.

Ao final de cada execução, os transcripts são auditados em busca dos caminhos
bloqueados. Um vazamento invalida a execução e obriga a repeti-la.

O fato de os projetos estarem no mesmo diretório-pai é conveniência de
organização e não autoriza acesso cruzado.

## 5. Condições iniciais equivalentes

As três execuções usam:

- o mesmo modelo de IA e o mesmo nível de raciocínio: **Claude Opus 5, esforço
  de raciocínio extra high**, em todas as cinco execuções;
- o mesmo prompt-base, alterando apenas engine e caminho do projeto;
- a mesma matriz de aceitação como condição de término (§5.1);
- as mesmas permissões de sistema e de rede;
- os mesmos materiais brutos;
- os mesmos requisitos e critérios de aceitação;
- acesso equivalente a documentação pública;
- contas, credenciais e dispositivos equivalentes quando indispensáveis.

### 5.1 Sem teto: fixa-se a saída e mede-se o custo

**Não há orçamento máximo.** Todas as execuções correm até atender a matriz de
aceitação ou até encerrarem por bloqueio ou estagnação. O que é igual entre elas
é a **saída exigida**, não o esforço permitido.

Essa é a comparação mais limpa disponível: mesma entrega, custos diferentes. Um
teto — em relógio ou em esforço — transformaria a medida em "quão longe cada uma
chegou com X", que responde uma pergunta mais fraca e, se apertado demais, mede
apenas que ninguém terminou. Sem teto, **custo até a aceitação** vira a métrica
primária e é diretamente comparável entre as engines.

O preço disso é conhecido e aceito: o custo total do experimento fica aberto, e
uma execução pode demorar dias. Isso é a realidade que se quer medir.

#### O critério de parada

Sem teto, ainda é preciso saber quando uma execução acabou. Ela encerra quando:

1. **o agente declara conclusão** — e a matriz externa é então aplicada; ou
2. **o agente declara bloqueio** — não vê caminho e o teto de direções técnicas
   do §7.1 já se esgotou; ou
3. **estagnação** — o operador observa ausência de progresso mensurável ao longo
   de vários ciclos consecutivos: nenhum critério novo fechado, os mesmos erros
   reaparecendo, tentativas circulando entre abordagens já falhadas.

A estagnação é julgamento do operador e por isso entra no registro **com a
evidência que a sustentou**: quais critérios estavam abertos, há quantos ciclos,
e o que se repetia. Ela não é esgotamento de orçamento disfarçado — é o
reconhecimento de que a execução parou de gerar informação nova.

### 5.2 Versões e ponto de partida

A versão exata de cada engine, de suas ferramentas e das dependências escolhidas
deve ser registrada. As versões não precisam ter o mesmo número ou data; devem
ser versões atuais e adequadas para um novo projeto na data do experimento.

Cada execução começa a partir de um projeto vazio ou do template vazio oficial
mais próximo disso. Templates de quebra-cabeça, kits de monetização prontos e
projetos de demonstração que já implementem parte substancial do produto não são
pontos de partida válidos.

## 6. Ferramentas permitidas

O agente pode usar:

- editor e ferramentas oficiais da engine;
- interfaces de linha de comando;
- APIs de automação e inspeção fornecidas pela engine;
- integração oficial com agentes ou protocolos de ferramentas;
- compiladores, exportadores e test runners;
- terminal, controle de versão e ferramentas gerais de desenvolvimento;
- documentação pública oficial ou comunitária;
- bibliotecas, extensões e plugins gratuitos e publicamente acessíveis;
- scripts e ferramentas de teste que o próprio agente criar durante a execução.

Toda dependência externa escolhida deve ser registrada com nome, versão, origem,
justificativa **e origem institucional** — da engine, extensão oficial, plugin de
comunidade ou código próprio. A escolha espontânea e a integração correta dessas
dependências são parte da avaliação.

Ferramentas pagas, assets pagos e soluções prontas que implementem o produto não
podem ser introduzidos em apenas uma engine. Uma exceção exige registro e
aplicação equivalente às demais.

## 7. Intervenção humana

O usuário pode realizar ações que não possam ser delegadas com segurança, como
autorizar uma operação, autenticar uma conta, fornecer credenciais sem expô-las,
aceitar termos, conectar um dispositivo ou executar uma etapa que exija presença
humana.

Toda intervenção é registrada literalmente e classificada:

1. **Autorização:** aprovação de acesso ou operação já proposta pelo agente.
2. **Ambiente:** ação externa inevitável, sem ensinar como implementar o produto.
3. **Esclarecimento:** explicação de um requisito de produto ambíguo.
4. **Diagnóstico:** indicação humana da causa ou localização provável de um erro.
5. **Direção técnica:** indicação de API, recurso da engine ou solução a utilizar.
6. **Implementação humana:** alteração manual de código, cena, configuração ou asset.

As categorias não recebem peso aqui; serão comparadas separadamente.

### 7.1 Quando o agente pede direção técnica

Não responder preserva a pureza da medida e produz execuções truncadas, que não
geram dado sobre build, ciclo de vida ou monetização. Responder sempre garante
execuções completas e destrói a medida de quanto o agente sabia sozinho. A regra
adotada limita a contaminação a um valor conhecido e igual para todos:

- **responde-se o mínimo necessário para destravar**, nunca a solução completa;
- toda direção técnica é registrada **literalmente**, com a pergunta e a resposta;
- o limite é de **três direções técnicas por execução**;
- esgotado o limite, a execução prossegue sem novas direções e, se travar,
  encerra como **bloqueio**.

O usuário não corrige silenciosamente uma implementação. Toda correção permanece
visível como parte do histórico.

## 8. Prompts e esclarecimentos

- O prompt inicial vem de `shared/PROMPT-BASE.md`, único para todas as execuções.
- Diferenças entre prompts limitam-se ao nome da engine e ao caminho do projeto.
- O prompt não antecipa problemas já encontrados em outra engine.
- Perguntas equivalentes recebem respostas equivalentes.
- Uma descoberta feita numa execução não vira dica para as seguintes.
- Se uma dúvida revelar defeito real na premissa ou na especificação, o documento
  é versionado e a mudança registrada para todas as execuções afetadas.

Quando uma alteração de requisito ocorrer depois que uma engine já foi avaliada,
registra-se se aquela parte precisa ser repetida para manter a comparação válida.

## 9. As execuções

São **cinco**, com papéis distintos.

| # | Execução | Papel | Entra na comparação? |
|---|---|---|---|
| 1 | Godot | comparação | sim |
| 2 | Unity | comparação | sim |
| 3 | Defold (cru) | comparação | sim |
| 4 | Confirmação | repete a execução cujo resultado sustentaria a migração, em condições idênticas | não; testa reprodutibilidade |
| 5 | Defold (treinado) | mede quanto vale o conhecimento acumulado | **não**; medição à parte |

### 9.1 A ordem

Godot, Unity, Defold cru, confirmação, Defold treinado.

O Defold cru fica por último entre as três porque o **operador** aprende ao longo
do experimento, e a última posição é a vantajosa. Pôr a incumbente ali é
deliberadamente conservador contra a conclusão cara: se o Defold perder ocupando a
posição favorável, "migrar" fica robusto. Se ganhar, "ficar" é a decisão barata,
que não exige evidência forte.

### 9.2 A execução treinada

A execução 5 recebe acesso ao corpus acumulado do projeto anterior — documentação
de decisões, notas de arquitetura e armadilhas já depuradas — que as execuções 1
a 3 têm bloqueado.

Ela responde uma pergunta que a comparação de três engines não responde: **quanto
vale o conhecimento acumulado?** Se ela humilhar a execução crua, migrar não custa
só reescrever o jogo — custa descartar um ativo que precisaria ser reconstruído do
zero na engine nova.

**Ela não entra na comparação entre engines.** Godot e Unity não têm corpus
equivalente, e compará-las com um Defold treinado favoreceria o Defold
artificialmente.

### 9.3 Repetições

Uma execução por engine. Mais uma repetição de confirmação, em condições
idênticas, na execução cujo resultado sustentaria a migração.

A assimetria é deliberada e está declarada **antes** da leitura dos resultados: a
decisão que o experimento alimenta é assimétrica — ficar é barato e reversível,
migrar é caro e não é —, então o erro que vale prevenir é migrar por causa de
ruído.

Com uma execução por célula, o experimento **não separa engine de variância do
agente**. A limitação fica registrada e o §11 proíbe generalizar a partir dela.

## 10. Procedimento de uma execução

Antes de iniciar: registrar engine, versão, ferramentas, sistema operacional e
estado inicial; confirmar que o projeto está vazio e isolado; confirmar as versões
da premissa, da especificação e do protocolo; abrir um chat novo, sem contexto das
outras; registrar o horário de início.

Durante: permitir que o agente planeje e escolha livremente o caminho técnico;
registrar builds, testes, falhas, bloqueios e intervenções; não alterar a régua de
aceitação; não passar dicas entre execuções; preservar as evidências produzidas.

Ao encerrar: registrar se a execução encerrou por conclusão, bloqueio ou
estagnação, e a evidência disso; executar os critérios externos de aceitação; capturar resultados, logs
e evidências visuais; registrar pendências e defeitos **sem corrigi-los antes do
primeiro retrato final**; preservar uma revisão identificável do projeto avaliado.

## 11. Dados a coletar

Além do que a `ESPECIFICACAO.md` exige no registro de execução, os resultados
devem permitir comparar:

**Custo**
- custo em tokens e em dólares por execução;
- tempo até a primeira execução bem-sucedida;
- tempo total até a declaração de conclusão;
- wall-clock separado em **raciocínio do agente** e **espera de ferramenta**.

A separação do tempo é obrigatória porque os dois atribuem a causas opostas: o
primeiro diz que a engine é conceitualmente difícil ou mal documentada; o segundo
é compilação, importação e abertura de editor. Somados, os efeitos se escondem, e
é justamente a diferença entre "a engine é lenta" e "a engine é confusa".

**Autonomia**
- ciclos editar → executar → observar → corrigir fechados **sem humano no meio**;
- capacidade do agente de observar o estado do editor e do jogo;
- capacidade do agente de verificar as próprias alterações;
- passos manuais ainda necessários ao final.

**Atrito**
- tempo de build e de repetição do ciclo;
- builds ou execuções com falha;
- quantidade e categoria das intervenções humanas;
- tentativas até atender cada critério de aceitação.

**Resultado**
- acerto funcional na primeira entrega;
- defeitos restantes ao final;
- dependências adicionadas e sua origem institucional;
- tamanho e complexidade aproximada da solução;
- sucesso na geração e execução do pacote mobile;
- sucesso e robustez das integrações mobile exigidas;
- comportamento em diferentes proporções, dispositivos e ciclos de vida;
- clareza da documentação e reprodutibilidade deixadas pelo agente.

Registra-se também quando o agente reimplementa manualmente algo que a engine ou
uma dependência já oferece. O fato é observado, não previamente proibido, porque a
escolha técnica é parte do teste.

## 12. Avaliação

- Nenhuma engine recebe nota final antes de todas as evidências comparáveis existirem.
- Resultados brutos permanecem disponíveis junto de qualquer resumo ou nota.
- Falhas são atribuídas com cautela entre agente, engine, ferramenta, plugin,
  serviço externo e ambiente.
- Uma vantagem isolada não é generalizada sem verificar se aparece em mais de uma
  etapa do trabalho.
- Diferenças marginais são distinguidas de diferenças capazes de justificar o
  custo de uma migração real.

Os pesos dos critérios e o limiar de decisão serão definidos depois que a
especificação e a matriz estiverem fechadas, mas **antes** da leitura dos
resultados finais.

## 13. Serviços externos, contas e segredos

- Segredos nunca são gravados em código, prompts, logs ou controle de versão.
- Integrações usam ambientes de teste, sandbox ou identificadores de teste sempre
  que disponíveis.
- Custos externos precisam de autorização prévia e devem ser equivalentes.
- Indisponibilidade de serviço externo é registrada e, quando possível, testada
  novamente, sem penalizar automaticamente a engine.
- Etapas que dependam de aprovação de loja ou propagação externa são medidas
  separadamente do trabalho que o agente controla.

A monetização do Estágio B foi escolhida para **não** depender de loja: o portão
da demonstração é local e o anúncio usa unidade de teste oficial. Isso remove uma
classe inteira de falha que não é culpa da engine e que embaralharia a atribuição.

## 14. Controle de mudanças

- Protocolo, premissa e especificação são versionados.
- Toda exceção indica motivo, engine afetada, momento e impacto provável.
- Uma regra nova não é aplicada retroativamente a apenas uma implementação.
- Alterações durante o experimento preservam as versões anteriores dos documentos
  usados por cada execução.

## 15. Decisões fechadas

| Ponto | Decisão |
|---|---|
| Premissa | `shared/PREMISSA.md` v1.1 |
| Escopo | Estágio A (jogo + pacote Android) → portão → Estágio B (monetização) |
| Núcleo do Estágio A | Enxuto: sem fúria, bomba secundária, obstáculos ou múltiplos idiomas |
| Monetização | Portão da demonstração local + anúncio com unidade de teste oficial |
| Arte | Alvo fixo (cinco silhuetas e cores dadas); aceitação por piso observável |
| Arte, duas passadas | Passada 1 só com o texto da §11 (julga A13/A14 e é congelada); passada 2 com as imagens de referência, no mesmo ponto das três execuções, sem aviso prévio ao agente |
| Orçamento | **Sem teto.** Fixa-se a saída, mede-se o custo; parada por conclusão, bloqueio ou estagnação |
| Plataforma | Android obrigatório; iOS opcional e registrado à parte |
| Alvo de verificação | Aparelho físico Samsung SM-G780G (Galaxy S20 FE 5G), Android 13, API 33, arm64-v8a, tela nativa 1080×2400 densidade 480, por depuração sem fio. Perfil 20:9 é o nativo; perfil 9:16 sai por `wm size 1080x1920` e `wm density 420`, revertido com `reset`. Substituiu o emulador em 2026-08-22 (ver §16.1) |
| Execuções | Godot, Unity, Defold cru, confirmação, Defold treinado |
| Ordem | Defold cru por último entre as três; treinado depois de tudo |
| Intervenção cat. 5 | Mínimo necessário, registrado, teto de três por execução |
| Isolamento | Bloqueio cirúrgico por mecanismo, auditado ao final; um repositório remoto privado por execução, nunca compartilhado; cada diretório de teste é repositório git próprio |
| Repetições | Uma por engine, mais uma confirmação na execução decisiva |
| Agente | Claude Opus 5, raciocínio extra high, igual nas cinco execuções |

## 16.1 Mudança de alvo de verificação — 2026-08-22

O alvo saiu do emulador Android e passou a ser um aparelho físico.

**Motivo imediato:** disco. O emulador, suas imagens de sistema e os dois AVDs
ocupavam 11,4 GB numa máquina que chegou a 2,8 GB livres, com o Unity ainda por
instalar.

**Motivo de mérito, que pesa mais:** o emulador rodava com renderização por
software por falta de memória, então qualquer observação de desempenho ali
mediria o emulador e não a engine. E o critério A16 é precisão de toque — um
dedo num painel real o testa, um clique de mouse não.

**O que a troca preservou.** A15 exige duas proporções e um aparelho tem uma só.
A resolução lógica é forçada por software, e foi verificado antes de apagar o
emulador que o aparelho aceita o override e que a captura devolve a resolução
forçada.

**O que a troca custou.** O Godot fechou A15, A16 e A17 no emulador e precisa
refazê-los no aparelho, para que as três execuções sejam julgadas no mesmo alvo
(§5). O custo dessa reverificação é registrado à parte: paga uma mudança de
ambiente, não um defeito do agente.

**Risco introduzido.** Um aparelho pode dormir, cair da rede ou perder
autorização, e cada queda dessas vira intervenção humana — que suja justamente a
métrica de autonomia. Por isso a conexão é sem fio e não por cabo: a porta USB do
aparelho estava intermitente, e cabo ruim derruba conexão no meio de verificação.

## 16. Ainda em aberto

- versões exatas das três engines na data de início;
- pesos dos critérios e limiar de decisão, a fixar antes da leitura final.
