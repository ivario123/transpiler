//! Defines the intermediate representation of the language.

pub mod function;
pub mod intrinsic;
pub mod operand;
pub mod operations;
use function::Function;
use operations::{Assign, BinOp, UnOp};

use syn::{Expr, Ident};

use self::function::Jump;

#[derive(Debug, Clone)]
/// Top level intermediate representation of the program.
pub struct IR {
    /// This must be a [`Vec`]
    pub ret: Option<Ident>,
    // pub extensions: Vec<IRExpr>,
    pub extensions: Vec<RustSyntax>,
}


#[derive(Debug, Clone)]
/// Top level syntactical element.
pub enum RustSyntax {
    // TODO! Make this accept full expressions
    If(Expr, Box<Vec<RustSyntax>>, Option<Box<Vec<RustSyntax>>>),
    For(Ident, Expr, Box<Vec<RustSyntax>>),
    Exprs(Vec<Box<IRExpr>>),
    RustExpr(Expr),
}

#[derive(Debug, Clone)]
pub enum IRExpr {
    UnOp(UnOp),
    BinOp(BinOp),
    Assign(Assign),
    Function(Function),
    Jump(Jump),
}
