#!/usr/bin/env bash
set -euo pipefail

cargo test
cargo build --release

echo "Sara: testes e binário release concluídos."
