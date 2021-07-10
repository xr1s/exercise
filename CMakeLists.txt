cmake_minimum_required(VERSION 3.21)

project(exercise VERSION 1.0)

# export compile_commands.json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# -std=c++20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 列出所有的产物 (用于增加全局依赖)
function(get_all_targets _result _dir)
  get_property(_subdirs DIRECTORY "${_dir}" PROPERTY SUBDIRECTORIES)
  foreach(_subdir IN LISTS _subdirs)
    get_all_targets(${_result} "${_subdir}")
  endforeach()
  get_directory_property(_sub_targets DIRECTORY "${_dir}" BUILDSYSTEM_TARGETS)
  set(${_result} ${${_result}} ${_sub_targets} PARENT_SCOPE)
endfunction()

# 全局依赖
include(ExternalProject)
ExternalProject_Add(igloo
  GIT_REPOSITORY "https://github.com/codewars/igloo.git"
  GIT_TAG        "codewars"
  TEST_COMMAND   ""
  CMAKE_ARGS
    "-DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/third_party/igloo"
)
include_directories("${CMAKE_CURRENT_BINARY_DIR}/third_party/igloo/include")
list(APPEND dependencies "igloo")

# Codewars 使用的是 C++17，range 此时还未入标准库
# 能用 C++20 标准库 <ranges> 的地方就用 C++20
ExternalProject_Add(range-v3
  GIT_REPOSITORY  "https://github.com/ericniebler/range-v3"
  INSTALL_DIR     "${CMAKE_CURRENT_BINARY_DIR}/third_party/range-v3"
  CMAKE_ARGS
    "-DRANGE_V3_TESTS=OFF"
    "-DRANGE_V3_EXAMPLES=OFF"
    "-DRANGE_V3_PERF=OFF"
    "-DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/third_party/range-v3"
)
include_directories("${CMAKE_CURRENT_BINARY_DIR}/third_party/range-v3/include")
list(APPEND dependencies "range-v3")

# 每个子项目列在这里
add_subdirectory(Codewars)

# 为所有产物增加全局依赖
get_all_targets(targets "${CMAKE_CURRENT_SOURCE_DIR}")
foreach(target ${targets})
  if (NOT "${target}" IN_LIST dependencies)
    add_dependencies(${target} ${dependencies})
  endif()
endforeach()
