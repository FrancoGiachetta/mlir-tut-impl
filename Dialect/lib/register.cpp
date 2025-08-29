#include "mlir-c/RegisterEverything.h"
#include "mlir/CAPI/IR.h" // for unwrap()
#include "mlir/InitAllDialects.h"

#include "lib/Dialect/Poly/PolyDialect.h"
#include "register.h"

static void CPPRegisterDialects(mlir::DialectRegistry &registry) {
  registry.insert<mlir::tutorial::poly::PolyDialect>();
}

void CRegisterDialects(MlirDialectRegistry registry) {
  CPPRegisterDialects(*unwrap(registry));
}
