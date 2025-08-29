cmake_minimum_required(VERSION 3.20.0)

project(mlir-tutorial-impl LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17 CACHE STRING "C++ standard to conform to")

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(BUILD_DEPS ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(BinDir "${CMAKE_SOURCE_DIR}/bin")
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${BinDir}/static-libs")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${BinDir}/shared-libs")

# This variable defines where to find MLIRConfig.cmake. It can be modified when building
# LLVM sources. Since I have installed it with brew, I don't have it set.
set(MLIR_DIR /opt/homebrew/opt/llvm@19/lib/cmake/mlir)

# this will find the config needed for MLIR and also for LLVM. 
find_package(MLIR REQUIRED CONFIG)

message(STATUS "Using MLIRConfig.cmake in: ${MLIR_DIR}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

set(MLIR_BINARY_DIR ${CMAKE_BINARY_DIR})
list(APPEND CMAKE_MODULE_PATH "${MLIR_CMAKE_DIR}")

# this will load some usefull cmake functions like: add_mlir_dialect, mlir_tablegen, etc; which
# will help us build the dialect's sources.
include(AddLLVM)
include(TableGen)
include(AddMLIR)

include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${MLIR_INCLUDE_DIRS})
include_directories(${PROJECT_SOURCE_DIR})
include_directories(${PROJECT_BINARY_DIR})

# Include CMakeLists.txt from these subdirectories
add_subdirectory(Dialect)
add_subdirectory(tests)
add_subdirectory(tools)
