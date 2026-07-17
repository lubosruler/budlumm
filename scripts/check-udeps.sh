#!/usr/bin/env bash
# ============================================================================
# check-udeps.sh — G3 (ADIM8 §3.3): cargo-udeps kullanılmayan-bağımlılık kapısı
#
# `cargo +nightly udeps --all-targets` çıktısını alır; "unused crates:" bölümünde
# listelenen her crate'i .github/udeps-baseline.txt (izin verilenler) ile
# karşılaştırır. Baseline'da OLMAYAN kullanılmayan dep = FAIL (ratchet: yeni
# kullanılmayan dep eklenemez; temizlik PR'ları baseline'ı boşaltır).
# Baseline dosyası yoksa SKIP (ilk CI koşusu ölçer, 2. adımda taban yazılır —
# vacuous-gate YOK).
#
# Kullanım:
#   bash scripts/check-udeps.sh <udeps-cikti>   # kapı
#   bash scripts/check-udeps.sh --self-test     # kanarya
# ============================================================================
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BASELINE="$ROOT/.github/udeps-baseline.txt"

gate() {
  local out="$1"
  [ -s "$out" ] || { echo "FAIL: udeps çıktısı yok/boş: $out"; return 1; }
  if [ ! -f "$BASELINE" ]; then
    echo "SKIP: $BASELINE yok — ilk ölçüm (adım 1); bulgular:"
    cat "$out"
    return 0
  fi
  # udeps çıktı formatı: 'unused crates:' altında girintili satırlar (crate adıyla başlar)
  local unused
  unused=$(sed -n '/unused crates:/,$p' "$out" | tail -n +2 | grep -E '^\S' || true)
  if [ -z "$unused" ]; then
    echo "OK: kullanılmayan bağımlılık yok."
    return 0
  fi
  local fail=0 line
  while IFS= read -r line; do
    if ! grep -qxF "$line" "$BASELINE"; then
      echo "FAIL: baseline'da olmayan kullanılmayan bağımlılık: $line"
      fail=1
    fi
  done <<< "$unused"
  [ "$fail" -eq 0 ] && { echo "OK: tüm kullanılmayanlar bilinen baseline'da ($(echo "$unused" | wc -l) adet)."; return 0; }
  return 1
}

if [ "${1:-}" = "--self-test" ]; then
  tmp=$(mktemp -d); base_bak="$BASELINE"
  printf 'note: some note\nunused crates:\nserde_temizlik\ntokio_koprusu\n' > "$tmp/u.txt"
  printf 'serde_temizlik\n' > "$ROOT/.github/udeps-baseline.txt"
  code=0
  gate "$tmp/u.txt" >/dev/null 2>&1 || code=$?
  [ "$code" -eq 1 ] || { echo "VACUOUS GATE: bilinmeyen dep (tokio_koprusu) geçti!"; rm -f "$ROOT/.github/udeps-baseline.txt"; exit 1; }
  printf 'note: ok\nAll dependencies are used\n' > "$tmp/temiz.txt"
  gate "$tmp/temiz.txt" >/dev/null 2>&1 || { echo "BOZUK KAPI: temiz çıktı reddedildi!"; rm -f "$ROOT/.github/udeps-baseline.txt"; exit 1; }
  rm -f "$ROOT/.github/udeps-baseline.txt"
  echo "kanarya OK: bilinmeyen-dep FAIL, temiz PASS."
  exit 0
fi
[ $# -ge 1 ] || { echo "kullanım: $0 <udeps-cikti> | --self-test"; exit 1; }
gate "$1"
