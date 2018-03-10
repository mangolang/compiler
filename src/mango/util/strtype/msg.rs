use regex::Regex;
use super::StrType;
use std::fmt;

lazy_static! {
    static ref VALID_MESSAGE: Regex = Regex::new(r"^[\p{L}\d +\-_:/\\']*$").unwrap();
}

/// Type for valid identifier names.
///
/// # Implementation
///
/// * Name strings are interned for fast equality checking.
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Msg {
    msg: String
}

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.msg);
    }
}

///// Note that using this causes a panic for invalid inputs.
//impl From<String> for Msg {
//    fn from(msg: String) -> Msg {
//        return Msg::new(msg).unwrap();
//    }
//}
//
//impl From<&'static str> for Msg {
//    fn from(msg: &str) -> Msg {
//        return Msg::new(msg.to_owned()).unwrap();
//    }
//}

impl StrType for Msg {
    fn new<S>(msg: S) -> Result<Msg, Msg> where S: ToString {
        // todo: prevent this to_string stuff
        return match Msg::validate(&msg.to_string()) {
            Ok(txt) => Ok(Msg { msg: txt.to_string() }),
            Err(msg) => Err(msg)
        }
    }

    fn validate(msg: &str) -> Result<&str, Msg> {
        if ! VALID_MESSAGE.is_match(&msg.to_string()) {
            return Err(Msg::from_valid("Messages should consist of printable text.".to_owned()));
        }
        return Ok(msg);
    }
}
