set(WOFF2_SRC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/woff2/src")
set(WOFF2_OUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/woff2/out")

set(WOFF2_INFO_FILES       font.cc table_tags.cc variable_length.cc
                           woff2_common.cc woff2_info.cc)
set(WOFF2_COMPRESS_FILES   table_tags.cc variable_length.cc woff2_common.cc
                           font.cc glyph.cc normalize.cc transform.cc
                           woff2_enc.cc woff2_compress.cc)
set(WOFF2_DECOMPRESS_FILES table_tags.cc variable_length.cc woff2_common.cc
                           woff2_dec.cc woff2_out.cc woff2_decompress.cc)

add_custom_target("build-woff2-original"
  COMMAND ${CMAKE_COMMAND} -S ${WOFF2_SRC_DIR}
                           -B ${WOFF2_SRC_DIR}/build
                           -G Ninja
                           -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
  # Only the Makefile build links against the bundled Brotli
  # Important so the compressed files match perfectly
  COMMAND make
  WORKING_DIRECTORY "${WOFF2_SRC_DIR}"
)

add_custom_target("config-woff2"
  # Filter: split compile_commands.json per binary
  COMMAND ${CMAKE_COMMAND} -E make_directory
          ${WOFF2_SRC_DIR}/build/cmd_woff2_info
          ${WOFF2_SRC_DIR}/build/cmd_woff2_compress
          ${WOFF2_SRC_DIR}/build/cmd_woff2_decompress
  COMMAND python3 ${FILTER_SCRIPT}
          ${WOFF2_SRC_DIR}/build/compile_commands.json
          ${WOFF2_SRC_DIR}/build/cmd_woff2_info/compile_commands.json
          ${WOFF2_INFO_FILES}
  COMMAND python3 ${FILTER_SCRIPT}
          ${WOFF2_SRC_DIR}/build/compile_commands.json
          ${WOFF2_SRC_DIR}/build/cmd_woff2_compress/compile_commands.json
          ${WOFF2_COMPRESS_FILES}
  COMMAND python3 ${FILTER_SCRIPT}
          ${WOFF2_SRC_DIR}/build/compile_commands.json
          ${WOFF2_SRC_DIR}/build/cmd_woff2_decompress/compile_commands.json
          ${WOFF2_DECOMPRESS_FILES}
  DEPENDS "build-woff2-original"
)

foreach(_model IN LISTS CPP2RUST_MODELS)
  add_custom_target("regen-woff2-${_model}"
    COMMAND ${CPP2RUST_BINARY} -dir ${WOFF2_SRC_DIR}/build/cmd_woff2_info
            -model ${_model}
            -o ${WOFF2_OUT_DIR}/${_model}/src/bin/woff2_info.rs
    COMMAND ${CPP2RUST_BINARY} -dir ${WOFF2_SRC_DIR}/build/cmd_woff2_compress
            -model ${_model}
            -o ${WOFF2_OUT_DIR}/${_model}/src/bin/woff2_compress.rs
    COMMAND ${CPP2RUST_BINARY} -dir ${WOFF2_SRC_DIR}/build/cmd_woff2_decompress
            -model ${_model}
            -o ${WOFF2_OUT_DIR}/${_model}/src/bin/woff2_decompress.rs
    DEPENDS "config-woff2"
  )
endforeach()
