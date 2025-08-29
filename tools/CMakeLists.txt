get_property(dialect_libs GLOBAL PROPERTY MLIR_DIALECT_LIBS)
get_property(conversion_libs GLOBAL PROPERTY MLIR_CONVERSION_LIBS)

set(LIBS
    ${dialect_libs}
    ${conversion_libs}
    MLIROptLib
    MLIRPass
)

add_llvm_executable(tutorial-opt tutorial-opt.cpp)


target_link_libraries(tutorial-opt 
PUBLIC
dialect_bindings
PRIVATE 
${LIBS}
)

llvm_update_compile_flags(tutorial-opt)
mlir_check_all_link_libraries(tutorial-opt)

# # <-- explicitly set rpath on the executable
# set_target_properties(tutorial-opt PROPERTIES
#     BUILD_RPATH "@loader_path/../lib/Dialect"
#     INSTALL_RPATH "@loader_path/../lib/Dialect"
# )
