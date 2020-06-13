use ::std::fmt;

use crate::io::source::SourceSlice;
use crate::util::errors::Severity;

pub type MangoResult<T> = Result<T, MangoErr>;
pub type MsgResult<T> = Result<T, ErrMsg>;

/// This is for errors that are related to specific problems with the source.
/// This is the type that should eventually be returned and reported.
#[derive(Debug, Clone)]
pub struct MangoErr {
    typ: MangoErrType,
    message: ErrMsg,
    severity: Severity,
}

pub enum MangoErrType {
    Read,
    Syntax { src: SourceSlice },
}

impl MangoErr {
    fn msg(&self) -> &ErrMsg {
        match self {
            MangoErr::Read(msg) => &msg,
            MangoErr::Syntax(msg, _) => &msg,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.msg().friendly
    }

    pub fn friendly(&self) -> &self {
        &self.msg().friendly
    }
    pub fn debug(&self) -> &self {
        &self.msg().debug
    }
}

/// This is for a plain text error, possibly with debug version.
/// This can be returned initially, in code that is removed from the source handling. It
/// should eventually be turned into `MangoErr`.
#[derive(Debug, Clone)]
pub struct ErrMsg {
    pub friendly: String,
    pub debug: Option<String>,
}

impl ErrMsg {
    pub fn new(friendly: impl Into<String>) -> Self {
        ErrMsg { friendly: friendly.into(), debug: None }
    }

    pub fn new_debug(friendly: impl Into<String>, debug: impl Into<String>) -> Self {
        ErrMsg { friendly: friendly.into(), debug: Some(debug.into()) }
    }

    pub fn as_str(&self) -> &str {
        &self.friendly
    }

    pub fn friendly(&self) -> &self {
        &self.friendly
    }

    pub fn debug(&self) -> &self {
        &self.debug
    }
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
