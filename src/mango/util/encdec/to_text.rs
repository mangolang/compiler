
/// Types which have a text representation that:
///
/// * Is human readable
/// * Is not minimally formatted
/// * Is unambiguous
/// * Does not include metadata
pub trait ToText {
    fn to_text(&self) -> &str;
}
