use mango::util::encdec::ToText;
use std::fmt::Debug;
use std::hash::Hash;

//
//pub mod tokens;
//
//pub mod collection;
//
//pub mod mock;
//

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {

}

impl ToText for Token {
    fn to_text(&self) -> String {
        unimplemented!() // todo
    }
}

pub trait IsToken: PartialEq + Eq + Hash + Debug + ToText {}

//impl<'a> PartialEq for Token + 'a {
//    #[allow(unused_variables)]
//    fn eq(&self, other: &Token) -> bool {
//        true // TODO: similar to BaseAST?
//    }
//}
//
//impl<'a> Hash for Token + 'a {
//    #[allow(unused_variables)]
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        // TODO: similar to BaseAST?
//    }
//}
