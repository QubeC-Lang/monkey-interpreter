use std::any::Any;

pub(crate) mod identifier;
pub(crate) mod let_statement;
pub(crate) mod program;

/// The trait of a node in the abstract syntax tree (AST).
pub trait AstNode {
    /// Returns the literal value of the token associated with this AST node.
    fn token_literal(&self) -> String;
}

/// The trait of a statement in the abstract syntax tree (AST),
/// representing a complete instruction that does not produce a value,
/// such as variable declarations or control flow statements.
pub trait Statement: AstNode {
    /// Returns this statement as [`Any`] so callers can downcast trait objects
    /// to concrete statement types.
    fn as_any(&self) -> &dyn Any;
}

/// The trait of an expression in the abstract syntax tree (AST),
/// representing a construct that produces a value, such as arithmetic operations,
/// function calls, or variable references.
pub(crate) trait Expression: AstNode {
    /// Returns this expression as [`Any`] so callers can downcast trait objects
    /// to concrete expression types.
    fn as_any(&self) -> &dyn Any;
}
