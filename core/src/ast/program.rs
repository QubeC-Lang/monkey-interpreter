use crate::ast;

/// The structure representing a program in the abstract syntax tree (AST),
/// which consists of a sequence of statements that form the body of the program.
pub struct Program {
    /// The list of statements that make up the program.
    pub statements: Vec<Box<dyn ast::Statement>>,
}

impl ast::AstNode for Program {
    fn token_literal(&self) -> String {
        if let Some(first_statement) = self.statements.first() {
            first_statement.token_literal()
        } else {
            String::new()
        }
    }
}
