#ifndef LIB_DIALECT_POLY_POLYOPS_TD_
#define LIB_DIALECT_POLY_POLYOPS_TD_

include "mlir/IR/OpBase.td"
include "PolyTypes.td"

def Poly_AddOp : Op<PolyDialect, "add"> {
  let summary = "Addition operation between polynomials.";
  let arguments = (ins Poly:$lhs, Poly:$rhs);
  let results = (outs Poly:$output);
  let assemblyFormat = "$lhs `,` $rhs attr-dict `:` `(` type($lhs) `,` type($rhs) `)` `->` type($output)";
}

#endif  // LIB_DIALECT_POLY_POLYOPS_TD_
