use std::fmt::Debug;

#[derive(Debug)]
pub enum MaybeEnd<T: Debug> {
    Item(T),
    End,
}

#[derive(Debug)]
pub enum MaybeMatch<T: Debug> {
    Item(T),
    NoMatch,
    End,
}

impl <T: Debug> Try for MaybeEnd<T>