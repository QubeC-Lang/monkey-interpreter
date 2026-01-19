/// Type definitions of tokens recognized by the tokenizer.
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    /// The token type for illegal or unrecognized characters.
    Illegal,
    /// The token type for the end of the file/input.
    Eof,

    // Identifiers + Literals
    /// The token type for identifiers, e.g. variable names and function names.
    Identifier, // foobar, x, y, fn, ...
    /// The token type for integer literals.
    Integer,    // 65536

    // Operators
    /// The token type for the assignment operator.
    Assign, // =
    /// The token type for the addition operator.
    Plus,   // +

    // Delimiters
    /// The token type for the comma delimiter.
    Comma,            // ,
    /// The token type for the semicolon delimiter.
    Semicolon,        // ;
    /// The token type for the left parenthesis.
    LeftParenthesis,  // (
    /// The token type for the right parenthesis.
    RightParenthesis, // )
    /// The token type for the left brace.
    LeftBrace,        // {
    /// The token type for the right brace.
    RightBrace,       // }

    // Keywords
    /// The token type for the `fn` keyword.
    Function, // fn
    /// The token type for the `let` keyword.
    Let,      // let
}

/// Structure representing a token with its type and literal value.
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

/// Looks up the token type for a given identifier string.
/// If the identifier matches a keyword, returns the corresponding [`TokenType`](TokenType);
/// otherwise, returns [`TokenType::Identifier`](TokenType::Identifier).
pub fn identifier_lookup(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        _ => TokenType::Identifier,
    }
}
