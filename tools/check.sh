#!/usr/bin/env bash
set -euo pipefail

cargo test
cargo build --release

echo "engine-sensor: testes e binário release concluídos."
