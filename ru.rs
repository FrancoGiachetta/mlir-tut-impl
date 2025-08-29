#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
mod dialect_gen {
    /**`poly` dialect.

The poly dialect defines types and operations for single-variable
polynomials over integers.
*/
    pub mod poly {
        ///An [`add`](AddOperation) operation. Addition operation between polynomials..
        /**

*/
        ///
        pub struct AddOperation<'c> {
            operation: ::melior::ir::operation::Operation<'c>,
        }
        impl<'c> AddOperation<'c> {
            /// Returns a name.
            pub fn name() -> &'static str {
                "poly.add"
            }
            /// Returns a generic operation.
            pub fn as_operation(&self) -> &::melior::ir::operation::Operation<'c> {
                &self.operation
            }
            /// Creates a builder.
            pub fn builder(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> AddOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Unset,
                ::melior::dialect::ods::__private::Unset,
            > {
                AddOperationBuilder::new(context, location)
            }
            pub fn output(
                &self,
            ) -> Result<
                ::melior::ir::operation::OperationResult<'c, '_>,
                ::melior::Error,
            > {
                self.operation.result(0usize)
            }
            pub fn lhs(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(0usize)
            }
            pub fn rhs(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(1usize)
            }
        }
        ///A builder for an [`add`](AddOperation) operation.
        pub struct AddOperationBuilder<'c, O0, O1> {
            builder: ::melior::ir::operation::OperationBuilder<'c>,
            context: &'c ::melior::Context,
            _state: ::std::marker::PhantomData<(O0, O1)>,
        }
        impl<
            'c,
        > AddOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Unset,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn new(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> Self {
                Self {
                    context,
                    builder: ::melior::ir::operation::OperationBuilder::new(
                        "poly.add",
                        location,
                    ),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            O1,
        > AddOperationBuilder<'c, ::melior::dialect::ods::__private::Unset, O1> {
            pub fn lhs(
                self,
                lhs: ::melior::ir::Value<'c, '_>,
            ) -> AddOperationBuilder<'c, ::melior::dialect::ods::__private::Set, O1> {
                AddOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[lhs]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > AddOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn rhs(
                self,
                rhs: ::melior::ir::Value<'c, '_>,
            ) -> AddOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Set,
                ::melior::dialect::ods::__private::Set,
            > {
                AddOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[rhs]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > AddOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Set,
        > {
            pub fn build(self) -> AddOperation<'c> {
                self.builder
                    .enable_result_type_inference()
                    .build()
                    .expect("valid operation")
                    .try_into()
                    .expect("should be a valid AddOperation")
            }
        }
        #[allow(clippy::too_many_arguments)]
        ///Creates an [`add`](AddOperation) operation.
        pub fn add<'c>(
            context: &'c ::melior::Context,
            lhs: ::melior::ir::Value<'c, '_>,
            rhs: ::melior::ir::Value<'c, '_>,
            location: ::melior::ir::Location<'c>,
        ) -> AddOperation<'c> {
            AddOperation::builder(context, location).lhs(lhs).rhs(rhs).build()
        }
        impl<'c> TryFrom<::melior::ir::operation::Operation<'c>> for AddOperation<'c> {
            type Error = ::melior::Error;
            fn try_from(
                operation: ::melior::ir::operation::Operation<'c>,
            ) -> Result<Self, Self::Error> {
                Ok(Self { operation })
            }
        }
        impl<'c> From<AddOperation<'c>> for ::melior::ir::operation::Operation<'c> {
            fn from(operation: AddOperation<'c>) -> Self {
                operation.operation
            }
        }
        ///A [`constant`](ConstantOperation) operation. Define a constant polynomial via an attribute..
        /**

*/
        ///
        pub struct ConstantOperation<'c> {
            operation: ::melior::ir::operation::Operation<'c>,
        }
        impl<'c> ConstantOperation<'c> {
            /// Returns a name.
            pub fn name() -> &'static str {
                "poly.constant"
            }
            /// Returns a generic operation.
            pub fn as_operation(&self) -> &::melior::ir::operation::Operation<'c> {
                &self.operation
            }
            /// Creates a builder.
            pub fn builder(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> ConstantOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Unset,
                ::melior::dialect::ods::__private::Unset,
            > {
                ConstantOperationBuilder::new(context, location)
            }
            pub fn output(
                &self,
            ) -> Result<
                ::melior::ir::operation::OperationResult<'c, '_>,
                ::melior::Error,
            > {
                self.operation.result(0usize)
            }
            #[allow(clippy::needless_question_mark)]
            pub fn coefficients(
                &self,
            ) -> Result<::melior::ir::attribute::Attribute<'c>, ::melior::Error> {
                Ok(self.operation.attribute("coefficients")?.try_into()?)
            }
            pub fn set_coefficients(
                &mut self,
                value: ::melior::ir::attribute::Attribute<'c>,
            ) {
                self.operation.set_attribute("coefficients", value.into());
            }
        }
        ///A builder for a [`constant`](ConstantOperation) operation.
        pub struct ConstantOperationBuilder<'c, T0, A0> {
            builder: ::melior::ir::operation::OperationBuilder<'c>,
            context: &'c ::melior::Context,
            _state: ::std::marker::PhantomData<(T0, A0)>,
        }
        impl<
            'c,
        > ConstantOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Unset,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn new(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> Self {
                Self {
                    context,
                    builder: ::melior::ir::operation::OperationBuilder::new(
                        "poly.constant",
                        location,
                    ),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            A0,
        > ConstantOperationBuilder<'c, ::melior::dialect::ods::__private::Unset, A0> {
            pub fn output(
                self,
                output: ::melior::ir::Type<'c>,
            ) -> ConstantOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Set,
                A0,
            > {
                ConstantOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_results(&[output]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            T0,
        > ConstantOperationBuilder<'c, T0, ::melior::dialect::ods::__private::Unset> {
            pub fn coefficients(
                self,
                coefficients: ::melior::ir::attribute::Attribute<'c>,
            ) -> ConstantOperationBuilder<
                'c,
                T0,
                ::melior::dialect::ods::__private::Set,
            > {
                ConstantOperationBuilder {
                    context: self.context,
                    builder: self
                        .builder
                        .add_attributes(
                            &[
                                (
                                    ::melior::ir::Identifier::new(self.context, "coefficients"),
                                    coefficients.into(),
                                ),
                            ],
                        ),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > ConstantOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Set,
        > {
            pub fn build(self) -> ConstantOperation<'c> {
                self.builder
                    .build()
                    .expect("valid operation")
                    .try_into()
                    .expect("should be a valid ConstantOperation")
            }
        }
        #[allow(clippy::too_many_arguments)]
        ///Creates a [`constant`](ConstantOperation) operation.
        pub fn constant<'c>(
            context: &'c ::melior::Context,
            output: ::melior::ir::Type<'c>,
            coefficients: ::melior::ir::attribute::Attribute<'c>,
            location: ::melior::ir::Location<'c>,
        ) -> ConstantOperation<'c> {
            ConstantOperation::builder(context, location)
                .output(output)
                .coefficients(coefficients)
                .build()
        }
        impl<'c> TryFrom<::melior::ir::operation::Operation<'c>>
        for ConstantOperation<'c> {
            type Error = ::melior::Error;
            fn try_from(
                operation: ::melior::ir::operation::Operation<'c>,
            ) -> Result<Self, Self::Error> {
                Ok(Self { operation })
            }
        }
        impl<'c> From<ConstantOperation<'c>> for ::melior::ir::operation::Operation<'c> {
            fn from(operation: ConstantOperation<'c>) -> Self {
                operation.operation
            }
        }
        ///An [`eval`](EvalOperation) operation. Evaluates a Polynomial at a given input value..
        /**

*/
        ///
        pub struct EvalOperation<'c> {
            operation: ::melior::ir::operation::Operation<'c>,
        }
        impl<'c> EvalOperation<'c> {
            /// Returns a name.
            pub fn name() -> &'static str {
                "poly.eval"
            }
            /// Returns a generic operation.
            pub fn as_operation(&self) -> &::melior::ir::operation::Operation<'c> {
                &self.operation
            }
            /// Creates a builder.
            pub fn builder(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> EvalOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Unset,
                ::melior::dialect::ods::__private::Unset,
                ::melior::dialect::ods::__private::Unset,
            > {
                EvalOperationBuilder::new(context, location)
            }
            pub fn output(
                &self,
            ) -> Result<
                ::melior::ir::operation::OperationResult<'c, '_>,
                ::melior::Error,
            > {
                self.operation.result(0usize)
            }
            pub fn input(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(0usize)
            }
            pub fn point(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(1usize)
            }
        }
        ///A builder for an [`eval`](EvalOperation) operation.
        pub struct EvalOperationBuilder<'c, T0, O0, O1> {
            builder: ::melior::ir::operation::OperationBuilder<'c>,
            context: &'c ::melior::Context,
            _state: ::std::marker::PhantomData<(T0, O0, O1)>,
        }
        impl<
            'c,
        > EvalOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Unset,
            ::melior::dialect::ods::__private::Unset,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn new(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> Self {
                Self {
                    context,
                    builder: ::melior::ir::operation::OperationBuilder::new(
                        "poly.eval",
                        location,
                    ),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            O0,
            O1,
        > EvalOperationBuilder<'c, ::melior::dialect::ods::__private::Unset, O0, O1> {
            pub fn output(
                self,
                output: ::melior::ir::Type<'c>,
            ) -> EvalOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Set,
                O0,
                O1,
            > {
                EvalOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_results(&[output]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            T0,
            O1,
        > EvalOperationBuilder<'c, T0, ::melior::dialect::ods::__private::Unset, O1> {
            pub fn input(
                self,
                input: ::melior::ir::Value<'c, '_>,
            ) -> EvalOperationBuilder<
                'c,
                T0,
                ::melior::dialect::ods::__private::Set,
                O1,
            > {
                EvalOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[input]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            T0,
        > EvalOperationBuilder<
            'c,
            T0,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn point(
                self,
                point: ::melior::ir::Value<'c, '_>,
            ) -> EvalOperationBuilder<
                'c,
                T0,
                ::melior::dialect::ods::__private::Set,
                ::melior::dialect::ods::__private::Set,
            > {
                EvalOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[point]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > EvalOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Set,
        > {
            pub fn build(self) -> EvalOperation<'c> {
                self.builder
                    .build()
                    .expect("valid operation")
                    .try_into()
                    .expect("should be a valid EvalOperation")
            }
        }
        #[allow(clippy::too_many_arguments)]
        ///Creates an [`eval`](EvalOperation) operation.
        pub fn eval<'c>(
            context: &'c ::melior::Context,
            output: ::melior::ir::Type<'c>,
            input: ::melior::ir::Value<'c, '_>,
            point: ::melior::ir::Value<'c, '_>,
            location: ::melior::ir::Location<'c>,
        ) -> EvalOperation<'c> {
            EvalOperation::builder(context, location)
                .output(output)
                .input(input)
                .point(point)
                .build()
        }
        impl<'c> TryFrom<::melior::ir::operation::Operation<'c>> for EvalOperation<'c> {
            type Error = ::melior::Error;
            fn try_from(
                operation: ::melior::ir::operation::Operation<'c>,
            ) -> Result<Self, Self::Error> {
                Ok(Self { operation })
            }
        }
        impl<'c> From<EvalOperation<'c>> for ::melior::ir::operation::Operation<'c> {
            fn from(operation: EvalOperation<'c>) -> Self {
                operation.operation
            }
        }
        ///A [`from_tensor`](FromTensorOperation) operation. Creates a Polynomial from integer coefficients stored in a tensor..
        /**

*/
        ///
        pub struct FromTensorOperation<'c> {
            operation: ::melior::ir::operation::Operation<'c>,
        }
        impl<'c> FromTensorOperation<'c> {
            /// Returns a name.
            pub fn name() -> &'static str {
                "poly.from_tensor"
            }
            /// Returns a generic operation.
            pub fn as_operation(&self) -> &::melior::ir::operation::Operation<'c> {
                &self.operation
            }
            /// Creates a builder.
            pub fn builder(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> FromTensorOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Unset,
                ::melior::dialect::ods::__private::Unset,
            > {
                FromTensorOperationBuilder::new(context, location)
            }
            pub fn output(
                &self,
            ) -> Result<
                ::melior::ir::operation::OperationResult<'c, '_>,
                ::melior::Error,
            > {
                self.operation.result(0usize)
            }
            pub fn input(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(0usize)
            }
        }
        ///A builder for a [`from_tensor`](FromTensorOperation) operation.
        pub struct FromTensorOperationBuilder<'c, T0, O0> {
            builder: ::melior::ir::operation::OperationBuilder<'c>,
            context: &'c ::melior::Context,
            _state: ::std::marker::PhantomData<(T0, O0)>,
        }
        impl<
            'c,
        > FromTensorOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Unset,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn new(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> Self {
                Self {
                    context,
                    builder: ::melior::ir::operation::OperationBuilder::new(
                        "poly.from_tensor",
                        location,
                    ),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            O0,
        > FromTensorOperationBuilder<'c, ::melior::dialect::ods::__private::Unset, O0> {
            pub fn output(
                self,
                output: ::melior::ir::Type<'c>,
            ) -> FromTensorOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Set,
                O0,
            > {
                FromTensorOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_results(&[output]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            T0,
        > FromTensorOperationBuilder<'c, T0, ::melior::dialect::ods::__private::Unset> {
            pub fn input(
                self,
                input: ::melior::ir::Value<'c, '_>,
            ) -> FromTensorOperationBuilder<
                'c,
                T0,
                ::melior::dialect::ods::__private::Set,
            > {
                FromTensorOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[input]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > FromTensorOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Set,
        > {
            pub fn build(self) -> FromTensorOperation<'c> {
                self.builder
                    .build()
                    .expect("valid operation")
                    .try_into()
                    .expect("should be a valid FromTensorOperation")
            }
        }
        #[allow(clippy::too_many_arguments)]
        ///Creates a [`from_tensor`](FromTensorOperation) operation.
        pub fn from_tensor<'c>(
            context: &'c ::melior::Context,
            output: ::melior::ir::Type<'c>,
            input: ::melior::ir::Value<'c, '_>,
            location: ::melior::ir::Location<'c>,
        ) -> FromTensorOperation<'c> {
            FromTensorOperation::builder(context, location)
                .output(output)
                .input(input)
                .build()
        }
        impl<'c> TryFrom<::melior::ir::operation::Operation<'c>>
        for FromTensorOperation<'c> {
            type Error = ::melior::Error;
            fn try_from(
                operation: ::melior::ir::operation::Operation<'c>,
            ) -> Result<Self, Self::Error> {
                Ok(Self { operation })
            }
        }
        impl<'c> From<FromTensorOperation<'c>>
        for ::melior::ir::operation::Operation<'c> {
            fn from(operation: FromTensorOperation<'c>) -> Self {
                operation.operation
            }
        }
        ///A [`mul`](MulOperation) operation. Subtraction operation between polynomials..
        /**

*/
        ///
        pub struct MulOperation<'c> {
            operation: ::melior::ir::operation::Operation<'c>,
        }
        impl<'c> MulOperation<'c> {
            /// Returns a name.
            pub fn name() -> &'static str {
                "poly.mul"
            }
            /// Returns a generic operation.
            pub fn as_operation(&self) -> &::melior::ir::operation::Operation<'c> {
                &self.operation
            }
            /// Creates a builder.
            pub fn builder(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> MulOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Unset,
                ::melior::dialect::ods::__private::Unset,
            > {
                MulOperationBuilder::new(context, location)
            }
            pub fn output(
                &self,
            ) -> Result<
                ::melior::ir::operation::OperationResult<'c, '_>,
                ::melior::Error,
            > {
                self.operation.result(0usize)
            }
            pub fn lhs(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(0usize)
            }
            pub fn rhs(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(1usize)
            }
        }
        ///A builder for a [`mul`](MulOperation) operation.
        pub struct MulOperationBuilder<'c, O0, O1> {
            builder: ::melior::ir::operation::OperationBuilder<'c>,
            context: &'c ::melior::Context,
            _state: ::std::marker::PhantomData<(O0, O1)>,
        }
        impl<
            'c,
        > MulOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Unset,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn new(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> Self {
                Self {
                    context,
                    builder: ::melior::ir::operation::OperationBuilder::new(
                        "poly.mul",
                        location,
                    ),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            O1,
        > MulOperationBuilder<'c, ::melior::dialect::ods::__private::Unset, O1> {
            pub fn lhs(
                self,
                lhs: ::melior::ir::Value<'c, '_>,
            ) -> MulOperationBuilder<'c, ::melior::dialect::ods::__private::Set, O1> {
                MulOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[lhs]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > MulOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn rhs(
                self,
                rhs: ::melior::ir::Value<'c, '_>,
            ) -> MulOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Set,
                ::melior::dialect::ods::__private::Set,
            > {
                MulOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[rhs]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > MulOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Set,
        > {
            pub fn build(self) -> MulOperation<'c> {
                self.builder
                    .enable_result_type_inference()
                    .build()
                    .expect("valid operation")
                    .try_into()
                    .expect("should be a valid MulOperation")
            }
        }
        #[allow(clippy::too_many_arguments)]
        ///Creates a [`mul`](MulOperation) operation.
        pub fn mul<'c>(
            context: &'c ::melior::Context,
            lhs: ::melior::ir::Value<'c, '_>,
            rhs: ::melior::ir::Value<'c, '_>,
            location: ::melior::ir::Location<'c>,
        ) -> MulOperation<'c> {
            MulOperation::builder(context, location).lhs(lhs).rhs(rhs).build()
        }
        impl<'c> TryFrom<::melior::ir::operation::Operation<'c>> for MulOperation<'c> {
            type Error = ::melior::Error;
            fn try_from(
                operation: ::melior::ir::operation::Operation<'c>,
            ) -> Result<Self, Self::Error> {
                Ok(Self { operation })
            }
        }
        impl<'c> From<MulOperation<'c>> for ::melior::ir::operation::Operation<'c> {
            fn from(operation: MulOperation<'c>) -> Self {
                operation.operation
            }
        }
        ///A [`sub`](SubOperation) operation. Subtraction operation between polynomials..
        /**

*/
        ///
        pub struct SubOperation<'c> {
            operation: ::melior::ir::operation::Operation<'c>,
        }
        impl<'c> SubOperation<'c> {
            /// Returns a name.
            pub fn name() -> &'static str {
                "poly.sub"
            }
            /// Returns a generic operation.
            pub fn as_operation(&self) -> &::melior::ir::operation::Operation<'c> {
                &self.operation
            }
            /// Creates a builder.
            pub fn builder(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> SubOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Unset,
                ::melior::dialect::ods::__private::Unset,
            > {
                SubOperationBuilder::new(context, location)
            }
            pub fn output(
                &self,
            ) -> Result<
                ::melior::ir::operation::OperationResult<'c, '_>,
                ::melior::Error,
            > {
                self.operation.result(0usize)
            }
            pub fn lhs(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(0usize)
            }
            pub fn rhs(&self) -> Result<::melior::ir::Value<'c, '_>, ::melior::Error> {
                self.operation.operand(1usize)
            }
        }
        ///A builder for a [`sub`](SubOperation) operation.
        pub struct SubOperationBuilder<'c, O0, O1> {
            builder: ::melior::ir::operation::OperationBuilder<'c>,
            context: &'c ::melior::Context,
            _state: ::std::marker::PhantomData<(O0, O1)>,
        }
        impl<
            'c,
        > SubOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Unset,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn new(
                context: &'c ::melior::Context,
                location: ::melior::ir::Location<'c>,
            ) -> Self {
                Self {
                    context,
                    builder: ::melior::ir::operation::OperationBuilder::new(
                        "poly.sub",
                        location,
                    ),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
            O1,
        > SubOperationBuilder<'c, ::melior::dialect::ods::__private::Unset, O1> {
            pub fn lhs(
                self,
                lhs: ::melior::ir::Value<'c, '_>,
            ) -> SubOperationBuilder<'c, ::melior::dialect::ods::__private::Set, O1> {
                SubOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[lhs]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > SubOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Unset,
        > {
            pub fn rhs(
                self,
                rhs: ::melior::ir::Value<'c, '_>,
            ) -> SubOperationBuilder<
                'c,
                ::melior::dialect::ods::__private::Set,
                ::melior::dialect::ods::__private::Set,
            > {
                SubOperationBuilder {
                    context: self.context,
                    builder: self.builder.add_operands(&[rhs]),
                    _state: Default::default(),
                }
            }
        }
        impl<
            'c,
        > SubOperationBuilder<
            'c,
            ::melior::dialect::ods::__private::Set,
            ::melior::dialect::ods::__private::Set,
        > {
            pub fn build(self) -> SubOperation<'c> {
                self.builder
                    .enable_result_type_inference()
                    .build()
                    .expect("valid operation")
                    .try_into()
                    .expect("should be a valid SubOperation")
            }
        }
        #[allow(clippy::too_many_arguments)]
        ///Creates a [`sub`](SubOperation) operation.
        pub fn sub<'c>(
            context: &'c ::melior::Context,
            lhs: ::melior::ir::Value<'c, '_>,
            rhs: ::melior::ir::Value<'c, '_>,
            location: ::melior::ir::Location<'c>,
        ) -> SubOperation<'c> {
            SubOperation::builder(context, location).lhs(lhs).rhs(rhs).build()
        }
        impl<'c> TryFrom<::melior::ir::operation::Operation<'c>> for SubOperation<'c> {
            type Error = ::melior::Error;
            fn try_from(
                operation: ::melior::ir::operation::Operation<'c>,
            ) -> Result<Self, Self::Error> {
                Ok(Self { operation })
            }
        }
        impl<'c> From<SubOperation<'c>> for ::melior::ir::operation::Operation<'c> {
            fn from(operation: SubOperation<'c>) -> Self {
                operation.operation
            }
        }
    }
}
