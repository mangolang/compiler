#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum StreamElem<T> {
    Elem(T),
    End,
}
