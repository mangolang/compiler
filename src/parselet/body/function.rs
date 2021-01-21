use crate::lexeme::Lexeme;

#[derive(Debug)]
pub struct ResolvedFunctionParselet {
    //TODO @mark: unsure about how to structure this one
}

/// The signature is enough to know whether a block is used. If it is not, then
#[derive(Debug)]
pub enum FunctionParselet {
    Pending(Vec<Lexeme>),
    Parsed(ResolvedFunctionParselet),
    //TODO @mark: unsure about how to structure this one
}

impl FunctionParselet {
    pub fn create(lexemes: Vec<Lexeme>) -> Self {
        FunctionParselet::Pending(lexemes)
    }

    pub fn parsed(&mut self) -> &ResolvedFunctionParselet {
        match self {
            FunctionParselet::Pending(lexemes) => {
                //TODO @mark:
                let body = ResolvedFunctionParselet {};
                *self = FunctionParselet::Parsed(body);
                if let FunctionParselet::Parsed(resolved) = self {
                    return resolved
                }
                unreachable!()
            },
            FunctionParselet::Parsed(resolved) => resolved,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_once() {
        //TODO @mark: maybe I should not need mutability?
        let mut func = FunctionParselet::Pending(vec![]);
        let body = func.parsed();
        unimplemented!()
    }
}
