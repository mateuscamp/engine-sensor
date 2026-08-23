#!/usr/bin/env bash
set -euo pipefail

cargo test --test corpus -- --ignored

echo "Sara: os cinco projetos do corpus não têm conflito bloqueante."
