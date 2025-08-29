#ifndef DIALECT_REGISTER_H
#define DIALECT_REGISTER_H

#include "mlir-c/IR.h"

#ifdef __cplusplus
extern "C" {
#endif

/// Appends all upstream dialects and extensions to the dialect registry.
MLIR_CAPI_EXPORTED void CRegisterDialects(MlirDialectRegistry registry);

#ifdef __cplusplus
}
#endif

#endif
