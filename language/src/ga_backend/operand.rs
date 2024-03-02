use proc_macro2::TokenStream;
use quote::quote;
use crate::{ast::operand::*, Compile};

impl Compile for Operand {
    type Output = TokenStream;
    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        match self {
            Self::Expr(e) => e.compile(state),
            Self::Ident(i) => i.compile(state),
            Self::FunctionCall(f) => (*f).compile(state),
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
