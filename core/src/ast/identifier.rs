use crate::ast;
use crate::structs;

/// A struct representing an identifier in the abstract syntax tree (AST).
pub(crate) struct Identifier {
    pub(crate) token: structs::token::Token,
    pub(crate) value: String,
}

impl ast::AstNode for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl ast::Expression for Identifier {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
