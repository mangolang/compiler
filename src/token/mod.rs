use crate::util::encdec::ToText;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;

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

impl<'a> Hash for Token + 'a {
    #[allow(unused_variables)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: similar to BaseAST?
    }
}
