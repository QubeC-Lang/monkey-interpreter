use crate::ast;
use crate::structs;

/// A struct representing a 'let' statement in the abstract syntax tree (AST).
pub(crate) struct LetStatement {
    pub(crate) token: structs::token::Token,
    pub(crate) name: ast::identifier::Identifier,
    pub(crate) value: Option<Box<dyn ast::Expression>>,
}

impl ast::AstNode for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl ast::Statement for LetStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
