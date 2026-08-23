#!/usr/bin/env bash
set -euo pipefail

# Produz o artefato distribuído em dist/. O binário é stripped pelo perfil de
# release do Cargo.toml, e não por um passo manual: o tamanho registrado em
# docs/RESULTADO-0.1.0.md é o deste arquivo.

cargo test
cargo build --release

install -m 755 target/release/sara dist/sara-linux-x86_64
( cd dist && sha256sum sara-linux-x86_64 > SHA256SUMS )

echo "Sara: dist/sara-linux-x86_64 com $(stat -c %s dist/sara-linux-x86_64) bytes."
sed 's/^/  /' dist/SHA256SUMS
