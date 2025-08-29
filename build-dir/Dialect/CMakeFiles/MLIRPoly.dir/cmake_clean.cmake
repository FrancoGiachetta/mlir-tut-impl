file(REMOVE_RECURSE
  "libMLIRPoly.a"
  "libMLIRPoly.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/MLIRPoly.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
