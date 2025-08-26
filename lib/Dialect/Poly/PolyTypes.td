#ifndef LIB_DIALECT_POLY_POLYTYPES_TD_
#define LIB_DIALECT_POLY_POLYTYPES_TD_

include "PolyDialect.td"
include "mlir/IR/AttrTypeBase.td"

// * mnemonic, and so TypeMnemonic, represents the string that will be used for the IR representation of the type.
// * name is a suffix used in the name of the generated class. In this case, <suffix>Type
class PolyType<string name, string TypeMnemonic> : TypeDef<PolyDialect, name> {
    let mnemonic = TypeMnemonic;
}

def Poly : PolyType<"Polynomial", "poly"> {
    let summary = "A polinomial with i32 coefficients";
    let description = [{
        A type for polynomials with integer coefficients in a single-variable polynomial ring.
    }];
    let parameters = (ins "int":$degreeBound);
    let assemblyFormat = "`<`$degreeBound`>`";
}

#endif 
