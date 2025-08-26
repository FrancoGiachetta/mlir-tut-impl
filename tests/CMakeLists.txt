configure_lit_site_cfg(
    ${CMAKE_CURRENT_SOURCE_DIR}/lit.cmake.site.cfg.py.in
    ${CMAKE_CURRENT_BINARY_DIR}/lit.site.cfg.py
    MAIN_CONFIG
    ${CMAKE_CURRENT_SOURCE_DIR}/lit.cmake.cfg.py
)

set(MLIR_TUTORIAL_TEST_DEPENDS
    FileCheck count not
    mlir-opt
    mlir-cpu-runner
)

set(LLVM_EXTERNAL_LIT /opt/homebrew/opt/lit/bin/lit CACHE STRING "Description" FORCE)

add_lit_testsuite(check-mlir-tutorial "Running the MLIR tutorial regression tests"
    ${CMAKE_CURRENT_BINARY_DIR}
    DEPENDS ${MLIR_TUTORIAL_TEST_DEPENDS}
)
