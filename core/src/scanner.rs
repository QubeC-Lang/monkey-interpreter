/// A UTF-8-aware cursor over source text.
///
/// `Scanner` is deliberately concerned only with character traversal. It does
/// not know which characters form tokens or how those tokens are classified.
pub(crate) struct Scanner<'a> {
    /// The input string being scanned.
    input: &'a str,
    /// The current byte position in the input (points to the current character).
    position: usize,
    /// The next byte position to read (after the current character).
    read_position: usize,
    /// The current character under examination.
    character: Option<char>,
}

impl<'a> Scanner<'a> {
    /// Creates a scanner positioned at the first character of the input.
    pub(crate) fn new(input: &'a str) -> Self {
        let mut scanner = Self {
            input,
            position: 0,
            read_position: 0,
            character: None,
        };
        scanner.advance();
        scanner
    }

    /// Returns the current character without advancing the scanner's state.
    pub(crate) fn current(&self) -> Option<char> {
        self.character
    }

    /// Peeks at the next character in the input without advancing the scanner's state.
    pub(crate) fn peek(&self) -> Option<char> {
        self.input
            .get(self.read_position..)
            .and_then(|remaining| remaining.chars().next())
    }

    /// Reads the next character from the input and updates the scanner's state
    /// to point to the new character.
    pub(crate) fn advance(&mut self) {
        self.position = self.read_position;
        self.character = self
            .input
            .get(self.read_position..)
            .and_then(|remaining| remaining.chars().next());

        if let Some(character) = self.character {
            self.read_position += character.len_utf8();
        }
    }

    /// Advances while the current character satisfies the supplied predicate.
    pub(crate) fn skip_while(&mut self, predicate: impl Fn(char) -> bool) {
        while self.character.is_some_and(&predicate) {
            self.advance();
        }
    }

    /// Reads characters while they satisfy the supplied predicate and returns
    /// the corresponding slice of the original input.
    pub(crate) fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> &'a str {
        let start = self.position;
        self.skip_while(predicate);
        &self.input[start..self.position]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverses_utf8_on_character_boundaries() {
        let mut scanner = Scanner::new("é=");

        assert_eq!(scanner.current(), Some('é'));
        assert_eq!(scanner.peek(), Some('='));
        assert_eq!(scanner.take_while(|character| character != '='), "é");
        assert_eq!(scanner.current(), Some('='));
    }
}
