#include <mlir/Dialect/CommonFolders.h>

#include "Dialect/include/Poly/PolyOps.h"

namespace mlir {
namespace tutorial {
namespace poly {

OpFoldResult ConstantOp::fold(ConstantOp::FoldAdaptor adaptor) {
  return adaptor.getCoefficients();
}

OpFoldResult AddOp::fold(AddOp::FoldAdaptor adaptor) {
  return constFoldBinaryOp<IntegerAttr, APInt, void>(
      adaptor.getOperands(), [&](APInt a, APInt b) { return a + b; });
}

OpFoldResult SubOp::fold(SubOp::FoldAdaptor adaptor) {
  return constFoldBinaryOp<IntegerAttr, APInt, void>(
      adaptor.getOperands(), [&](APInt a, APInt b) { return a - b; });
}

OpFoldResult MulOp::fold(MulOp::FoldAdaptor adaptor) {
  auto lhs = dyn_cast_or_null<DenseIntElementsAttr>(adaptor.getOperands()[0]);
  auto rhs = dyn_cast_or_null<DenseIntElementsAttr>(adaptor.getOperands()[1]);

  if (!lhs || !rhs)
    return nullptr;

  auto degree =
      llvm::cast<PolynomialType>(getResult().getType()).getDegreeBound();
  auto maxIndex = lhs.size() + rhs.size();

  SmallVector<APInt, 8> result;
  result.reserve(maxIndex);

  for (int i = 0; i < maxIndex; ++i)
    result.push_back(APInt((*lhs.begin()).getBitWidth(), 0));

  int i = 0;
  for (auto lhsIt = lhs.value_begin<APInt>(); lhsIt != lhs.value_end<APInt>();
       ++lhsIt) {
    int j = 0;
    for (auto rhsIt = rhs.value_begin<APInt>(); rhsIt != rhs.value_end<APInt>();
         ++rhsIt) {
      // To avoid overflows, reduce the resulting degree so that x^n = 1;
      result[(i + j) % degree] = *rhsIt * (*lhsIt);
      ++j;
    }
    ++i;
  }

  return DenseIntElementsAttr::get(
      RankedTensorType::get(static_cast<int64_t>(result.size()),
                            IntegerType::get(getContext(), 32)),
      result);
}

OpFoldResult FromTensorOp::fold(FromTensorOp::FoldAdaptor adaptor) {
  return dyn_cast_or_null<DenseIntElementsAttr>(adaptor.getInput());
}
} // namespace poly
} // namespace tutorial
} // namespace mlir
