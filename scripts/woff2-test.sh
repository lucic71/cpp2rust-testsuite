#!/bin/bash

set -e

# Usage: woff2-test.sh <build_dir> <source_dir> <model...>
BUILD_DIR="$1"
SRC_DIR="$2/woff2"
shift 2
MODELS=("$@")

FONTS_DIR="$SRC_DIR/tests"
TMP_DIR="$BUILD_DIR/tmp-woff2"

. "$(dirname "$0")/woff2-common.sh"
setup_woff2_tmp

# Run each model and compare against original

for model in "${MODELS[@]}"; do
  RUST_BIN="$SRC_DIR/out/$model/target/release"
  MODEL_DIR="$TMP_DIR/$model"

  rm -fr "$MODEL_DIR"
  mkdir -p "$MODEL_DIR"

  cp "$TMP_DIR"/original/*.ttf "$MODEL_DIR"

  for f in "$MODEL_DIR"/*.ttf; do
    "$RUST_BIN"/woff2_compress "$f" &
  done
  wait

  # Compare woff2 files against original
  for f in "$MODEL_DIR"/*.woff2; do
    base=$(basename "$f" .woff2)
    diff "$f" "$TMP_DIR/original/$base.woff2" \
      || { echo "FAIL [$model]: woff2 mismatch on $base"; exit 1; }
  done

  # Decompress and compare ttf roundtrip
  rm -f "$MODEL_DIR"/*.ttf
  for f in "$MODEL_DIR"/*.woff2; do
    "$RUST_BIN"/woff2_decompress "$f" &
  done
  wait

  for f in "$MODEL_DIR"/*.ttf; do
    base=$(basename "$f" .ttf)
    diff "$f" "$TMP_DIR/cc-decompressed/$base.ttf" \
      || { echo "FAIL [$model]: ttf mismatch on $base"; exit 1; }
  done

  # Compare woff2_info output
  for f in "$MODEL_DIR"/*.woff2; do
    base=$(basename "$f" .woff2)
    "$RUST_BIN"/woff2_info "$f" | tail -n +2 > "$MODEL_DIR/$base.info"
    diff "$MODEL_DIR/$base.info" "$TMP_DIR/original/$base.info" \
      || { echo "FAIL [$model]: info mismatch on $base"; exit 1; }
  done

  echo "WOFF2 $model tests passed!"
done

rm -fr "$TMP_DIR"
