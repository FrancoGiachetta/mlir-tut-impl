use dialect_intregration::{build_module, initialize_context};

fn main() {
    let ctx = initialize_context();
    let module = build_module(&ctx);

    std::fs::write("mlir-prepass.mlir", module.as_operation().to_string()).unwrap();
}
