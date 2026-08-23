# Contrato AI-first do projeto

Este projeto é alterado por agentes e precisa ser verificável sem depender de uma
pessoa olhando a engine.

## Regras

1. Domínio não importa cena, entrada, áudio, GPU, janela nem relógio global.
2. Toda aleatoriedade de regra recebe uma semente e passa por um gerador único.
3. Entrada física é normalizada em um ponto antes de alterar o estado.
4. Uma propriedade animada tem um proprietário. Composição deliberada precisa de
   exceção exata e justificada em `sara.toml`.
5. Logs para verificação são estruturados; texto livre é apenas para pessoas.
6. O comando de teste e o comando de prova de cena vivem no repositório.
7. Conclusão exige executar `sara check . --format json`.
8. Durante o Marco 6, mudança de entrada ou animação registra resultado, tempo,
   avisos e inspeção humana em `.sara/USOS.md`.

## Como ler o resultado

- saída 0: não há conflito comprovado; avisos ainda precisam ser relatados;
- saída 1: há conflito comprovado e a tarefa não está concluída;
- saída 2: a ferramenta não conseguiu provar que analisou o projeto inteiro; a tarefa
  também não está concluída.

Aviso não autoriza adivinhar. Explique o limite, reduza a construção dinâmica quando
isso melhorar o projeto ou registre uma exceção específica com motivo.

## Evidência mínima

- teste de domínio headless;
- teste ou captura da cena pelo caminho de produção;
- semente e entradas suficientes para reproduzir o caso;
- log ou artefato que permita a outro agente verificar a conclusão.
