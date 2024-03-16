//! Defines the an intermediate language used to define a vector of [`Operation`](general_assembly::Operation)s.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(rustdoc::all)]
pub mod ast;
pub mod ga_backend;
pub mod parser;

use quote::format_ident;
use syn::Ident;

struct TranspilerState<T> {
    to_declare: Vec<Ident>,
    to_insert_above: Vec<T>,
    intermediate_counter: usize,
}


trait Compile {
    type Output;
    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Self::Output;
}

impl<T> TranspilerState<T> {
    fn new() -> Self {
        Self {
            to_declare: Vec::new(),
            to_insert_above: Vec::new(),
            intermediate_counter: 0,
        }
    }

    fn intermediate(&mut self) -> Ident {
        let new_ident = format_ident!("intermediate_{}", self.intermediate_counter);
        self.to_declare.push(new_ident.clone());
        self.intermediate_counter += 1;
        new_ident
    }
}
