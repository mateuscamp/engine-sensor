# Sara

Verificador AI-first interno e estudo sobre desenvolvimento de jogos quando quem
escreve o código é um agente. **Sara é o nome provisório enquanto o projeto for esta
camada de verificação.** Se um dia ele se tornar uma engine, nome e marca serão uma
nova decisão. Publicação e monetização permanecem adiadas.

O primeiro resultado é o `sara 0.1.0`: um CLI Rust, offline, para detectar posse
concorrente de animação e entrada em projetos Godot 4.7 e Defold 1.13. Ele bloqueia
somente conflitos comprovados e transforma ambiguidades em avisos explicados.

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
expansão do escopo, incluindo o spike Godot de visão instrumentada posterior ao uso
pessoal.

## A decisão atual

Há evidência de uma lacuna em verificabilidade para agentes. Ainda não há evidência
de que uma engine completa seja necessária ou economicamente viável. Por isso, o
projeto permanece deliberadamente na camada de verificação compatível com engines
existentes. A próxima prova são dez mudanças reais em ao menos dois projetos pessoais.

As decisões estão registradas na
[ADR 0001](docs/decisoes/0001-validar-mecanismos-antes-da-engine-completa.md) e na
[ADR 0002](docs/decisoes/0002-lancamento-interno-e-foco-adaptativo.md). A troca do
codinome anterior por Sara está na
[ADR 0003](docs/decisoes/0003-sara-como-nome-provisorio.md). O próximo experimento
aprovado, ainda condicionado ao Marco 6, está na
[ADR 0004](docs/decisoes/0004-spike-de-visao-instrumentada-em-godot.md).

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
