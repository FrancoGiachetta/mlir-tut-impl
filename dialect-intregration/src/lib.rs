mod dialect_gen {
    melior::dialect! {
        name: "poly",
        files: [
            "PolyDialect.td",
            "PolyOps.td",
            "PolyTypes.td",
        ],
        include_directories: ["mlir/Dialect/Poly"],
    }
}

// #[cfg(test)]
// mod tests {
//     use melior::{
//         dialect::{
//             arith, ods::{func}, DialectRegistry
//         }, helpers::BuiltinBlockExt, ir::{
//             attribute::{DenseI32ArrayAttribute, StringAttribute, TypeAttribute}, r#type::FunctionType, Block, BlockLike, Location, Module, Region, Type
//         }, pass::{self, PassManager}, utility::{register_all_dialects, register_all_llvm_translations}, Context
//     };

//     use crate::{dialect_gen, melior_gen};

//     pub fn load_all_dialects(context: &Context) {
//         let registry = DialectRegistry::new();
//         register_all_dialects(&registry);
//         context.append_dialect_registry(&registry);
//         context.load_all_available_dialects();
//     }

//     pub fn create_test_context() -> Context {
//         let context = Context::new();

//         context.attach_diagnostic_handler(|diagnostic| {
//             eprintln!("{diagnostic}");
//             true
//         });

//         load_all_dialects(&context);
//         register_all_llvm_translations(&context);

//         context
//     }

//     fn convert_module<'c>(context: &'c Context, module: &mut Module<'c>) {
//         let pass_manager = PassManager::new(context);

//         pass_manager.add_pass(pass::conversion::create_func_to_llvm());
//         pass_manager
//             .nested_under("func.func")
//             .add_pass(pass::conversion::create_arith_to_llvm());
//         pass_manager
//             .nested_under("func.func")
//             .add_pass(pass::conversion::create_index_to_llvm());
//         pass_manager.add_pass(pass::conversion::create_scf_to_control_flow());
//         pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
//         pass_manager.add_pass(pass::conversion::create_finalize_mem_ref_to_llvm());

//         assert_eq!(pass_manager.run(module), Ok(()));
//         assert!(module.as_operation().verify());
//     }

//     fn test_operation<'c>(
//         name: &str,
//         context: &'c Context,
//         argument_types: &[Type<'c>],
//         callback: impl FnOnce(&Block<'c>),
//     ) {
//         let location = Location::unknown(context);
//         let mut module = Module::new(location);

//         module.body().append_operation(
//             func::func(
//                 context,
//                 {
//                     let block = Block::new(
//                         &argument_types
//                             .iter()
//                             .copied()
//                             .map(|r#type| (r#type, location))
//                             .collect::<Vec<_>>(),
//                     );

//                     callback(&block);

//                     let region = Region::new();
//                     region.append_block(block);
//                     region
//                 },
//                 StringAttribute::new(context, "foo"),
//                 TypeAttribute::new(FunctionType::new(context, argument_types, &[]).into()),
//                 location,
//             )
//             .into(),
//         );

//         convert_module(context, &mut module);

//         assert!(module.as_operation().verify());
//         //insta::assert_snapshot!(name, module.as_operation());
//     }

//     #[test]
//     fn test_poly_constant() {
//         let context = create_test_context();
//         let location = Location::unknown(&context);

//         test_operation("poly_constant", &context, &[], |block| {
//             let constant = block
//                 .append_op_result(arith::constant(
//                     &context,
//                     DenseI32ArrayAttribute::new(&context, &[1, 2, 3]).into(),
//                     location,
//                 ))
//                 .unwrap();
//             let co = dialect_gen::poly::from_tensor();
//         });
//     }
// }
