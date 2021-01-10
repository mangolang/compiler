pub trait CharOps {
    /// Remove all matching characters from the string.
    // Signature may be changed to support a set of characters, if the need arises.
    fn without_char(&self, strip: char) -> String;

    fn char_len(&self) -> usize;
}

impl<'a> CharOps for &'a str {
    fn without_char(&self, strip: char) -> String {
        self.chars().filter(|chr| chr != &strip).collect()
    }

    fn char_len(&self) -> usize {
        self.chars().count()
    }
}

impl CharOps for String {
    fn without_char(&self, strip: char) -> String {
        self.chars().filter(|chr| chr != &strip).collect()
    }

    fn char_len(&self) -> usize {
        self.chars().count()
    }
}
