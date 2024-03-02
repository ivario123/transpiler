//! Defines all AST types that are concern functions
use super::operand::Operand;
use syn::{Expr, Ident, Lit};

#[derive(Debug, Clone)]
/// Enumerates all supported function types
pub enum Function {
    Ident(Ident, Vec<Expr>),
    Intrinsic(Box<Intrinsic>),
}

/// A simple representation of a normal rust function call
///
/// These reffer to functions outside of the macro call.
/// For these we simply ignore them and re call them in
/// the output.
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub ident: Function,
    pub args: Vec<Expr>,
}

// TODO! Implement remaining set flag things
#[derive(Debug, Clone)]
/// Enumerates all of the built in functions
///
/// These are ways of calling [`general_assembly`]
/// instructions that are not arithemetic operations
pub enum Intrinsic {
    ZeroExtend(ZeroExtend),
    SignExtend(SignExtend),
    ConditionalJump(ConditionalJump),
    SetNFlag(SetNFlag),
    SetZFlag(SetZFlag),
    LocalAddress(LocalAddress),
}

// ===============================================
//              Defintion of intrinsics
// ===============================================
#[derive(Debug, Clone)]
pub struct ZeroExtend {
    pub operand: Operand,
    pub bits: Ident,
}

#[derive(Debug, Clone)]
pub struct SignExtend {
    pub operand: Operand,
    pub bits: Ident,
}

#[derive(Debug, Clone)]
pub struct LocalAddress {
    pub name: Lit,
    pub bits: Lit,
}

#[derive(Debug, Clone)]
pub struct ConditionalJump {
    pub operand: Operand,
    pub condition: Ident,
}

#[derive(Debug, Clone)]
pub struct SetNFlag {
    pub operand: Operand,
}

#[derive(Debug, Clone)]
pub struct SetZFlag {
    pub operand: Operand,
}
