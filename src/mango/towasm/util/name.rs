pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: String) -> Option<Self> {
        // todo: filter out illegal names
        return Some(Name { name });
    }
}
