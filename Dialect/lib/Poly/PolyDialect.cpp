#include <llvm/ADT/TypeSwitch.h>
#include <mlir/IR/Builders.h>

#include "Dialect/include/Poly/PolyDialect.h"
#include "Dialect/include/Poly/PolyOps.h"
#include "Dialect/include/Poly/PolyTypes.h"

#include "Dialect/include/Poly/PolyOpsDialect.cpp.inc"
#define GET_TYPEDEF_CLASSES
#include "Dialect/include/Poly/PolyOpsTypes.cpp.inc"
#define GET_OP_CLASSES
#include "Dialect/include/Poly/PolyOps.cpp.inc"

namespace mlir {
namespace tutorial {
namespace poly {
void PolyDialect::initialize() {
  addTypes<
#define GET_TYPEDEF_LIST
#include "Dialect/include/Poly/PolyOpsTypes.cpp.inc"
      >();
  addOperations<
#define GET_OP_LIST
#include "Dialect/include/Poly/PolyOps.cpp.inc"
      >();
}

Operation *PolyDialect::materializeConstant(OpBuilder &builder, Attribute value,
                                            Type type, Location loc) {
  auto coeffs = dyn_cast<DenseIntElementsAttr>(value);

  if (!coeffs)
    return nullptr;

  return builder.create<ConstantOp>(loc, type, coeffs);
}
} // namespace poly
} // namespace tutorial
} // namespace mlir
