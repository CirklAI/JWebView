cmake_minimum_required(VERSION 3.31)
project(JWebView)
include(FetchContent)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")

find_package(JNI REQUIRED)
find_package(Threads REQUIRED)

FetchContent_Declare(
        webview
        GIT_REPOSITORY https://github.com/webview/webview
        GIT_TAG 0.12.0)
FetchContent_MakeAvailable(webview)

include_directories(include)
include_directories(${JNI_INCLUDE_DIRS})
add_library(JWebView SHARED src/library.cpp)
target_link_libraries(JWebView PRIVATE webview::core)
target_link_libraries(JWebView PRIVATE Threads::Threads)