use super::operand::Operand;

/// Enumerates all valid binary operations.
///
/// This is merely a type-level denotation of
/// operations such as + or -.
#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Sub,
    Add,
    AddWithCarry,
    Div,
    Mul,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalLeftShift,
    LogicalRightShift,
    ArithmeticRightShift,
}

/// Enumerates all valid unary operations.
///
/// This is merely a type-level denotation of
/// operations such as !.
#[derive(Debug, Clone)]
pub enum UnaryOperation {
    BitwiseNot,
}

/// An assign statement.
/// 
/// This is syntactically equivalent to
/// ```ignore
/// a = b;
/// ```
#[derive(Debug, Clone)]
pub struct Assign {
    pub dest: Operand,
    pub rhs: Operand,
}

/// A unary operation.
/// 
/// This is syntactically equivalent to
/// ```ignore
/// a = !b;
/// ```
#[derive(Debug, Clone)]
pub struct UnOp {
    pub dest: Operand,
    pub op: UnaryOperation,
    pub rhs: Operand,
}

/// A binary operation.
/// 
/// This is syntactically equivalent to
/// ```ignore
/// a = b + c; // Or any other binary operation
/// ```
#[derive(Debug, Clone)]
pub struct BinOp {
    pub dest: Operand,
    pub op: BinaryOperation,
    pub lhs: Operand,
    pub rhs: Operand,
}