use ::std::fmt;

use crate::io::source::SourceSlice;

pub type MangoResult<T> = Result<T, MangoErr>;
pub type MsgResult<T> = Result<T, ErrMsg>;

/// This is for errors that are related to specific problems with the source.
/// This is the type that should eventually be returned and reported.
#[derive(Debug, Clone)]
pub enum MangoErr {
    Read { friendly: String, debug: Option<String> },
    //TODO @mark: SourceSlice will have to refer to Rc SourceFile, because current borrow way, errors cannot get to a higher level than SourceFile
    //TODO @mark: I thought about making a borrowed and an Rc version, but apparently it's not worth it https://stackoverflow.com/q/31264670
    Syntax { friendly: String, debug: Option<String>, src: SourceSlice },
}

/// This is for a plain text error, possibly with debug version.
/// This can be returned initially, in code that is removed from the source handling. It
/// should eventually be turned into `MangoErr`.
#[derive(Debug, Clone)]
pub struct ErrMsg {
    friendly: String,
    debug: Option<String>,
}

impl From<String> for ErrMsg {
    fn from(text: String) -> Self {
        ErrMsg { friendly: text, debug: None }
    }
}

impl From<&str> for ErrMsg {
    fn from(text: &str) -> Self {
        ErrMsg { friendly: text.to_owned(), debug: None }
    }
}

impl fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.friendly)
    }
}
