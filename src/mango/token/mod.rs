use std::fmt::Debug;
use mango::util::encdec::ToText;

//
//pub mod tokens;
//
//pub mod collection;
//
//pub mod mock;
//
pub trait Token: ToText + Debug {}

impl<'a> PartialEq for Token + 'a {
    #[allow(unused_variables)]
    fn eq(&self, other: &Token) -> bool {
        true // TODO: similar to BaseAST?
    }
}
