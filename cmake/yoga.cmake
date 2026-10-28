set(YOGA_SRC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/yoga/src")
set(YOGA_OUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/yoga/out")
set(YOGA_GTEST_DIR "${CMAKE_CURRENT_SOURCE_DIR}/yoga/gtest")

set(YOGA_BENCHMARK_FILTER       /src/yoga/ Benchmark.cpp TreeDeserialization.cpp)
set(YOGA_BENCHMARKLEGACY_FILTER /src/yoga/ YGBenchmark.c)

add_custom_target("build-yoga-original"
  COMMAND ${CMAKE_COMMAND} -S ${YOGA_SRC_DIR}/benchmark
                           -B ${YOGA_SRC_DIR}/build
                           -G Ninja
                           -DCMAKE_BUILD_TYPE=Release
                           -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
  COMMAND ${CMAKE_COMMAND} --build ${YOGA_SRC_DIR}/build
  COMMAND ${CMAKE_COMMAND} -S ${YOGA_GTEST_DIR}
                           -B ${YOGA_GTEST_DIR}/build
                           -G Ninja
                           -DCMAKE_BUILD_TYPE=Release
                           -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
  COMMAND ${CMAKE_COMMAND} --build ${YOGA_GTEST_DIR}/build
  WORKING_DIRECTORY "${YOGA_SRC_DIR}"
)

add_custom_target("config-yoga"
  COMMAND ${CMAKE_COMMAND} -E make_directory benchmark_tu benchmarklegacy_tu
  COMMAND python3 ${FILTER_SCRIPT} compile_commands.json benchmark_tu/compile_commands.json
          ${YOGA_BENCHMARK_FILTER}
  COMMAND python3 ${FILTER_SCRIPT} compile_commands.json benchmarklegacy_tu/compile_commands.json
          ${YOGA_BENCHMARKLEGACY_FILTER}
  COMMAND ${CMAKE_COMMAND} -E make_directory ${YOGA_GTEST_DIR}/build/yogatests_tu
  COMMAND ${CMAKE_COMMAND} -E copy ${YOGA_GTEST_DIR}/build/compile_commands.json
          ${YOGA_GTEST_DIR}/build/yogatests_tu/compile_commands.json
  WORKING_DIRECTORY ${YOGA_SRC_DIR}/build
  DEPENDS "build-yoga-original"
)

foreach(_model IN LISTS CPP2RUST_MODELS)
  add_custom_target("regen-yoga-${_model}"
    COMMAND ${CPP2RUST_BINARY} -dir ${YOGA_SRC_DIR}/build/benchmark_tu -model ${_model}
            -o ${YOGA_OUT_DIR}/${_model}/src/bin/benchmark.rs
    COMMAND ${CPP2RUST_BINARY} -dir ${YOGA_SRC_DIR}/build/benchmarklegacy_tu -model ${_model}
            -o ${YOGA_OUT_DIR}/${_model}/src/bin/benchmarklegacy.rs
    COMMAND ${CPP2RUST_BINARY} -dir ${YOGA_GTEST_DIR}/build/yogatests_tu -model ${_model}
            -o ${YOGA_OUT_DIR}/${_model}/src/bin/yogatests.rs
    DEPENDS "config-yoga"
  )
endforeach()
