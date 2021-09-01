cmake_minimum_required(VERSION 3.21)

project(exercise VERSION 1.0)

# export compile_commands.json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# -std=c++20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 全局依赖
include(FetchContent)

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
FetchContent_Declare(boost
  GIT_REPOSITORY "https://github.com/boostorg/boost.git"
  GIT_TAG        "boost-1.77.0"
)
FetchContent_MakeAvailable(boost)

# 每个子项目列在这里
add_subdirectory(Codewars)
