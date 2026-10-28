#!/bin/bash

set -eo pipefail

# Usage: yoga-test.sh <build_dir> <source_dir> <model...>
BUILD_DIR="$1"
SRC_DIR="$2/yoga"
shift 2
MODELS=("$@")

TMP_DIR="$BUILD_DIR/tmp-yoga"

strip_numbers() {
  sed -E 's/[0-9]+\.[0-9]+ ms/X ms/g'
}

run_all() {
  local bin_dir="$1" out_dir="$2" tests_bin="$3"
  mkdir -p "$out_dir"
  "$tests_bin" > "$out_dir/yogatests.out"
  "$bin_dir/benchmarklegacy" | strip_numbers > "$out_dir/benchmarklegacy.out"
}

rm -fr "$TMP_DIR"

run_all "$SRC_DIR/src/build" "$TMP_DIR/original" "$SRC_DIR/gtest/build/yogatests" \
  || { echo "FAIL: cpp yoga"; exit 1; }

for model in "${MODELS[@]}"; do
  RUST_BIN="$SRC_DIR/out/$model/target/release"
  run_all "$RUST_BIN" "$TMP_DIR/$model" "$RUST_BIN/yogatests" \
    || { echo "FAIL [$model]: yoga crashed"; exit 1; }

  diff -r "$TMP_DIR/original" "$TMP_DIR/$model" \
    || { echo "FAIL [$model]: output mismatch"; exit 1; }

  echo "Yoga $model tests passed!"
done

rm -fr "$TMP_DIR"
