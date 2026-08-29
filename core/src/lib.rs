mod classifier;
mod scanner;
mod structs;
mod tokenizer;
mod utils;

pub mod repl;

pub use structs::token::{Token, TokenType};
pub use tokenizer::Tokenizer;

/// A function that always returns None of type Option<()>
/// as this is used as a placeholder for the core function of
/// the library that is yet to be implemented.
pub fn nothing() -> Option<()> {
    None
}
