// #include "lib/Conversion/PolyToStandard/PolyToStandard.h"
// #include "lib/Dialect/Noisy/NoisyDialect.h"
// #include "lib/Dialect/Poly/PolyDialect.h"s
// #include "lib/Transform/Affine/Passes.h"
// #include "lib/Transform/Arith/Passes.h"
// #include "lib/Transform/Noisy/Passes.h"
#include "mlir/Conversion/ArithToLLVM/ArithToLLVM.h"
#include "mlir/Conversion/ControlFlowToLLVM/ControlFlowToLLVM.h"
#include "mlir/Conversion/FuncToLLVM/ConvertFuncToLLVMPass.h"
#include "mlir/Conversion/SCFToControlFlow/SCFToControlFlow.h"
#include "mlir/Conversion/TensorToLinalg/TensorToLinalgPass.h"
#include "mlir/Dialect/Bufferization/Pipelines/Passes.h"
#include "mlir/Dialect/Bufferization/Transforms/Passes.h"
#include "mlir/Dialect/Linalg/Passes.h"
#include "mlir/InitAllDialects.h"
#include "mlir/InitAllPasses.h"
#include "mlir/Pass/PassManager.h"
#include "mlir/Pass/PassRegistry.h"
#include "mlir/Tools/mlir-opt/MlirOptMain.h"
#include "mlir/Transforms/Passes.h"

void polyToLLVMPipelineBuilder(mlir::OpPassManager &manager) {
}

int main(int argc, char **argv) {
  mlir::DialectRegistry registry;
  // registry.insert<mlir::tutorial::poly::PolyDialect>();
  mlir::registerAllDialects(registry);
  mlir::registerAllPasses();

  return 0;
}
