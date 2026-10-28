cmake_minimum_required(VERSION 3.13...3.26)
project(yogatests)

set(YOGA_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../src)
include(${YOGA_ROOT}/cmake/project-defaults.cmake)

add_subdirectory(${YOGA_ROOT}/yoga ${CMAKE_CURRENT_BINARY_DIR}/yoga)

file(GLOB SOURCES CONFIGURE_DEPENDS
    ${YOGA_ROOT}/tests/*.cpp
    ${YOGA_ROOT}/tests/**/*.cpp)

list(FILTER SOURCES EXCLUDE REGEX "/tests/(CacheTest|EventPublisherTest|EventsTest|YGConfigTest|YGFlexBasisFitContentTest|YGHadOverflowTest|YGPersistentNodeCloningTest)\\.cpp$")

set(SHIM_INCLUDE ${CMAKE_CURRENT_SOURCE_DIR}/include)
set(MAIN_CPP ${CMAKE_CURRENT_BINARY_DIR}/main.cpp)

add_custom_command(
  OUTPUT ${MAIN_CPP}
  COMMAND python3 ${CMAKE_CURRENT_SOURCE_DIR}/gen_main.py
          ${CMAKE_CXX_COMPILER} ${MAIN_CPP} "${SHIM_INCLUDE}\;${YOGA_ROOT}" ${SOURCES}
  DEPENDS ${SOURCES} ${CMAKE_CURRENT_SOURCE_DIR}/gen_main.py ${SHIM_INCLUDE}/gtest/gtest.h
)

add_executable(yogatests ${SOURCES} ${MAIN_CPP})
target_include_directories(yogatests PRIVATE ${SHIM_INCLUDE})
target_link_libraries(yogatests yogacore)
