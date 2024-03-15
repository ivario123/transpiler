//! Defines all AST types that are concern functions
use super::{operand::Operand, operations::BinaryOperation};
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
    Resize(Resize),
    SetNFlag(SetNFlag),
    SetZFlag(SetZFlag),
    LocalAddress(LocalAddress),
    SetVFlag(SetVFlag),
    SetCFlag(SetCFlag),
    Flag(Flag),
    Register(Register),
    Ror(Ror),
    Sra(Sra),
    Signed(Signed),
}

// ===============================================
//              Defintion of intrinsics
// ===============================================

#[derive(Debug, Clone)]
pub struct Jump {
    pub target: Operand,
    pub condtion: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Signed {
    pub op1: Operand,
    pub op2: Operand,
    pub operation: BinaryOperation,
}

#[derive(Debug, Clone)]
pub struct Resize {
    pub operand: Operand,
    pub bits: Expr,
}

#[derive(Debug, Clone)]
pub struct ZeroExtend {
    pub operand: Operand,
    pub bits: Expr,
}

#[derive(Debug, Clone)]
pub struct SignExtend {
    pub operand: Operand,
    pub bits: Expr,
}

#[derive(Debug, Clone)]
pub struct Flag {
    pub name: Lit,
}

#[derive(Debug, Clone)]
pub struct Register {
    pub name: Lit,
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

#[derive(Debug, Clone)]
pub struct SetCFlag {
    pub operand1: Operand,
    pub operand2: Operand,
    pub sub: Lit,
    pub carry: Lit,
}

#[derive(Debug, Clone)]
pub struct SetVFlag {
    pub operand1: Operand,
    pub operand2: Operand,
    pub sub: Lit,
    pub carry: Lit,
}

#[derive(Debug, Clone)]
pub struct Ror {
    pub operand: Operand,
    pub n: Expr,
}

#[derive(Debug, Clone)]
pub struct Sra {
    pub operand: Operand,
    pub n: Expr,
}
