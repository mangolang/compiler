use ::std::fmt;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::common::error::MsgResult;
use crate::util::strtype::StrType;
use std::borrow::Cow;

lazy_static! {
    static ref VALID_MESSAGE: Regex = Regex::new(r"^[\p{L}\d +\-_:/\\'.,]*$").unwrap();
}

/// Type for valid identifier names.
///
/// # Implementation
///
/// * Name strings are interned for fast equality checking.
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Msg {
    msg: String,
}

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StrType for Msg {
    fn new(msg: Cow<String>) -> MsgResult<Self> {
        match Msg::validate(msg.as_str()) {
            Ok(_) => Ok(Msg { msg: smsg }),
            Err(msg) => Err(msg.into_owned()),
        }
    }

    fn validate(msg: &str) -> MsgResult<()> {
        if !VALID_MESSAGE.is_match(&msg.to_string()) {
            // Make sure this is a valid string, otherwise it causes an infinite loop making error messages for it!
            return Err("Messages should consist of printable text.".into());
        }
        Ok(())
    }
}
