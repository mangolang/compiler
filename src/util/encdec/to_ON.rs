// todo: implement and move (possibly to another library)
pub enum ON {
    Null,
    True,
    False,
    Integer,
    Double,
    String,
    Object,
    List,
    // todo: not complete/final
}

/// Types which have an object notation representation.
///
/// The format is somewhat similar to JSON, and can be encoded as such.
pub trait ToObjectNotation {
    #[allow(non_snake_case)]
    fn to_ON(&self) -> ON;
}
