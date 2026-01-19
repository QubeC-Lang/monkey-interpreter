/// Type definitions of tokens recognized by the tokenizer.
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + Literals
    Identifier, // foobar, x, y, fn, ...
    Integer,    // 65536

    // Operators
    Assign, // =
    Plus,   // +

    // Delimiters
    Comma,            // ,
    Semicolon,        // ;
    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftBrace,        // {
    RightBrace,       // }

    // Keywords
    Function, // fn
    Let,      // let
}

/// Structure representing a token with its type and literal value.
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
