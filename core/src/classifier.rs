use crate::TokenType;
use crate::utils;

/// Assigns a lexical category to a source fragment.
pub(crate) struct Classifier;

impl Classifier {
    /// Looks up the token type for a given lexeme.
    ///
    /// Keywords and operators receive their respective token types, legal identifiers become
    /// [`TokenType::Identifier`], digit sequences become [`TokenType::Integer`],
    /// and unrecognized fragments become [`TokenType::Illegal`].
    pub(crate) fn classify(lexeme: &str) -> TokenType {
        match lexeme {
            "" => TokenType::Eof,
            "=" => TokenType::Assign,
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Asterisk,
            "/" => TokenType::Slash,
            "<" => TokenType::LessThan,
            ">" => TokenType::GreaterThan,
            "!" => TokenType::Bang,
            "==" => TokenType::Equal,
            "!=" => TokenType::NotEqual,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            "(" => TokenType::LeftParenthesis,
            ")" => TokenType::RightParenthesis,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ if lexeme.chars().all(utils::is_legal_identifier_char) => {
                TokenType::Identifier
            }
            _ if lexeme.chars().all(|character| character.is_ascii_digit()) => {
                TokenType::Integer
            }
            _ => TokenType::Illegal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_language_lexemes() {
        assert_eq!(Classifier::classify("let"), TokenType::Let);
        assert_eq!(Classifier::classify("value"), TokenType::Identifier);
        assert_eq!(Classifier::classify("65536"), TokenType::Integer);
        assert_eq!(Classifier::classify("=="), TokenType::Equal);
        assert_eq!(Classifier::classify("é"), TokenType::Illegal);
        assert_eq!(Classifier::classify(""), TokenType::Eof);
    }
}
