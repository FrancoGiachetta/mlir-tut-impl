#include <mlir/CAPI/Wrap.h>
#include <mlir/InitAllDialects.h>
#include <mlir/InitAllPasses.h>
#include <mlir/Tools/mlir-opt/MlirOptMain.h>
#include <mlir/Transforms/Passes.h>

#include "Dialect/include/Poly/PolyDialect.h"

int main(int argc, char **argv) {
  mlir::DialectRegistry registry;

  registry.insert<mlir::tutorial::poly::PolyDialect>();
  // CRegisterDialects(wrap(&registry));

  mlir::registerAllDialects(registry);
  mlir::registerAllPasses();

  return mlir::asMainReturnCode(
      mlir::MlirOptMain(argc, argv, "Tutorial Pass Driver", registry));
}
