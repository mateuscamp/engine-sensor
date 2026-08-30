# Sara — o sensor

Verificador AI-first interno e estudo sobre desenvolvimento de jogos quando quem
escreve o código é um agente. **Este repositório é o sensor**, e só ele: desde a
[ADR 0016](docs/decisoes/0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md)
o pré-projeto de uma possível engine vive em repositório próprio, com série de decisões
própria. **Sara é o nome provisório enquanto o projeto for esta camada de verificação**
([ADR 0003](docs/decisoes/0003-sara-como-nome-provisorio.md)). Publicação e monetização
permanecem adiadas.

O primeiro resultado é o `sara 0.1.0`: um CLI Rust, offline, para detectar posse
concorrente de animação e entrada em projetos Godot 4.7 e Defold 1.13. Ele bloqueia
somente conflitos comprovados e transforma ambiguidades em avisos explicados.

Desde a [ADR 0005](docs/decisoes/0005-foco-em-godot-com-defold-congelado.md), Godot é o
foco de desenvolvimento e o adapter Defold está congelado: sem regra nova, mas com as
duas regressões históricas preservadas — são elas o teste causal do Portão 0.

Tudo aqui vem de medição. Um agente construiu o mesmo jogo mobile em **Godot, Unity e
Defold**, sem contexto entre as execuções e sem receber uma única indicação de API. Os
registros brutos, o protocolo e os critérios de aceitação estão em `estudo/`.

## Usar o verificador

O binário Linux x86_64 está em [`dist/sara-linux-x86_64`](dist/sara-linux-x86_64).
Para experimentar sem instalação global:

```text
./dist/sara-linux-x86_64 init /caminho/do/jogo
./dist/sara-linux-x86_64 check /caminho/do/jogo
./dist/sara-linux-x86_64 check /caminho/do/jogo --profile android --format json
```

`sara init` cria o contrato em `.sara/`, fragmentos separados para `AGENTS.md` e
`CLAUDE.md`, um portão executável da engine e `sara.toml`. Arquivos de instruções já
existentes nunca são sobrescritos: o agente ou a pessoa incorpora o fragmento
correspondente de forma deliberada.

Os códigos de saída são `0` para nenhum conflito comprovado, `1` para conflito e `2`
para configuração, parsing ou falha interna. A integridade do binário pode ser
conferida com [`dist/SHA256SUMS`](dist/SHA256SUMS).

## Por onde entender o projeto

**[RESULTADOS.md](RESULTADOS.md)** — o resultado do estudo: os números, o que ele concluiu,
o que ele explicitamente não concluiu, e o que ficou aberto.

**[docs/DIAGNOSTICO-INICIAL.md](docs/DIAGNOSTICO-INICIAL.md)** - o diagnóstico de
necessidade, as opções de produto, as características mensuráveis, os riscos, a
economia inicial no itch.io e os portões que separam ferramenta, runtime e engine.

**[docs/METODO-ARQUITETURAL.md](docs/METODO-ARQUITETURAL.md)** - como as duas
referências de arquitetura serão usadas como método de decisão sem transformar os
livros em requisitos do produto.

**[docs/RESULTADO-0.1.0.md](docs/RESULTADO-0.1.0.md)** - testes, corpus, desempenho,
trade-offs e o que ainda precisa ser provado no uso pessoal.

**[docs/ROTEIRO.md](docs/ROTEIRO.md)** - os marcos e fitness functions que limitam a
expansão do escopo, e o registro do Marco 7 cancelado.

**[docs/AUDITORIA-ARQUITETURAL.md](docs/AUDITORIA-ARQUITETURAL.md)** - a auditoria do
projeto contra as duas referências, os oito achados, as fitness functions de governança
e o plano final de implementação. O desenho está em
[docs/arquitetura.svg](docs/arquitetura.svg).

**[docs/PERGUNTA-7.md](docs/PERGUNTA-7.md)** - a última fitness function do Marco 7, a única
que produzia conhecimento: imagem mais estado diagnostica melhor que imagem isolada? Sim — e o
limite que a resposta trouxe é maior que ela.

**[docs/CASO-DA-ARANHA.md](docs/CASO-DA-ARANHA.md)** - uma sessão de trabalho real lida
inteira: nove defeitos, quem achou cada um, e por que os dois que importavam passaram por
303 casos verdes, pela Sentinela e pelo `sara check`. É matéria-prima de decisão, não
decisão.

**[docs/CASO-DO-DESENHISTA.md](docs/CASO-DO-DESENHISTA.md)** - a primeira ferramenta do
corpus cujo produto inteiro é uma **fala para a agente**, e não conteúdo: um prompt virado
desenho. E a medição que ela permitiu — dos 33 pontos de entrada do porte, a Sara declara 2;
os outros 31 são tecla e sinal de botão, e quatro deles vão ao aparelho.

## A decisão atual

Há evidência de uma lacuna em verificabilidade para agentes. Ainda não há evidência
de que uma engine completa seja necessária ou economicamente viável. Por isso, o
projeto permanece deliberadamente na camada de verificação compatível com engines
existentes.

**O Marco 6 encerrou em 28 de agosto de 2026, por conclusão**, com treze mudanças reais —
todas do porte do BomberBoom — vinte e três dias antes do critério de parada. O portão
decidiu **manter a Sara privada**, pela
[ADR 0013](docs/decisoes/0013-manter-a-sara-privada-ao-fim-do-marco-6.md).

É o desfecho que uma previsão datada de 25/08 antecipou, antes da evidência que o
confirmaria: *"o recorte é pequeno demais para ser produto e grande o bastante para ser
portão privado"*. Nenhuma das duas condições que ela listou como capazes de derrubá-la
aconteceu.

O que a série mostrou, e vale pelos dois lados: **1 aviso útil e 0 falsos em treze usos**,
e um defeito que só a Sara achou — o mesmo que, na versão Defold, chegou ao jogador. Contra:
**onze dos treze casos ainda exigiram inspeção humana**, que é o recurso escasso que o
projeto existe para economizar. A ADR 0013 registra os dois.

**Em 29 de agosto de 2026 o pré-projeto da engine saiu deste repositório**, pela
[ADR 0016](docs/decisoes/0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md).
A causa não foi ambição de escopo: foi um defeito medido. Quatro rodadas de revisão
externa auditaram uma branch que estava trinta commits atrás do acervo — sem uma ADR
aceita, sem a fitness function dela e sem um eixo inteiro do verificador. Dois produtos
dividindo uma árvore, uma série de decisões e um portão. **Verde por construção, porque a
pergunta foi feita a um recorte que não continha a resposta** — o defeito que este projeto
existe para nomear, acontecendo com ele mesmo.

A fronteira agora é física e tem portão executável: `docs/engine/` não existe aqui, e
`adr_0016_o_sensor_nao_hospeda_o_pre_projeto_da_engine` reprova se voltar. A única
ligação entre os dois repositórios é a matriz do legado, que mora lá e cita este por
caminho e revisão alcançável a partir de `origin/main`.

| ADR | Decisão |
|---|---|
| [0001](docs/decisoes/0001-validar-mecanismos-antes-da-engine-completa.md) | validar mecanismos antes de uma engine completa |
| [0002](docs/decisoes/0002-lancamento-interno-e-foco-adaptativo.md) | lançamento interno e foco adaptativo de engine |
| [0003](docs/decisoes/0003-sara-como-nome-provisorio.md) | Sara como nome provisório da camada |
| [0004](docs/decisoes/0004-spike-de-visao-instrumentada-em-godot.md) | ~~spike de visão instrumentada em Godot~~ — a Sentinela já o entregava (0014) |
| [0005](docs/decisoes/0005-foco-em-godot-com-defold-congelado.md) | foco em Godot, com Defold congelado como corpus de regressão |
| [0006](docs/decisoes/0006-contrato-estrito-de-relatorio-e-codigos-de-saida.md) | contrato estrito de relatório e códigos de saída |
| [0007](docs/decisoes/0007-observe-como-binario-separado.md) | `sara observe` nasce como binário separado |
| [0008](docs/decisoes/0008-gramaticas-tree-sitter-comunitarias-fixadas.md) | gramáticas tree-sitter comunitárias, fixadas por versão exata |
| [0009](docs/decisoes/0009-baseline-em-projeto-real-expoe-regra-ausente.md) | baseline em projeto real também autoriza regra ausente |
| [0010](docs/decisoes/0010-canal-fisico-de-entrada-sem-mapa-de-acoes.md) | canal físico de entrada em Godot, sem exigir mapa de ações |
| [0011](docs/decisoes/0011-marco-7-exige-comparacao-com-ferramenta-existente.md) | o Marco 7 não começa sem comparação com ferramenta existente — cumprida |
| [0012](docs/decisoes/0012-sara-e-corpus-coevoluem.md) | Sara e corpus coevoluem; a evolução do instrumento é registrada |
| [0013](docs/decisoes/0013-manter-a-sara-privada-ao-fim-do-marco-6.md) | manter a Sara privada ao fim do Marco 6 |
| [0014](docs/decisoes/0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md) | não construir o spike do Marco 7 — a Sentinela já o entregava — e preservar a pergunta 7 |
| [0015](docs/decisoes/0015-a-verdade-de-design-sao-tres-campos-no-carimbo.md) | a verdade de design são três campos no carimbo, e um deles bloqueia |
| [0016](docs/decisoes/0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md) | a engine sai de casa antes do G0, e este repositório é o sensor |

## Os artigos

Leem-se em ordem, mas cada um se sustenta sozinho.

| | Artigo | Sobre |
|---|---|---|
| 1 | [O Agente Não Vê](artigos/1-o-agente-nao-ve.html) | A versão de jogos do *Clean Code para agentes*, do Akita. O que muda quando o resultado do código é uma tela e não um valor de retorno. Termina num `CLAUDE.md` pronto para usar. |
| 2 | [O Motor que Narra](artigos/2-o-motor-que-narra.html) | O projeto de uma engine cujo usuário principal é um processo. Cada regra do artigo 1 vira um mecanismo. Com API, arquitetura, riscos e roteiro. |
| 3 | [O Que Só Aparece Depois](artigos/3-o-que-so-aparece-depois.html) | Seis lições de quatro projetos com história — `gods`, `bomberboom`, `boomlitude`, `mineboom` — que um experimento de uma sessão não pode medir. |

Os arquivos são HTML autocontido: abrir no navegador basta.

## O estudo

```
estudo/
├── PROTOCOLO.md         regras de equilíbrio, isolamento e registro (v2)
├── ESPECIFICACAO.md     matriz de aceitação: 19 critérios no Estágio A, 13 no B
├── PREMISSA.md          o produto, em resultados observáveis (v1.1)
└── registros/
    ├── godot.md         registro completo da execução Godot
    ├── unity.md         idem, Unity
    └── defold.md        idem, Defold
```

Os registros são o material bruto: marcos, custo, intervenções literais com categoria,
tentativas até fechar cada critério, dependências com origem institucional, e os defeitos
que ficaram — anotados antes de serem corrigidos, como o protocolo exige.

## A tese, em uma frase

As três engines são acessíveis a um agente sem orientação humana. A diferença não está no
que ele consegue escrever — está **no que ele consegue provar sozinho**, e é esse o recurso
que falta quando o projeto inteiro depende da atenção de uma pessoa.
