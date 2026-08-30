/// Type definitions of tokens recognized by the tokenizer.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    /// The token type for illegal or unrecognized characters.
    Illegal,
    /// The token type for the end of the file/input.
    Eof,

    // Identifiers + Literals
    /// The token type for identifiers, e.g. variable names and function names.
    Identifier, // foobar, x, y, fn, ...
    /// The token type for integer literals.
    Integer, // 65536

    // Operators
    /// The token type for the assignment operator.
    Assign, // =
    /// The token type for the addition operator.
    Plus, // +
    /// The token type for the subtraction operator.
    Minus, // -
    /// The token type for the multiplication operator.
    Asterisk, // *
    /// The token type for the division operator.
    Slash, // /
    /// The token type for the less than operator.
    LessThan, // <
    /// The token type for the greater than operator.
    GreaterThan, // >
    /// The token type for the negation operator.
    Bang, // !
    /// The token type for the equality operator.
    Equal, // ==
    /// The token type for the not equal operator.
    NotEqual, // !=

    // Delimiters
    /// The token type for the comma delimiter.
    Comma, // ,
    /// The token type for the semicolon delimiter.
    Semicolon, // ;
    /// The token type for the left parenthesis.
    LeftParenthesis, // (
    /// The token type for the right parenthesis.
    RightParenthesis, // )
    /// The token type for the left brace.
    LeftBrace, // {
    /// The token type for the right brace.
    RightBrace, // }

    // Keywords
    /// The token type for the `fn` keyword.
    Function, // fn
    /// The token type for the `let` keyword.
    Let, // let
    /// The token type for the `true` keyword.
    True, // true
    /// The token type for the `false` keyword.
    False, // false
    /// The token type for the `if` keyword.
    If, // if
    /// The token type for the `else` keyword.
    Else, // else
    /// The token type for the `return` keyword.
    Return, // return
}

impl std::fmt::Display for TokenType {
    /// A all caps `fmt` implementation for `TokenType` to provide a string representation of the token type.
    /// This is for error messages display and debugging purposes, and is not intended for use in the tokenizer or parser.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_type_str = match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::Integer => "INTEGER",
            TokenType::Assign => "ASSIGN",
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Asterisk => "ASTERISK",
            TokenType::Slash => "SLASH",
            TokenType::LessThan => "LESS_THAN",
            TokenType::GreaterThan => "GREATER_THAN",
            TokenType::Bang => "BANG",
            TokenType::Equal => "EQUAL",
            TokenType::NotEqual => "NOT_EQUAL",
            TokenType::Comma => "COMMA",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::LeftParenthesis => "LEFT_PARENTHESIS",
            TokenType::RightParenthesis => "RIGHT_PARENTHESIS",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::If => "IF",
            TokenType::Else => "ELSE",
            TokenType::Return => "RETURN",
        };
        write!(f, "{token_type_str}")
    }
}

/// Structure representing a token with its type and literal value.
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
