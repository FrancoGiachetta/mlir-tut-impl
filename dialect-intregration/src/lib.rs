mod dialect_gen {
    use mlir_sys::MlirDialectRegistry;

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
        pub fn CRegisterDialects(registry: MlirDialectRegistry);
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

        unsafe {
            dialect_gen::CRegisterDialects(registry.to_raw());
        }

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
