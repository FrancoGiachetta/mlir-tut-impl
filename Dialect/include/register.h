#ifndef DIALECT_REGISTER_H
#define DIALECT_REGISTER_H

#include <mlir-c/IR.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Appends all upstream dialects and extensions to the dialect registry.
MLIR_CAPI_EXPORTED void registerCustomDialects(MlirDialectRegistry registry);
/// Creates a Poly type.
MLIR_CAPI_EXPORTED MlirType getPolynomialType(MlirContext context,
                                              uint32_t degree);

#ifdef __cplusplus
}
#endif

#endif
