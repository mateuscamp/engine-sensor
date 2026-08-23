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
