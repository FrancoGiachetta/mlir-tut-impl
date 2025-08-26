/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* Dialect Definitions                                                        *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|* From: PolyOps.td                                                           *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

MLIR_DEFINE_EXPLICIT_TYPE_ID(::mlir::tutorial::poly::PolyDialect)
namespace mlir {
namespace tutorial {
namespace poly {

PolyDialect::PolyDialect(::mlir::MLIRContext *context)
    : ::mlir::Dialect(getDialectNamespace(), context, ::mlir::TypeID::get<PolyDialect>())
    
     {
  
  initialize();
}

PolyDialect::~PolyDialect() = default;

} // namespace poly
} // namespace tutorial
} // namespace mlir
