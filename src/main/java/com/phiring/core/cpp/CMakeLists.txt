# Minimum version of CMake required for modern NDK features
cmake_minimum_required(VERSION 3.22.1)

# Defines the library name, which Kotlin uses via System.loadLibrary("sss_core")
project(sss_core)

# Defines the target shared library (.so file)
add_library(
    sss_core              # Name of the target (libsss_core.so)
    SHARED                # Specifies a shared library
    sss_core_native.cpp   # Lists the C++ source file(s)
)

# Set the C++ Standard to C++20 for modern features
target_compile_features(sss_core PUBLIC cxx_std_20)

# Apply hardening and optimization flags (MANDATORY for audited crypto)
target_compile_options(sss_core PUBLIC
    -O2
    -fstack-protector-strong
    -D_FORTIFY_SOURCE=2
    -Wformat -Wformat-security -Werror=format-security
    # CRITICAL: Ensures compiler does not remove memory zeroization calls
    -fno-builtin-memset 
)

# Links required Android and system libraries to your shared object.
target_link_libraries(sss_core
    log     # For Android logging (__android_log_print)
    android # For core Android/system APIs
)
