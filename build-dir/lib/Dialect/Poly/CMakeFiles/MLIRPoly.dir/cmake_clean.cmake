file(REMOVE_RECURSE
  "PolyDialect.md"
  "PolyOps.cpp.inc"
  "PolyOps.h.inc"
  "PolyOpsDialect.cpp.inc"
  "PolyOpsDialect.h.inc"
  "PolyOpsTypes.cpp.inc"
  "PolyOpsTypes.h.inc"
  "libMLIRPoly.a"
  "libMLIRPoly.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/MLIRPoly.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
