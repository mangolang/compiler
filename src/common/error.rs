use ::std::fmt;

use crate::common::error::MangoErrType::Read;
use crate::io::source::SourceSlice;

pub type MangoResult<T> = Result<T, MangoErr>;
pub type MsgResult<T> = Result<T, ErrMsg>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Debug,
}

/// This is for errors that are related to specific problems with the source.
/// This is the type that should eventually be returned and reported.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MangoErr {
    typ: MangoErrType,
    message: ErrMsg,
    severity: Severity,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MangoErrType {
    Read,
    Syntax { src: SourceSlice },
}

impl MangoErr {
    pub fn new(msg: impl Into<String>, severity: Severity, typ: MangoErrType) -> Self {
        MangoErr {
            typ: MangoErrType::Read,
            message: ErrMsg::new(msg),
            severity: Severity::Error,
        }
    }

    pub fn new_debug(msg: impl Into<String>, debug: impl Into<String>, severity: Severity, typ: MangoErrType) -> Self {
        MangoErr {
            typ: MangoErrType::Read,
            message: ErrMsg::new_debug(msg, debug),
            severity: Severity::Error,
        }
    }

    pub fn read(msg: impl Into<String>) -> Self {
        MangoErr::new(msg, Severity::Error, MangoErrType::Read)
    }

    pub fn syntax(msg: impl Into<String>, slice: SourceSlice) -> Self {
        MangoErr::new(msg, Severity::Error, MangoErrType::Syntax { src: slice })
    }

    fn msg(&self) -> &ErrMsg {
        &self.message
    }

    pub fn as_str(&self) -> &str {
        &self.msg().friendly
    }
}

/// This is for a plain text error, possibly with debug version.
/// This can be returned initially, in code that is removed from the source handling. It
/// should eventually be turned into `MangoErr`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ErrMsg {
    pub friendly: String,
    pub debug: Option<String>,
}

impl ErrMsg {
    pub fn new(friendly: impl Into<String>) -> Self {
        ErrMsg {
            friendly: friendly.into(),
            debug: None,
        }
    }

    pub fn new_debug(friendly: impl Into<String>, debug: impl Into<String>) -> Self {
        ErrMsg {
            friendly: friendly.into(),
            debug: Some(debug.into()),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.friendly
    }
}

impl From<String> for ErrMsg {
    fn from(text: String) -> Self {
        ErrMsg {
            friendly: text,
            debug: None,
        }
    }
}

impl From<&str> for ErrMsg {
    fn from(text: &str) -> Self {
        ErrMsg {
            friendly: text.to_owned(),
            debug: None,
        }
    }
}

impl fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.friendly)
    }
}
