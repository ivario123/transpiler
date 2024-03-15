use proc_macro2::TokenStream;
use quote::quote;

use crate::{ast::operand::*, Compile};

impl Compile for Operand {
    type Output = TokenStream;

    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        match self {
            Self::Expr(e) => e.compile(state),
            Self::Ident(i) => i.compile(state),
            Self::FieldExtract(f) => f.compile(state),
        }
    }
}
impl Compile for ExprOperand {
    type Output = TokenStream;

    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        match self {
            Self::Paren(p) => quote!((#p)),
            Self::Chain(i, it) => {
                let ident: TokenStream = (*i).compile(state);
                let ops: Vec<TokenStream> = it
                    .into_iter()
                    .map(|(ident, args)| {
                        let args = args
                            .into_iter()
                            .map(|el| (*el).compile(state))
                            .collect::<Vec<TokenStream>>();
                        quote!(#ident(#(#args),*))
                    })
                    .collect();
                quote!(#ident.#(#ops).*)
            }
            Self::Ident(i) => quote!(#i.clone()),
            Self::Literal(l) => quote!(#l),
            Self::FunctionCall(f) => f.compile(state),
        }
    }
}
impl Compile for IdentOperand {
    type Output = TokenStream;

    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        match self.define {
            true => state.to_declare.push(self.ident.clone()),
            false => {}
        };
        let ident = self.ident.clone();
        quote!(#ident.clone())
    }
}

impl Compile for DelimiterType {
    type Output = TokenStream;

    fn compile(&self, _state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        match self {
            Self::Const(l) => quote!(#l),
            Self::Ident(i) => quote!(#i),
        }
    }
}

impl Compile for FieldExtract {
    type Output = TokenStream;

    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        let intermediate1 = state.intermediate();
        let intermediate2 = state.intermediate();
        let (start, end) = (
            self.start.clone().compile(state),
            self.end.clone().compile(state),
        );
        let operand = self.operand.clone();
        let ty = self.ty.clone().unwrap_or(syn::parse_quote!(u32));
        state.to_insert_above.extend([
            quote!(
                Operation::Srl {
                    destination: #intermediate1.clone(),
                    operand: #operand.clone(),
                    shift: Operand::Immidiate((#start as #ty).into())
                }
            ),
            quote!(
                #[allow(clippy::unnecessary_cast)]
                Operation::And {
                    destination: #intermediate2.clone(),
                    operand1: #intermediate1.clone(),
                    operand2: Operand::Immidiate(
                        (
                            (
                                (
                                    (0b1u64 << (#end as u64 - #start as u64 + 1u64)) as u64
                                ) - (1 as u64)
                            )as #ty
                        ).into()
                    )

                }
            ),
        ]);
        quote!(#intermediate2)
    }
}
