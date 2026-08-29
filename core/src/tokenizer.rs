use crate::classifier::Classifier;
use crate::scanner::Scanner;
use crate::structs::token::{Token, TokenType};
use crate::utils;

/// The tokenizer responsible for converting input strings into tokens.
///
/// It extracts lexemes from the input and delegates their logical
/// classification to [`Classifier`].
pub struct Tokenizer<'a> {
    /// The scanner used to traverse the input string being tokenized.
    scanner: Scanner<'a>,
    /// Whether the iterator interface has already emitted the end-of-input token.
    finished: bool,
}

impl<'a> Tokenizer<'a> {
    /// Creates a tokenizer positioned at the first character of the input.
    pub fn new(input: &'a str) -> Self {
        Self {
            scanner: Scanner::new(input),
            finished: false,
        }
    }

    /// Reads an identifier from the input starting at the current position
    /// until a non-identifier character is encountered.
    /// Returns the identifier literal as a `String`.
    fn read_identifier(&mut self) -> String {
        self.scanner
            .take_while(utils::is_legal_identifier_char)
            .to_string()
    }

    /// Reads a number from the input starting at the current position
    /// until a non-digit character is encountered.
    /// Returns the number literal as a `String`.
    fn read_number(&mut self) -> String {
        self.scanner
            .take_while(|character| character.is_ascii_digit())
            .to_string()
    }

    /// Reads either a one-character operator or its two-character form when
    /// the expected second character follows it.
    fn read_operator(&mut self, second_character: char) -> String {
        let first_character = self.scanner.current().expect("operator must have a first character");

        if self.scanner.peek() == Some(second_character) {
            self.scanner.advance();
            self.scanner.advance();
            format!("{first_character}{second_character}")
        } else {
            self.scanner.advance();
            first_character.to_string()
        }
    }

    /// Reads the current character as a single lexeme and advances the scanner.
    fn read_single_character_lexeme(&mut self) -> String {
        let character = self.scanner.current().expect("lexeme must have a character");
        self.scanner.advance();
        character.to_string()
    }

    /// Retrieves the lexeme beginning at the current character and advances
    /// the tokenizer to the first character after that lexeme.
    fn next_lexeme(&mut self) -> String {
        self.scanner.skip_while(char::is_whitespace);

        match self.scanner.current() {
            Some('=') | Some('!') => self.read_operator('='),
            Some(character) if utils::is_legal_identifier_char(character) => {
                // Do not advance again here: read_identifier already advances
                // to the first character after the identifier.
                self.read_identifier()
            }
            Some(character) if character.is_ascii_digit() => {
                // Do not advance again here: read_number already advances
                // to the first character after the number.
                self.read_number()
            }
            Some(_) => self.read_single_character_lexeme(),
            None => String::new(),
        }
    }

    /// Retrieves the token corresponding to the next lexeme read by the tokenizer.
    /// Returns an `Eof` token once the input is exhausted.
    pub fn next_token(&mut self) -> Token {
        let literal = self.next_lexeme();
        let token_type = Classifier::classify(&literal);
        Token { token_type, literal }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let token = self.next_token();
        self.finished = token.token_type == TokenType::Eof;
        Some(token)
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
    fn iterates_through_eof_once() {
        let token_types: Vec<_> = Tokenizer::new("let x = 1;")
            .map(|token| token.token_type)
            .collect();

        assert_eq!(
            token_types,
            vec![
                TokenType::Let,
                TokenType::Identifier,
                TokenType::Assign,
                TokenType::Integer,
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn reports_non_ascii_characters_without_panicking() {
        let tokens: Vec<_> = Tokenizer::new("é=").collect();

        assert_eq!(tokens[0].token_type, TokenType::Illegal);
        assert_eq!(tokens[0].literal, "é");
        assert_eq!(tokens[1].token_type, TokenType::Assign);
        assert_eq!(tokens[2].token_type, TokenType::Eof);
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
