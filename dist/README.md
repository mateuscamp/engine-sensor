# Sara 0.1.0 para Linux x86_64

O arquivo `sara-linux-x86_64` é o binário interno do verificador AI-first.

```text
chmod +x sara-linux-x86_64
./sara-linux-x86_64 --help
./sara-linux-x86_64 init /caminho/do/jogo
./sara-linux-x86_64 check /caminho/do/jogo
```

Confira a integridade com `sha256sum -c SHA256SUMS`. O binário funciona offline e não
possui telemetria nem mecanismo de atualização.

## Reconstrução de 23/08/2026

Este arquivo foi reconstruído sobre a fronteira núcleo/adapter da Fase 1 da auditoria.
O comportamento é idêntico ao do artefato anterior — mesma saída e mesmo código de saída
nas dezoito fixtures, nos dois perfis —, mas o conteúdo e o hash mudaram. Duas somas
diferentes portanto existem para a versão `0.1.0`; a válida é a do `SHA256SUMS` ao lado.

Reproduza o artefato com `tools/dist.sh`. O `strip` está no perfil de release do
`Cargo.toml`, não em um passo manual.
