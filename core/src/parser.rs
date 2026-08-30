use crate::Tokenizer;
use crate::ast;
use crate::structs;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Option<structs::token::Token>,
    peek_token: Option<structs::token::Token>,
    errors: Vec<String>,
}

impl Parser<'_> {
    pub fn new(tokenizer: Tokenizer) -> Parser {
        let mut parser = Parser {
            tokenizer,
            current_token: None,
            peek_token: None,
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, token_type: &structs::token::TokenType) {
        let peek_token_type = self
            .peek_token
            .as_ref()
            .map(|token| token.token_type.to_string())
            .unwrap_or_else(|| "None".to_string());
        let message = format!(
            "expected next token to be {}, got {} instead",
            token_type, peek_token_type
        );
        self.errors.push(message);
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.tokenizer.next();
    }

    fn is_current_token(&self, token_type: &structs::token::TokenType) -> bool {
        self.current_token
            .as_ref()
            .is_some_and(|token| token.token_type == *token_type)
    }

    fn is_peek_token(&self, token_type: &structs::token::TokenType) -> bool {
        self.peek_token
            .as_ref()
            .is_some_and(|token| token.token_type == *token_type)
    }

    fn expect_peek(&mut self, token_type: &structs::token::TokenType) -> bool {
        if self.is_peek_token(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = self.current_token.clone()?;
        if !self.expect_peek(&structs::token::TokenType::Identifier) {
            return None;
        }

        let name_token = self.current_token.clone()?;
        let name = ast::identifier::Identifier {
            token: name_token.clone(),
            value: name_token.literal.clone(),
        };

        if !self.expect_peek(&structs::token::TokenType::Assign) {
            return None;
        }

        while !self.is_current_token(&structs::token::TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(ast::let_statement::LetStatement {
            token,
            name,
            value: None,
        }))
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.current_token.as_ref()?.token_type {
            structs::token::TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::program::Program> {
        let mut program = ast::program::Program {
            statements: Vec::new(),
        };

        while self.current_token.is_some()
            && self.current_token.as_ref().unwrap().token_type != structs::token::TokenType::Eof
        {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        Some(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_parser_errors(parser: &Parser, expected_errors: Vec<&str>) {
        let errors = parser.errors();

        // Print the errors and expected errors for debugging.
        println!("{} parser errors:", errors.len());
        for error in &errors {
            println!("\t{}", error);
        }
        println!("{} expected errors:", expected_errors.len());
        for expected_error in &expected_errors {
            println!("\t{}", expected_error);
        }

        // Check that the number of errors matches the expected number.
        assert_eq!(
            errors.len(),
            expected_errors.len(),
            "expected {} errors, got {}",
            expected_errors.len(),
            errors.len()
        );

        // Check that each error matches the expected error.
        for (i, expected_error) in expected_errors.iter().enumerate() {
            assert_eq!(
                errors[i], *expected_error,
                "expected error {} to be '{}', got '{}'",
                i, expected_error, errors[i]
            );
        }
    }

    fn test_let_statement(statement: &dyn ast::Statement, expected_name: &str) {
        assert_eq!(
            statement.token_literal(),
            "let",
            "statement.token_literal() not 'let'. got={}",
            statement.token_literal()
        );

        if let Some(let_statement) = statement
            .as_any()
            .downcast_ref::<ast::let_statement::LetStatement>()
        {
            assert_eq!(
                let_statement.name.value, expected_name,
                "let_statement.name.value not '{}'. got={}",
                expected_name, let_statement.name.value
            );
        } else {
            panic!("statement is not a LetStatement");
        }
    }

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 65536;
        ";
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::new(tokenizer);
        let program = parser.parse_program();

        check_parser_errors(&parser, vec![]);

        assert!(program.is_some(), "parse_program() returned None");
        assert_eq!(
            program.as_ref().unwrap().statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.as_ref().unwrap().statements.len()
        );

        let expected_identifiers = ["x", "y", "foobar"];
        for (i, expected_identifier) in expected_identifiers.iter().enumerate() {
            let statement = program.as_ref().unwrap().statements[i].as_ref();
            test_let_statement(statement, expected_identifier);
        }
    }

    #[test]
    fn test_let_statements_with_errors() {
        let input = "
            let x 5;
            let = 10;
            let 65536;
        ";
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::new(tokenizer);
        let program = parser.parse_program();

        check_parser_errors(
            &parser,
            vec![
                "expected next token to be ASSIGN, got INTEGER instead",
                "expected next token to be IDENTIFIER, got ASSIGN instead",
                "expected next token to be IDENTIFIER, got INTEGER instead",
            ],
        );
        assert!(program.is_some(), "parse_program() returned None");
        assert_eq!(
            program.as_ref().unwrap().statements.len(),
            0,
            "program.statements does not contain 0 statements. got={}",
            program.as_ref().unwrap().statements.len()
        );
    }
}
