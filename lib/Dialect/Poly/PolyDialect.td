include "mlir/IR/DialectBase.td"

def PolyDialect : Dialect {
    let name = "poly";
    let summary = "A dialect for polynomial operations";
    let description = [{
        The poly dialect defines types and operations for single-variable
        polynomials over integers.
    }];

  let cppNamespace = "::mlir::tutorial::poly";
}
