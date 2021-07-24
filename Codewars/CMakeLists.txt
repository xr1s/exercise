add_executable(find-the-odd-int "6 kyu/Find the odd int/main.cc")
target_link_libraries(find-the-odd-int igloo)

add_executable(decode-the-morse-code-for-real "2 kyu/Decode the Morse code, for real/main.cc")
target_link_libraries(decode-the-morse-code-for-real igloo range-v3 fmt::fmt)
