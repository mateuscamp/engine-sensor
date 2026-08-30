#!/usr/bin/env bash
#
# Portão do corpus — o confronto que a ADR 0012 §3 exige, com os três estados que a
# ADR 0017 decidiu.
#
# Códigos de saída, os mesmos três do contrato do `sara` (ADR 0006), pelo mesmo motivo:
#
#   0  APROVADO      os cinco projetos foram lidos e nenhum tem conflito bloqueante
#   1  REPROVADO     algum projeto estava lá e o confronto encontrou erro nele
#   2  INCONCLUSIVO  o corpus não estava no lugar declarado — não deu para conferir
#
# 2 não é 0. Não poder conferir não é ter conferido, e quem registra o resultado deste
# portão registra bloqueio externo, nunca aprovação implícita.
#
# Os caminhos vêm do ambiente; os valores de hoje são só o padrão documentado:
#
#   SARA_CORPUS_RAIZ            padrão /home/mateus
#   SARA_CORPUS_BOMBERBOOM_DF   padrão $SARA_CORPUS_RAIZ/defold/bomberboom-df
#   SARA_CORPUS_BOMBERBOOM_GD   padrão $SARA_CORPUS_RAIZ/godot/bomberboom-gd
#   SARA_CORPUS_BOOMLITUDE      padrão $SARA_CORPUS_RAIZ/godot/boomlitude
#   SARA_CORPUS_MINEBOOM        padrão $SARA_CORPUS_RAIZ/godot/mineboom
#   SARA_CORPUS_GODS            padrão $SARA_CORPUS_RAIZ/godot/gods
#
# O teste não é mais `#[ignore]`: `cargo test` sozinho já o roda. Este script continua
# existindo porque é ele que traduz o terceiro estado em código de saída — coisa que o
# arnês do Cargo, com dois estados, não sabe fazer.

set -uo pipefail

veredito="$(mktemp)"
trap 'rm -f "$veredito"' EXIT

SARA_CORPUS_VEREDITO="$veredito" cargo test --test corpus -- --nocapture
codigo_do_cargo=$?

estado="$(head -n 1 "$veredito" 2>/dev/null || true)"
detalhe="$(tail -n +2 "$veredito" 2>/dev/null || true)"

case "$estado" in
  aprovado)
    if [ "$codigo_do_cargo" -ne 0 ]; then
      echo "Sara: corpus INCONCLUSIVO — o veredito diz aprovado e o cargo saiu ${codigo_do_cargo}." >&2
      echo "Portão que se contradiz não aprova nada. Rode 'cargo test --test corpus' e leia a saída." >&2
      exit 2
    fi
    echo "Sara: corpus APROVADO — os cinco projetos do corpus foram lidos e nenhum tem conflito bloqueante."
    printf '%s\n' "$detalhe" | sed 's/^/  /'
    exit 0
    ;;
  reprovado)
    echo "Sara: corpus REPROVADO — o confronto encontrou conflito bloqueante:" >&2
    printf '%s\n' "$detalhe" | sed 's/^/  /' >&2
    exit 1
    ;;
  inconclusivo)
    echo "Sara: corpus INCONCLUSIVO — não foi possível conferir. Faltou:" >&2
    printf '%s\n' "$detalhe" | sed 's/^/  /' >&2
    echo "Registre isto como bloqueio externo, não como aprovação: não poder conferir não é ter conferido." >&2
    exit 2
    ;;
  *)
    echo "Sara: corpus INCONCLUSIVO — o teste não chegou a emitir veredito (cargo saiu ${codigo_do_cargo})." >&2
    echo "É falha de execução do portão, e ela também não vale por aprovação." >&2
    exit 2
    ;;
esac
