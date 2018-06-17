/// Remove all matching characters from the string.
// Signature may be changed to support a set of characters, if the need arises.
pub fn char_drop<S: Into<String>>(text: S, strip: &char) -> String {
    let text = text.into();
    text.chars().filter(|chr| chr != strip).collect()
}
