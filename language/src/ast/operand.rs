//! Defines all valid operand types

use super::{function::Function, IRExpr};
use syn::{BinOp, Expr, Ident, Lit, Type, UnOp};

/// Enumerates all valid operand types
#[derive(Debug, Clone)]
pub enum Operand {
    Expr(ExprOperand),
    Ident(IdentOperand),
    FieldExtract(FieldExtract),
    // UnOp(Box<UnOp>),
    // BinOp(Box<BinOp>),
}
#[derive(Debug, Clone)]
/// Enumerates a set of different operands.
///
/// These operands are not new identifiers but can be already defined
/// [`Ident`]ifiers.
pub enum ExprOperand {
    Paren(Expr),
    /// A chain like
    /// ```ignore
    /// a.local(<args>).unwrap()
    /// ```
    Chain(Box<ExprOperand>, Vec<(Ident, Vec<Box<Operand>>)>),
    Ident(Ident),
    Literal(Lit),
    FunctionCall(Function),
}
/// A (possibly) new identifier.
#[derive(Debug, Clone)]
pub struct IdentOperand {
    /// Wether or not to insert this in to the local scope or not
    pub define: bool,
    /// The identifier used
    pub ident: Ident,
}

#[derive(Debug, Clone)]
pub enum DelimiterType {
    Const(Lit),
    Ident(Ident),
}

#[derive(Debug, Clone)]
pub struct FieldExtract {
    pub operand: Ident,
    pub start: DelimiterType,
    pub end: DelimiterType,
    pub ty: Option<Type>,
}
