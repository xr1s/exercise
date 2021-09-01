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
    "-DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/_deps/igloo-build"
    "-Wno-dev"
    "."
  WORKING_DIRECTORY "${CMAKE_BINARY_DIR}/_deps/igloo-src"
)
add_library(igloo INTERFACE IMPORTED)
set_target_properties(igloo PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_BINARY_DIR}/_deps/igloo-build/include"
)
execute_process(
  COMMAND
    "${CMAKE_COMMAND}"
    "--build"
    "."
    "--target"
    "install"
    "--parallel"
  WORKING_DIRECTORY "${CMAKE_BINARY_DIR}/_deps/igloo-src"
)

add_executable(find-the-odd-int "6 kyu/Find the odd int/main.cc")
target_link_libraries(find-the-odd-int igloo)

add_executable(decode-the-morse-code-for-real "2 kyu/Decode the Morse code, for real/main.cc")
target_link_libraries(decode-the-morse-code-for-real igloo range-v3 fmt::fmt)
