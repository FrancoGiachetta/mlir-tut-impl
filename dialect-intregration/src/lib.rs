use melior::{
    Context,
    dialect::{DialectRegistry, func},
    helpers::BuiltinBlockExt,
    ir::{
        Attribute, Block, BlockLike, Identifier, Location, Module, Region, Type,
        attribute::{DenseI32ArrayAttribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
    utility::{register_all_dialects, register_all_passes},
};

use crate::bindings::dialect_gen;

mod bindings;

pub fn initialize_context() -> Context {
    let context = Context::new();
    context.append_dialect_registry(&{
        let registry = DialectRegistry::new();

        unsafe { bindings::dialect_gen::registerCustomDialects((&registry).to_raw()) };

        register_all_dialects(&registry);
        registry
    });
    context.load_all_available_dialects();
    register_all_passes();
    context
}

pub fn build_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    let u32_type: Type<'_> = IntegerType::new(ctx, 32).into();
    module.body().append_operation(func::func(
        ctx,
        StringAttribute::new(ctx, "entrypoint"),
        TypeAttribute::new(FunctionType::new(ctx, &[u32_type, u32_type], &[u32_type]).into()),
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));

            let coefficients = DenseI32ArrayAttribute::new(&ctx, &[2, 3]).into();

            let result = block
                .append_op_result(
                    dialect_gen::poly::constant(
                        &ctx,
                        dialect_gen::tensor(&ctx),
                        coefficients,
                        location,
                    )
                    .into(),
                )
                .unwrap();

            block.append_operation(func::r#return(&[result], location));

            region
        },
        &[(
            Identifier::new(ctx, "llvm.emit_c_interface"),
            Attribute::unit(ctx),
        )],
        location,
    ));

    module
}
