#include <cstdint>
#include <mlir/CAPI/IR.h>

#include "Dialect/include/Poly/PolyDialect.h"
#include "Dialect/include/Poly/PolyTypes.h"
#include "Dialect/include/register.h"

using namespace mlir::tutorial;
using namespace poly;

static void CPPRegisterDialects(mlir::DialectRegistry &registry) {
  registry.insert<mlir::tutorial::poly::PolyDialect>();
}

void registerCustomDialects(MlirDialectRegistry registry) {
  CPPRegisterDialects(*unwrap(registry));
}

MlirType getPolynomialType(MlirContext context, uint32_t degree) {
  return wrap(PolynomialType::get(unwrap(context), degree));
}
