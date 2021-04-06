use crate::lexeme::Lexeme;

pub trait Parseable {
    fn parse(lexemes: &[Lexeme]) -> Self;
}

/// Only compile the body of a block when it's used. The signature must be parsed
/// immediately to be able to know whether a block is used. But compiling the body
/// can be skipped until after it is certain whether the block is used anywhere.
#[derive(Debug, PartialEq, Eq)]
pub enum LazyParselet<T: Parseable> {
    Pending(Vec<Lexeme>),
    Parsed(T),
    //TODO @mark: unsure about how to structure this one
}

impl<T: Parseable> LazyParselet<T> {
    pub fn create(lexemes: Vec<Lexeme>) -> Self {
        LazyParselet::Pending(lexemes)
    }

    pub fn parsed(&mut self) -> &T {
        match self {
            LazyParselet::Pending(lexemes) => {
                //TODO @mark:
                let body = T::parse(&lexemes);
                *self = LazyParselet::Parsed(body);
                if let LazyParselet::Parsed(resolved) = self {
                    return resolved;
                }
                unreachable!()
            }
            LazyParselet::Parsed(resolved) => resolved,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexeme::collect::for_test::builder;

    use super::*;

    struct TestParseable(u32);

    impl Parseable for TestParseable {
        fn parse(lexemes: &[Lexeme]) -> Self {
            TestParseable(lexemes.len() as u32)
        }
    }

    #[test]
    fn resolve_1() {
        let mut func: LazyParselet<TestParseable> = LazyParselet::create(vec![]);
        let body = func.parsed();
        assert_eq!(body.0, 0);
    }

    #[test]
    fn resolve_2() {
        //TODO @mark: maybe I should not need mutability?
        let mut func: LazyParselet<TestParseable> = LazyParselet::create(vec![builder().period().build_only()]);
        let body = func.parsed();
        assert_eq!(body.0, 1);
    }
}
