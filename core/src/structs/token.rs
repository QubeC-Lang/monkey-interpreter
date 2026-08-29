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

/// Structure representing a token with its type and literal value.
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
