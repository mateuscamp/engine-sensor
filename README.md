# compositando-engine

Estudo sobre desenvolvimento de jogos quando quem escreve o código é um agente — e o
projeto de uma engine desenhada para isso.

Tudo aqui vem de medição. Um agente construiu o mesmo jogo mobile em **Godot, Unity e
Defold**, sem contexto entre as execuções e sem receber uma única indicação de API. Os
registros brutos, o protocolo e os critérios de aceitação estão em `estudo/`.

## Por onde começar

**[RESULTADOS.md](RESULTADOS.md)** — o resultado do estudo: os números, o que ele concluiu,
o que ele explicitamente não concluiu, e o que ficou aberto.

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
