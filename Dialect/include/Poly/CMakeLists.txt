# This mlir_tablegen(...) is analogus to tablegen(...) for LLVM. It tells TableGen
# to generate a Poly*.inc from a .td file. It allows to customize the building process.
# set(LLVM_TARGET_DEFINITIONS PolyOps.td)
# mlir_tablegen(PolyOps.h.inc -gen-op-decls)
# mlir_tablegen(PolyOps.cpp.inc -gen-op-defs)
# mlir_tablegen(PolyTypes.h.inc -gen-typedef-decls -typedefs-dialect=poly)
# mlir_tablegen(PolyTypes.cpp.inc -gen-typedef-defs -typedefs-dialect=poly)
# mlir_tablegen(PolyDialect.h.inc -gen-dialect-decls -dialect=poly)
# mlir_tablegen(PolyDialect.cpp.inc -gen-dialect-defs -dialect=poly)
# add_public_tablegen_target(MLIRPolyOpsIncGen)
# add_dependencies(mlir-headers MLIRPolyOpsIncGen)

# This is a more cleaner way of generating the Dialect files, though not very customizable. 
# It does the same as the instructions above.
add_mlir_dialect(PolyOps poly)

# Creates docs for every file generated
add_mlir_doc(PolyDialect PolyDialect Poly/ -gen-dialect-doc)

# set(LLVM_TARGET_DEFINITIONS PolyPatterns.td)
# mlir_tablegen(PolyCanonicalize.cpp.inc -gen-rewriters)
# add_public_tablegen_target(MLIRPolyCanonicalizationIncGen)
