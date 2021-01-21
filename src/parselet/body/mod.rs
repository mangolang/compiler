use crate::lexeme::Lexeme;

/// The signature is enough to know whether a block is used. If it is not, then
#[derive(Debug)]
pub struct ResolvedBodyParselet {
    //TODO @mark: unsure about how to structure this one
}

/// The signature is enough to know whether a block is used. If it is not, then
#[derive(Debug)]
pub enum BodyParselet {
    Pending(Vec<Lexeme>),
    Parsed(ResolvedBodyParselet),
    //TODO @mark: unsure about how to structure this one
}

impl BodyParselet {
    fn parsed(&mut self) -> &ResolvedBodyParselet {
        match self {
            BodyParselet::Pending(lexemes) => {
                //TODO @mark:
                let body = ResolvedBodyParselet {};
                *self = BodyParselet::Parsed(body);
                if let BodyParselet::Parsed(resolved) = self {
                    return resolved
                }
                unreachable!()
            },
            BodyParselet::Parsed(resolved) => resolved,
        }
    }
}
