//! Defines the an intermediate language used to define a vector of
//! [`Operation`](general_assembly::operation::Operation)s.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(rustdoc::all)]

pub mod ast;
pub mod ga_backend;
pub mod parser;

use std::collections::HashMap;

use quote::format_ident;
use syn::Ident;

/// All possible errors that can occur when using the [`TranspilerState`].
#[derive(Debug)]
pub enum Error {
    /// The program tried to access a variable that did not exist yet.
    UseBeforeDeclaration(String),

    /// Declared a value that is never used.
    UnusedDeclartion(String),
}

#[derive(Debug)]
struct TranspilerState<T: std::fmt::Debug> {
    to_declare: Vec<Vec<Ident>>,
    to_insert_above: Vec<T>,
    usage_counter: Vec<HashMap<String, usize>>,
    intermediate_counter: usize,
}

trait Compile {
    type Output: std::fmt::Debug;
    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error>;
}

/// Compiles the program will not fail if anny [`Error`] is encountered.
trait CompileUnchecked {
    type Output: std::fmt::Debug;
    fn compile_unchecked(&self, state: &mut TranspilerState<Self::Output>) -> Self::Output;
}

impl<T: std::fmt::Debug> TranspilerState<T> {
    fn new() -> Self {
        Self {
            to_declare: vec![Vec::new()],
            to_insert_above: Vec::new(),
            usage_counter: vec![HashMap::new()],
            intermediate_counter: 0,
        }
    }

    fn access_count(&self, name: &String) -> Option<usize> {
        for scope in self.usage_counter.iter() {
            if let Some(value) = scope.get(name) {
                return Some(*value);
            }
        }
        None
    }

    /// Increments the first occurance of that name.
    fn increment_access(&mut self, name: &String) {
        for scope in self.usage_counter.iter_mut() {
            if let Some(value) = scope.get_mut(name) {
                *value += 1;
            }
        }
    }

    pub fn to_declare(&mut self) -> Result<Vec<Ident>, Error> {
        let to_declare = self.to_declare.pop().expect("Invalid stack management");
        for el in to_declare.iter() {
            let key = el.to_string();
            match self.access_count(&key) {
                Some(value) => {
                    if value == 0 {
                        return Err(Error::UnusedDeclartion(key));
                    }
                }
                None => {
                    return Err(Error::UseBeforeDeclaration(key));
                }
            }
        }
        self.usage_counter.pop();

        Ok(to_declare)
    }

    pub fn declare_local(&mut self, ident: Ident) {
        self.to_declare.last_mut().unwrap().push(ident.clone());
        self.usage_counter
            .first_mut()
            .expect("declare local borked")
            .insert(ident.to_string(), 0);
    }

    pub fn access(&mut self, ident: Ident) {
        let key = ident.to_string();
        self.increment_access(&key)
    }

    pub fn access_str(&mut self, ident: String) {
        self.increment_access(&ident)
    }

    pub fn enter_scope(&mut self) {
        self.to_declare.push(Vec::new());
        self.usage_counter.push(HashMap::new());
    }

    pub fn intermediate(&mut self) -> ast::operand::IdentOperand {
        let new_ident = format_ident!("intermediate_{}", self.intermediate_counter);
        self.to_declare
            .last_mut()
            .expect("Intermediate broken")
            .push(new_ident.clone());
        self.usage_counter
            .last_mut()
            .expect("Intermediate broken")
            .insert(new_ident.clone().to_string(), 0);
        self.intermediate_counter += 1;
        ast::operand::IdentOperand {
            define: false,
            ident: new_ident,
        }
    }
}
