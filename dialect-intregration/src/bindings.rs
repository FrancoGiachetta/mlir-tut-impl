pub mod dialect_gen {
    use melior::{
        dialect::DialectRegistry, ir::{r#type::IntegerType, Type, TypeLike}, Context
    };
    use mlir_sys::{MlirContext, MlirDialectRegistry, MlirType, mlirUnrankedTensorTypeGet};

    melior::dialect! {
        name: "poly",
        files: [
            "PolyDialect.td",
            "PolyOps.td",
            "PolyTypes.td",
        ],
        include_directories: ["/Users/franco/Documents/Projects/mlir-tut-impl/Dialect/include/Poly/"]
    }

    #[link(name = "dialect_bindings")]
    unsafe extern "C" {
        /// Register dialects from cpp.
        pub fn registerCustomDialects(registry: MlirDialectRegistry);
        /// Create a Polynomial type from cpp.
        pub fn getPolynomialType(context: MlirContext, degree: i32) -> MlirType;
    }

    // Safe wrapper to register a dialect.
    pub fn register_custom_dialects(registry: &DialectRegistry) {
        unsafe {registerCustomDialects(registry.to_raw());}
    }

    /// Safe wrapper to create a polynomial type.
    pub fn polynomial<'c>(context: &'c Context, degree: i32) -> Type<'c> {
        unsafe { Type::from_raw(getPolynomialType(context.to_raw(), degree)) }
    }

    pub fn tensor<'c>(context: &'c Context) -> Type<'c> {
        unsafe {
            Type::from_raw(mlirUnrankedTensorTypeGet(
                IntegerType::new(context, 32).to_raw(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use melior::{
        Context,
        dialect::DialectRegistry,
        utility::{register_all_dialects, register_all_llvm_translations},
    };

    use crate::dialect_gen;

    pub fn load_all_dialects(context: &Context) {
        let registry = DialectRegistry::new();
        register_all_dialects(&registry);

        // register poly dialect
        dialect_gen::register_custom_dialects(&registry);

        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();
    }

    pub fn create_test_context() -> Context {
        let context = Context::new();

        context.attach_diagnostic_handler(|diagnostic| {
            eprintln!("{diagnostic}");
            true
        });

        load_all_dialects(&context);
        register_all_llvm_translations(&context);

        context
    }

    #[test]
    fn test_poly_is_loaded() {
        let ctx = create_test_context();

        let count = ctx.registered_dialect_count();

        assert_eq!(count, 48);
    }
}
