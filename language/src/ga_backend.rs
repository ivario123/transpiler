pub mod function;
pub mod operand;
pub mod operations;

use crate::{ast::*, Compile, CompilerState};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

impl Into<TokenStream> for IR {
    fn into(self) -> TokenStream {
        // let mut declerations: Vec<TokenStream> = vec![];
        // self.extensions
        //     .iter()
        //     .for_each(|el| el.declare(&mut declerations));
        let mut state = CompilerState::new();
        let ret = self.ret.clone().unwrap_or(format_ident!("ret"));
        let ext = self
            .extensions
            .into_iter()
            .map(|el| (ret.clone(), el).compile(&mut state))
            .collect::<Vec<TokenStream>>();
        let declerations = state.to_declare;
        let declaration_strings = declerations.iter().map(|el| el.to_string());
        match self.ret {
            Some(_) => quote!(
                #(let #declerations =
                  Operand::Local(#declaration_strings.to_owned());)*
                #(#ext;)*
            ),
            None => quote!(
                let ret =  Vec::new();
                #(let #declerations =
                  Operand::Local(#declaration_strings.to_owned());)*
                #(#ext;)*
            ),
        }
        .into()
    }
}

impl Compile for IRExpr {
    type Output = TokenStream;
    fn compile(&self, state: &mut crate::CompilerState<Self::Output>) -> Self::Output {
        match self {
            Self::Assign(assign) => assign.compile(state),
            Self::UnOp(unop) => unop.compile(state),
            Self::BinOp(binop) => binop.compile(state),
            Self::Function(f) => f.compile(state),
        }
    }
}

impl Compile for (Ident, RustSyntax) {
    type Output = TokenStream;
    fn compile(&self, state: &mut CompilerState<Self::Output>) -> Self::Output {
        match self.1.clone() {
            RustSyntax::If(e, happy_case, Some(sad_case)) => {
                let to_declare_global: Vec<Ident> = state.to_declare.drain(..).collect();
                let declaration_strings_global = to_declare_global.iter().map(|el| el.to_string());

                let happy_case = (self.0.clone(), *happy_case).compile(state);
                let to_declare_happy: Vec<Ident> = state.to_declare.drain(..).collect();
                let declaration_strings_happy = to_declare_happy.iter().map(|el| el.to_string());

                let sad_case = (self.0.clone(), *sad_case).compile(state);
                let to_declare_sad: Vec<Ident> = state.to_declare.drain(..).collect();
                let declaration_strings_sad = to_declare_sad.iter().map(|el| el.to_string());
                quote!(
                    #(let #to_declare_global =
                        Operand::Local(#declaration_strings_global.to_owned());)*
                    if #e {
                        #(let #to_declare_happy =
                            Operand::Local(#declaration_strings_happy.to_owned());)*
                        #happy_case;
                    } else {
                        #(let #to_declare_sad =
                            Operand::Local(#declaration_strings_sad.to_owned());)*
                        #sad_case;
                    }
                )
            }
            RustSyntax::If(e, happy_case, None) => {
                let to_declare_global: Vec<Ident> = state.to_declare.drain(..).collect();
                let declaration_strings_global = to_declare_global.iter().map(|el| el.to_string());

                let happy_case = (self.0.clone(), *happy_case).compile(state);
                let to_declare_happy: Vec<Ident> = state.to_declare.drain(..).collect();
                let declaration_strings_happy = to_declare_happy.iter().map(|el| el.to_string());
                quote!(
                    #(let #to_declare_global =
                        Operand::Local(#declaration_strings_global.to_owned());)*
                    if #e {
                        #(let #to_declare_happy =
                            Operand::Local(#declaration_strings_happy.to_owned());)*
                        #happy_case;
                    }
                )
            }
            RustSyntax::For(i, e, block) => {
                let to_declare_global: Vec<Ident> = state.to_declare.drain(..).collect();
                let declaration_strings_global = to_declare_global.iter().map(|el| el.to_string());
                let block = (self.0.clone(), *block).compile(state);
                let to_declare_inner: Vec<Ident> = state.to_declare.drain(..).collect();
                let declaration_strings_inner = to_declare_inner.iter().map(|el| el.to_string());
                quote!(
                    #(let #to_declare_global =
                        Operand::Local(#declaration_strings_global.to_owned());)*
                    for #i in #e {
                        #(let #to_declare_inner =
                            Operand::Local(#declaration_strings_inner.to_owned());)*
                        #block
                    }
                )
            }
            RustSyntax::Exprs(extensions) => {
                let mut ext = Vec::new();
                for el in extensions {
                    ext.push(el.compile(state));
                }
                let ret = self.0.clone();
                let declerations: Vec<Ident> = state.to_declare.drain(..).collect();
                let to_insert_above: Vec<TokenStream> = state.to_insert_above.drain(..).collect();
                let declaration_strings = declerations.iter().map(|el| el.to_string());
                quote!(
                #(let #declerations =
                    Operand::Local(#declaration_strings.to_owned());)*
                #ret.extend([
                    #(#to_insert_above,)*
                    #(#ext,)*
                ]);
                )
            }
            RustSyntax::RustExpr(_) => todo!(),
        }
    }
}
