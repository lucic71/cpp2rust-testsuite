setup_woff2_tmp() {
  [ -n "$TMP_DIR" ]   || { echo "setup_woff2_tmp: TMP_DIR not set" >&2; exit 1; }
  [ -n "$SRC_DIR" ]   || { echo "setup_woff2_tmp: SRC_DIR not set" >&2; exit 1; }
  [ -n "$FONTS_DIR" ] || { echo "setup_woff2_tmp: FONTS_DIR not set" >&2; exit 1; }

  rm -fr "$TMP_DIR"
  mkdir -p "$TMP_DIR/original"

  for f in "$FONTS_DIR"/**/*.ttf; do
    cp "$f" "$TMP_DIR/original"
  done

  # Run original binaries
  for f in "$TMP_DIR"/original/*.ttf; do
    "$SRC_DIR/src/woff2_compress" "$f" &
  done
  wait

  for f in "$TMP_DIR"/original/*.woff2; do
    "$SRC_DIR/src/woff2_info" "$f" | tail -n +2 > "$TMP_DIR/original/$(basename "$f" .woff2).info"
  done

  mkdir -p "$TMP_DIR/cc-decompressed"
  cp "$TMP_DIR"/original/*.woff2 "$TMP_DIR/cc-decompressed/"
  for f in "$TMP_DIR/cc-decompressed"/*.woff2; do
    "$SRC_DIR/src/woff2_decompress" "$f" &
  done
  wait
}
