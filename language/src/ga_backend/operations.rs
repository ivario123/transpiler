use proc_macro2::TokenStream;
use quote::quote;

use crate::{ast::operations::*, Compile};

impl Compile for Assign {
    type Output = TokenStream;
    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        let dst: TokenStream = self.dest.compile(state);
        let rhs: TokenStream = self.rhs.compile(state);
        let to_insert = state.to_insert_above.drain(..);
        quote! {
            #(#to_insert,)*
            Operation::Move { destination: #dst, source: #rhs }
        }
        .into()
    }
}

impl Compile for UnOp {
    type Output = TokenStream;
    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        let dst: TokenStream = self.dest.compile(state);
        let rhs: TokenStream = self.rhs.compile(state);
        let ret = match self.op {
            UnaryOperation::BitwiseNot => quote!(
                Operation::Not { destination: #dst, operand: #rhs }
            ),
        };

        let to_insert = state.to_insert_above.drain(..);
        quote!(
        #(#to_insert,)*
        #ret
        )
    }
}

impl Compile for BinOp {
    type Output = TokenStream;
    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        let dst: TokenStream = self.dest.compile(state);
        let rhs: TokenStream = self.rhs.compile(state);
        let lhs: TokenStream = self.lhs.compile(state);
        let ret = match self.op {
            BinaryOperation::Sub => quote!(
                        Operation::Sub {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::Add => quote!(
                        Operation::Add {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::AddWithCarry => quote!(
                        Operation::Adc {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::Div => quote!(
                        Operation::Div {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::Mul => quote!(
                        Operation::Mul {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::BitwiseOr => quote!(
                        Operation::Or {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::BitwiseAnd => quote!(
                        Operation::And {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::BitwiseXor => quote!(
                        Operation::Xor {
                            destination: #dst,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            ),
            BinaryOperation::LogicalLeftShift => quote!(
                        Operation::Sl {
                            destination: #dst,
                            operand: #lhs,
                            shift: #rhs
                        }
            ),
            BinaryOperation::LogicalRightShift => quote!(
                        Operation::Srl {
                            destination: #dst,
                            operand: #lhs,
                            shift: #rhs
                        }
            ),
            BinaryOperation::ArithmeticRightShift => quote!(
                        Operation::Sra {
                            destination: #dst,
                            operand: #lhs,
                            shift: #rhs
                        }
            ),
        };
        let to_insert = state.to_insert_above.drain(..);
        quote!(
        #(#to_insert,)*
        #ret
        )
    }
}
