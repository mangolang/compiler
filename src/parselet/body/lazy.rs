use crate::lexeme::Lexeme;

/// Only compile the body of a block when it's used. The signature must be parsed
/// immediately to be able to know whether a block is used. But compiling the body
/// can be skipped until after it is certain whether the block is used anywhere.
#[derive(Debug)]
pub enum LazyParselet<T> {
    Pending(Vec<Lexeme>),
    Parsed(T),
    //TODO @mark: unsure about how to structure this one
}

impl <T> LazyParselet<T> {
    pub fn create(lexemes: Vec<Lexeme>) -> Self {
        LazyParselet::Pending(lexemes)
    }

    pub fn parsed(&mut self) -> &T {
        match self {
            LazyParselet::Pending(lexemes) => {
                //TODO @mark:
                let body = T {};
                *self = LazyParselet::Parsed(body);
                if let LazyParselet::Parsed(resolved) = self {
                    return resolved
                }
                unreachable!()
            },
            LazyParselet::Parsed(resolved) => resolved,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_once() {
        //TODO @mark: maybe I should not need mutability?
        let mut func = LazyParselet::Pending(vec![]);
        let body = func.parsed();
        unimplemented!()
    }
}
