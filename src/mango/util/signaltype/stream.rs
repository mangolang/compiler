use mango::util::strtype::StrType;
use regex::Regex;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum StreamElem<T> {
    Elem(T),
    End,
}
