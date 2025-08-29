add_subdirectory(lib)
add_subdirectory(include)

# Create a library associated to the dialect
add_mlir_dialect_library(MLIRPoly
    ${PROJECT_SOURCE_DIR}/Dialect/lib/Poly/PolyDialect.cpp
    ${PROJECT_SOURCE_DIR}/Dialect/lib/Poly/PolyOps.cpp

    ADDITIONAL_HEADER_DIRS
    ${PROJECT_SOURCE_DIR}/Dialect/include/Poly

    LINK_LIBS PUBLIC
    )

install(TARGETS MLIRPoly LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})

add_library(dialect_bindings SHARED lib/register.cpp)

target_include_directories(dialect_bindings
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR/include/register.h}
)

target_link_libraries(dialect_bindings
    PUBLIC
        MLIRIR
        MLIRPoly
    PRIVATE
        MLIRSupport
)

install(TARGETS dialect_bindings LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
