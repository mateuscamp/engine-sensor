#!/usr/bin/env bash
set -euo pipefail

# Produz o artefato distribuído em dist/. O binário é stripped pelo perfil de
# release do Cargo.toml, e não por um passo manual: o tamanho registrado em
# docs/RESULTADO-0.1.0.md é o deste arquivo.

# A ordem importa desde 29/08/2026, e ela é o oposto da intuitiva. O portão
# `adr_0012_o_binario_publicado_responde_como_o_codigo` roda o binário de dist/ contra o
# do código sobre todas as fixtures: com `cargo test` antes do build, um dist/ atrasado
# reprova a suíte e o script nunca chegaria a consertá-lo. Construindo primeiro, a suíte
# passa a conferir o artefato que está sendo publicado, que é uma garantia mais forte do
# que a de antes -- e `set -e` impede que um teste vermelho siga adiante.
cargo build --release

install -m 755 target/release/sara dist/sara-linux-x86_64
( cd dist && sha256sum sara-linux-x86_64 > SHA256SUMS )

cargo test

echo "Sara: dist/sara-linux-x86_64 com $(stat -c %s dist/sara-linux-x86_64) bytes."
sed 's/^/  /' dist/SHA256SUMS
