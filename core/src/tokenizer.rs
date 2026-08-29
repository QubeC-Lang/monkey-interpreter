use crate::structs::token;
use crate::structs::token::{Token, TokenType};
use crate::utils;

/// The tokenizer responsible for converting input strings into tokens.
pub struct Tokenizer {
    /// The input string to be tokenized.
    input: String,
    /// The current position in the input (points to current char).
    position: usize,
    /// The current reading position in the input (after current char).
    read_position: usize,
    /// The current character under examination.
    character: Option<char>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            character: None,
        };
        tokenizer.read_char();
        tokenizer
    }

    /// Peeks at the next character in the input without advancing the tokenizer's state.
    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }

    /// Reads the next character from the input and updates the tokenizer's state
    /// to point to the new character.
    fn read_char(&mut self) {
        self.character = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Reads an identifier from the input starting at the current position
    /// till a non-identifier character is encountered.
    /// Returns the identifier literal as a String.
    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while let Some(ch) = self.character {
            if utils::is_legal_identifier_char(ch) {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_position..self.position].to_string()
    }

    /// Reads a number from the input starting at the current position
    /// till a non-digit character is encountered.
    /// Returns the number literal as a String.
    pub fn read_number(&mut self) -> String {
        let start_position = self.position;
        while let Some(ch) = self.character {
            if ch.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_position..self.position].to_string()
    }

    /// Retrieves the token corresponding to the current character read by the tokenizer,
    /// and advances the tokenizer to the next character.
    pub fn next_token(&mut self) -> Token {
        let token = match self.character {
            Some('=') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token { token_type: TokenType::Equal, literal: "==".to_string() }
                } else {
                    Token { token_type: TokenType::Assign, literal: "=".to_string() }
                }
            },
            Some('+') => Token { token_type: TokenType::Plus, literal: "+".to_string() },
            Some('-') => Token { token_type: TokenType::Minus, literal: "-".to_string() },
            Some('*') => Token { token_type: TokenType::Asterisk, literal: "*".to_string() },
            Some('/') => Token { token_type: TokenType::Slash, literal: "/".to_string() },
            Some('<') => Token { token_type: TokenType::LessThan, literal: "<".to_string() },
            Some('>') => Token { token_type: TokenType::GreaterThan, literal: ">".to_string() },
            Some('!') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token { token_type: TokenType::NotEqual, literal: "!=".to_string() }
                } else {
                    Token { token_type: TokenType::Bang, literal: "!".to_string() }
                }
            },
            Some(',') => Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Some(';') => Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Some('(') => Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Some(')') => Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Some('{') => Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Some('}') => Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            None => Token { token_type: TokenType::Eof, literal: "".to_string() },
            _ => {
                let literal = self.character.unwrap_or_else(|| ' ');

                if literal.is_ascii_whitespace() {
                    self.read_char();
                    return self.next_token();
                }

                if utils::is_legal_identifier_char(literal) {
                    let literal = self.read_identifier();
                    let token_type = token::identifier_lookup(&literal);
                    // Return here to avoid advancing the character again
                    // (done by read_identifier) to avoid skipping tokens.
                    return Token { token_type, literal }
                } else if literal.is_ascii_digit() {
                    let literal = self.read_number();
                    // Return here to avoid advancing the character again
                    // (done by read_number) to avoid skipping tokens.
                    return Token { token_type: TokenType::Integer, literal }
                } else {
                    Token { token_type: TokenType::Illegal, literal: literal.to_string() }
                }
            }
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "=+,(){}.;";

        let expected_output = vec![
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Plus, literal: "+".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            Token { token_type: TokenType::Illegal, literal: ".".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Eof, literal: "".to_string() },
        ];

        let mut tokenizer = Tokenizer::new(input);
        for expected_token in expected_output {
            let token = tokenizer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }

    #[test]
    fn test_basic_codes() {
        let input = "let five = 5;
                     let ten = 10;

                     let add = fn(x, y) {
                         x + y;
                     };

                     let result = add(five, ten);
                     !-/*5;
                     5 < 10 > 5;

                     10 == 10;
                     10 != 9;";

        let expected_output = vec![
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "five".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "ten".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "add".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Function, literal: "fn".to_string() },
            Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Token { token_type: TokenType::Identifier, literal: "x".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Identifier, literal: "y".to_string() },
            Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Token { token_type: TokenType::Identifier, literal: "x".to_string() },
            Token { token_type: TokenType::Plus, literal: "+".to_string() },
            Token { token_type: TokenType::Identifier, literal: "y".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "result".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Identifier, literal: "add".to_string() },
            Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Token { token_type: TokenType::Identifier, literal: "five".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Identifier, literal: "ten".to_string() },
            Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Bang, literal: "!".to_string() },
            Token { token_type: TokenType::Minus, literal: "-".to_string() },
            Token { token_type: TokenType::Slash, literal: "/".to_string() },
            Token { token_type: TokenType::Asterisk, literal: "*".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::LessThan, literal: "<".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::GreaterThan, literal: ">".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::Equal, literal: "==".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::NotEqual, literal: "!=".to_string() },
            Token { token_type: TokenType::Integer, literal: "9".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Eof, literal: "".to_string() },
        ];

        let mut tokenizer = Tokenizer::new(input);
        for expected_token in expected_output {
            let token = tokenizer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }

    #[test]
    fn test_keywords() {
        let input = "let five = 5;
                     let ten = 10;

                     let add = fn(x, y) {
                         x + y;
                     };

                     let result = add(five, ten);
                     !-/*5;
                     5 < 10 > 5;

                     if (5 < 10) {
                         return true;
                     } else {
                         return false;
                     }";

        let expected_output = vec![
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "five".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "ten".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "add".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Function, literal: "fn".to_string() },
            Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Token { token_type: TokenType::Identifier, literal: "x".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Identifier, literal: "y".to_string() },
            Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Token { token_type: TokenType::Identifier, literal: "x".to_string() },
            Token { token_type: TokenType::Plus, literal: "+".to_string() },
            Token { token_type: TokenType::Identifier, literal: "y".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Let, literal: "let".to_string() },
            Token { token_type: TokenType::Identifier, literal: "result".to_string() },
            Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Token { token_type: TokenType::Identifier, literal: "add".to_string() },
            Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Token { token_type: TokenType::Identifier, literal: "five".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Identifier, literal: "ten".to_string() },
            Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Bang, literal: "!".to_string() },
            Token { token_type: TokenType::Minus, literal: "-".to_string() },
            Token { token_type: TokenType::Slash, literal: "/".to_string() },
            Token { token_type: TokenType::Asterisk, literal: "*".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::LessThan, literal: "<".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::GreaterThan, literal: ">".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::If, literal: "if".to_string() },
            Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Token { token_type: TokenType::Integer, literal: "5".to_string() },
            Token { token_type: TokenType::LessThan, literal: "<".to_string() },
            Token { token_type: TokenType::Integer, literal: "10".to_string() },
            Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Token { token_type: TokenType::Return, literal: "return".to_string() },
            Token { token_type: TokenType::True, literal: "true".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            Token { token_type: TokenType::Else, literal: "else".to_string() },
            Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Token { token_type: TokenType::Return, literal: "return".to_string() },
            Token { token_type: TokenType::False, literal: "false".to_string() },
            Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            Token { token_type: TokenType::Eof, literal: "".to_string() },
        ];

        let mut tokenizer = Tokenizer::new(input);
        for expected_token in expected_output {
            let token = tokenizer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }
}
