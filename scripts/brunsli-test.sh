#!/bin/bash

set -e

# Usage: brunsli-test.sh <build_dir> <source_dir> <model...>
BUILD_DIR="$1"
SRC_DIR="$2/brunsli"
shift 2
MODELS=("$@")

IMGS_DIR="$SRC_DIR/tests"
TMP_DIR="$BUILD_DIR/tmp-brunsli"

rm -fr "$TMP_DIR"
mkdir -p "$TMP_DIR/cpp"

# Run original binaries
for f in "$IMGS_DIR"/*.jpg; do
  "$SRC_DIR/src/bin/cbrunsli" "$f" "$TMP_DIR/cpp/$(basename "$f" .jpg).brn" &
done
wait

for f in "$TMP_DIR/cpp"/*.brn; do
  "$SRC_DIR/src/bin/dbrunsli" "$f" "$TMP_DIR/cpp/$(basename "$f" .brn).jpg" &
done
wait

# Run each model and compare against original

for model in "${MODELS[@]}"; do
  RUST_BIN="$SRC_DIR/out/$model/target/release"
  MODEL_DIR="$TMP_DIR/$model"

  rm -fr "$MODEL_DIR"
  mkdir -p "$MODEL_DIR"

  for f in "$IMGS_DIR"/*.jpg; do
    "$RUST_BIN"/cbrunsli "$f" "$MODEL_DIR/$(basename "$f" .jpg).brn" &
  done
  wait

  # Compare brn files against original
  for f in "$MODEL_DIR"/*.brn; do
    base=$(basename "$f")
    diff "$f" "$TMP_DIR/cpp/$base" \
      || { echo "FAIL [$model]: brn mismatch on $base"; exit 1; }
  done

  # Decompress and compare roundtrip
  for f in "$MODEL_DIR"/*.brn; do
    "$RUST_BIN"/dbrunsli "$f" "$MODEL_DIR/$(basename "$f" .brn).jpg" &
  done
  wait

  for f in "$MODEL_DIR"/*.jpg; do
    base=$(basename "$f")
    diff "$f" "$TMP_DIR/cpp/$base" \
      || { echo "FAIL [$model]: jpg mismatch on $base"; exit 1; }
  done

  echo "Brunsli $model tests passed!"
done

rm -fr "$TMP_DIR"
