#!/bin/bash

set -e

# Usage: woff2-bench.sh <build_dir> <source_dir> <model...>
BUILD_DIR="$1"
SRC_DIR="$2/woff2"
shift 2
MODELS=("$@")

FONTS_DIR="$SRC_DIR/tests"
TMP_DIR="$BUILD_DIR/tmp-woff2-bench"

if ! command -v hyperfine >/dev/null 2>&1; then
  echo "error: hyperfine not found; install it to run benchmarks" >&2
  exit 1
fi

. "$(dirname "$0")/woff2-common.sh"
setup_woff2_tmp

bench_impl() {
  local label=$1 bin_dir=$2 work_dir=$3
  local op ext prepare cmd

  rm -fr "$work_dir"
  mkdir -p "$work_dir"

  for op in compress decompress; do
    case $op in
      compress)   ext=ttf   ;;
      decompress) ext=woff2 ;;
    esac

    prepare=$(printf 'rm -f "%s"/*.ttf "%s"/*.woff2 && cp "%s"/*.%s "%s"/' \
                     "$work_dir" "$work_dir" "$TMP_DIR/original" "$ext" "$work_dir")
    cmd=$(printf 'set -e; for f in "%s"/*.%s; do "%s/woff2_%s" "$f"; done' \
                 "$work_dir" "$ext" "$bin_dir" "$op")

    echo "=== WOFF2 $op ($label) ==="
    hyperfine --runs 3 \
      --export-csv "$BUILD_DIR/bench-woff2-$op-$label.csv" \
      --prepare "$prepare" \
      "$cmd"
  done
}

bench_impl original "$SRC_DIR/src" "$TMP_DIR/cc"

for model in "${MODELS[@]}"; do
  bench_impl "$model" "$SRC_DIR/out/$model/target/release" "$TMP_DIR/$model"
done

rm -fr "$TMP_DIR"
