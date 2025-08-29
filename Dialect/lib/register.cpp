#include <mlir/CAPI/IR.h>

#include "Dialect/include/Poly/PolyDialect.h"
#include "Dialect/include/register.h"

static void CPPRegisterDialects(mlir::DialectRegistry &registry) {
  registry.insert<mlir::tutorial::poly::PolyDialect>();
}

void CRegisterDialects(MlirDialectRegistry registry) {
  CPPRegisterDialects(*unwrap(registry));
}
