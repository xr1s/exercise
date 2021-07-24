cmake_minimum_required(VERSION 3.21)

project(exercise VERSION 1.0)

# export compile_commands.json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# -std=c++20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 全局依赖
include(FetchContent)

# igloo 是 Codewars 的测试框架
FetchContent_Declare(igloo
  GIT_REPOSITORY "https://github.com/codewars/igloo.git"
  GIT_TAG        "codewars"
)
FetchContent_GetProperties(igloo)
if (NOT igloo_POPULATED)
  FetchContent_Populate(igloo)
endif()
execute_process(
  COMMAND
    "${CMAKE_COMMAND}"
    "-DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/_deps/igloo-build"
    "-Wno-dev"
    "."
  WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/_deps/igloo-src"
)
add_library(igloo INTERFACE IMPORTED)
set_target_properties(igloo PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_BINARY_DIR}/_deps/igloo-build/include"
)
execute_process(
  COMMAND
    "${CMAKE_COMMAND}"
    "--build"
    "."
    "--target"
    "install"
    "--parallel"
  WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/_deps/igloo-src"
)

# range-v3
FetchContent_Declare(range-v3
  GIT_REPOSITORY  "https://github.com/ericniebler/range-v3.git"
  GIT_TAG         "0.11.0"
)
FetchContent_MakeAvailable(range-v3)

# fmtlib
FetchContent_Declare(fmtlib
  GIT_REPOSITORY  "https://github.com/fmtlib/fmt.git"
  GIT_TAG         "8.0.1"
)
FetchContent_MakeAvailable(fmtlib)

# Boost
FetchContent_Declare(boost.type_index
  GIT_REPOSITORY "https://github.com/boostorg/type_index.git"
  GIT_TAG        "boost-1.76.0"
)
FetchContent_MakeAvailable(boost.type_index)

# 每个子项目列在这里
add_subdirectory(Codewars)
