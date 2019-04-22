use crate::util::strtype::StrType;
use regex::Regex;
use std::fmt;

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
    fn new<S: Into<String>>(msg: S) -> Result<Self, Msg> {
        let smsg = msg.into();
        match Msg::validate(&smsg) {
            Ok(_) => Ok(Msg { msg: smsg }),
            Err(msg) => Err(msg),
        }
    }

    fn validate(msg: &str) -> Result<(), Msg> {
        if !VALID_MESSAGE.is_match(&msg.to_string()) {
            // Make sure this is a valid string, otherwise it causes an infinite loop making error messages for it!
            return Err(Msg::from_valid("Messages should consist of printable text."));
        }
        Ok(())
    }
}
