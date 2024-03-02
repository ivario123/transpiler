use crate::{ast::function::*, Compile, CompilerState};
use proc_macro2::TokenStream;
use quote::quote;

impl Compile for Function {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        match self {
            // This should not be managed by us
            Self::Ident(i, args) => quote! {#i(#(#args),*)},
            Self::Intrinsic(i) => i.compile(state),
        }
    }
}

impl Compile for Intrinsic {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        match self {
            Self::ZeroExtend(z) => z.compile(state),
            Self::SignExtend(s) => s.compile(state),
            Self::SetNFlag(n) => n.compile(state),
            Self::SetZFlag(z) => z.compile(state),
            Self::ConditionalJump(j) => j.compile(state),
            Self::LocalAddress(a) => a.compile(state),
        }
    }
}

impl Compile for FunctionCall {
    type Output = TokenStream;
    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        let f: TokenStream = self.ident.clone().compile(state);
        let args = self.args.clone();
        quote! {
            #f(#(#args),*)
        }
    }
}

impl Compile for LocalAddress {
    type Output = TokenStream;
    fn compile(&self, _state: &mut CompilerState<Self::Output>) -> Self::Output {
        let name = self.name.clone();
        let bits = self.bits.clone();
        quote!(Operand::AddressInLocal(#name.to_owned(),#bits))
    }
}

impl Compile for ConditionalJump {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        let operand = self.operand.compile(state);
        let condition = self.condition.clone();

        quote!(Operation::ConditionalJump { destination: #operand,condition:#condition.clone() })
    }
}

impl Compile for SetNFlag {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        let operand = self.operand.compile(state);
        quote!(Operation::SetNFlag { operand: #operand })
    }
}

impl Compile for SetZFlag {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        let operand = self.operand.compile(state);
        quote!(Operation::SetZFlag { operand: #operand })
    }
}

impl Compile for SignExtend {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        let intermediate = state.intermediate();
        let operand = self.operand.compile(state);
        let bits = self.bits.clone();
        state.to_insert_above.push(quote!(Operation::SignExtend {
                destination: #intermediate.clone(),
                operand: #operand, bits: #bits.clone()
        }));
        quote!(#intermediate)
    }
}

impl Compile for ZeroExtend {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        let intermediate = state.intermediate();
        let operand = self.operand.compile(state);
        let bits = self.bits.clone();
        state.to_insert_above.push(quote!(Operation::ZeroExtend { d
                estination: #intermediate.clone(),
                operand: #operand, bits: #bits.clone()
        }));
        quote!(#intermediate)
    }
}
