use crate::util::encdec::ToText;
use crate::io::slice::{SourceLocation, SourceSlice};

/// Start and end of blocks, signalled e.g. by indentation.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct StartBlockLexeme {
    source: SourceSlice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct EndBlockLexeme {
    is_dedent: bool,
    is_end_keyword: bool,
    source: SourceSlice,
}

impl StartBlockLexeme {
    pub fn new(source: SourceSlice) -> Self {
        StartBlockLexeme { source }
    }
}

impl EndBlockLexeme {
    pub fn new(is_dedent: bool, is_end_keyword: bool, source: SourceSlice) -> Self {
        assert!(is_dedent || is_end_keyword);
        EndBlockLexeme { is_dedent, is_end_keyword, source }
    }

    //TODO @mark: customization options temporarily optional
    pub fn new2() -> Self {
        Self::new(true, false)
    }
}

impl SourceLocation for StartBlockLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl SourceLocation for EndBlockLexeme {
    fn source(&self) -> &SourceSlice {
        &self.source
    }
}

impl ToText for StartBlockLexeme {
    // TODO: needs context information to render indents
    fn to_text(&self) -> String {
        " { ".to_owned()
    }
}

impl ToText for EndBlockLexeme {
    // TODO: needs context information to render indents
    fn to_text(&self) -> String {
        " } ".to_owned()
    }
}
