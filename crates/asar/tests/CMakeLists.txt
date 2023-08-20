cmake_minimum_required(VERSION 3.20.0)

project(asar_test)

add_executable(asar_test "main.c")
target_link_directories(asar_test PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/../../../target/debug/")

if(MSVC)
target_link_libraries(asar_test PRIVATE
  "asar_rs"
  "Bcrypt"
  "Ws2_32"
  "Userenv"
  "NtDll"
)
else()
target_link_libraries(asar_test PRIVATE
  "asar_rs"
)
endif()
