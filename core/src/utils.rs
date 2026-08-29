/// Identifies if a character is legal to be part of an identifier.
/// Legal characters are ASCII alphabetic characters and the underscore ('_').
pub(crate) fn is_legal_identifier_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
