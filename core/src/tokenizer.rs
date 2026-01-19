use crate::structs::token::{Token, TokenType};

/// The tokenizer responsible for converting input strings into tokens.
pub struct Tokenizer {
    /// The input string to be tokenized.
    input: String,
    /// The current position in the input (points to current char).
    position: usize,
    /// The current reading position in the input (after current char).
    read_position: usize,
    /// The current character under examination.
    character: Option<char>
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

    /// Reads the next character from the input and updates the tokenizer's state
    /// to point to the new character.
    pub fn read_char(&mut self) {
        self.character = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Retrieves the token corresponding to the current character read by the tokenizer,
    /// and advances the tokenizer to the next character.
    pub fn next_token(&mut self) -> Token {
        let token = match self.character {
            Some('=') => Token { token_type: TokenType::Assign, literal: "=".to_string() },
            Some('+') => Token { token_type: TokenType::Plus, literal: "+".to_string() },
            Some(',') => Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Some(';') => Token { token_type: TokenType::Semicolon, literal: ";".to_string() },
            Some('(') => Token { token_type: TokenType::LeftParenthesis, literal: "(".to_string() },
            Some(')') => Token { token_type: TokenType::RightParenthesis, literal: ")".to_string() },
            Some('{') => Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Some('}') => Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            None => Token { token_type: TokenType::Eof, literal: "".to_string() },
            _ => Token { token_type: TokenType::Illegal, literal: self.character.unwrap().to_string() },
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
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
        let mut i = 0;
        for expected_token in expected_output {
            println!("Testing token {}: expected {:?}, got {:?}", i, expected_token, tokenizer.character);
            i += 1;
            let token = tokenizer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }
}
