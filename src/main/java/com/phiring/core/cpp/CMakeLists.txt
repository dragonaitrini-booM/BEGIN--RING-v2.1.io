cmake_minimum_required(VERSION 3.22.1)

project(sss_core)

add_library(
    sss_core
    SHARED
    sss_core_native.cpp
)

target_compile_features(sss_core PUBLIC cxx_std_20)

# CRITICAL SECURITY FLAGS
target_compile_options(sss_core PUBLIC
    -O2
    -fstack-protector-strong
    -D_FORTIFY_SOURCE=2
    -Wformat -Wformat-security -Werror=format-security
    -fno-builtin-memset 
)

target_link_libraries(sss_core
    log
    android
)
