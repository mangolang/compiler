use crate::lexeme::Token;
use crate::util::encdec::ToText;

/// Start and end of blocks, signalled e.g. by indentation.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct StartBlockToken {}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct EndBlockToken {
    is_dedent: bool,
    is_end_keyword: bool,
}

impl StartBlockToken {
    pub fn new() -> Self {
        StartBlockToken {}
    }
}

impl EndBlockToken {
    pub fn new(is_dedent: bool, is_end_keyword: bool) -> Self {
        assert!(is_dedent || is_end_keyword);
        EndBlockToken { is_dedent, is_end_keyword }
    }

    //TODO @mark: customization options temporarily optional
    pub fn new2() -> Self {
        Self::new(true, false)
    }
}

impl ToText for StartBlockToken {
    // TODO: needs context information to render indents
    fn to_text(&self) -> String {
        " { ".to_owned()
    }
}

impl ToText for EndBlockToken {
    // TODO: needs context information to render indents
    fn to_text(&self) -> String {
        " } ".to_owned()
    }
}

impl Token for StartBlockToken {}

impl Token for EndBlockToken {}
