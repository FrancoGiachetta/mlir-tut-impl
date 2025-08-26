#ifndef LIB_DIALECT_POLY_POLYOPS_TD_
#define LIB_DIALECT_POLY_POLYOPS_TD_

include "mlir/IR/OpBase.td"
include "PolyTypes.td"
include "mlir/Interfaces/SideEffectInterfaces.td"

// Type constraint for poly binop arguments: polys, vectors of polys, or
// tensors of polys.
def PolyOrContainer : TypeOrContainer<Poly, "poly-or-container">;

class PolyBinOp<string mnemonic> : Op<PolyDialect, mnemonic, [Pure, ElementwiseMappable, SameOperandsAndResultElementType]> {
  let summary = "Addition operation between polynomials.";
  let arguments = (ins Poly:$lhs, Poly:$rhs);
  let results = (outs Poly:$output);
  let assemblyFormat = "$lhs `,` $rhs attr-dict `:` `(` type($lhs) `,` type($rhs) `)` `->` type($output)";
}

def PolyAddOp : PolyBinOp<"add"> {
  let summary = "Addition operation between polynomials.";
}

def PolySubOp : PolyBinOp<"sub"> {
  let summary = "Subtraction operation between polynomials.";
}

def PolyMulOp : PolyBinOp<"mul"> {
  let summary = "Subtraction operation between polynomials.";
}

def PolyFromTensorOp : Op<PolyDialect, "from_tensor", [Pure]> {
  let summary = "Creates a Polynomial from integer coefficients stored in a tensor.";
  let arguments = (ins TensorOf<[AnyInteger]>:$input);
  let results = (outs Poly:$output);
  let assemblyFormat = "$input attr-dict `:` type($input) `->` type($output)";
}

def Poly_EvalOp : Op<PolyDialect, "eval"> {
  let summary = "Evaluates a Polynomial at a given input value.";
  let arguments = (ins Poly:$input, AnyInteger:$point);
  let results = (outs AnyInteger:$output);
  let assemblyFormat = "$input `,` $point attr-dict `:` `(` type($input) `,` type($point) `)` `->` type($output)";
}
#endif  // LIB_DIALECT_POLY_POLYOPS_TD_
