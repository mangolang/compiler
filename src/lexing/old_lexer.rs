use std::str::FromStr;

use smallvec::smallvec;

use crate::io::typ::Reader;
use crate::io::typ::ReaderResult::*;
use crate::lexing::string_lexer::StringLexer;
use crate::lexing::typ::SubLexer;
use crate::lexing::typ::SubLexerResult;
use crate::token::collect::all::TokenVec;
use crate::token::special::UnlexableToken;
use crate::token::Tokens;
use crate::token::tokens::AssociationToken;
use crate::token::tokens::EndBlockToken;
use crate::token::tokens::EndStatementToken;
use crate::token::tokens::IdentifierToken;
use crate::token::tokens::KeywordToken;
use crate::token::tokens::literal::LiteralToken;
use crate::token::tokens::OperatorToken;
use crate::token::tokens::ParenthesisCloseToken;
use crate::token::tokens::ParenthesisOpenToken;
use crate::token::tokens::StartBlockToken;
use crate::util::strslice::char_ops::CharOps;
use crate::util::strslice::charsliceto;



// TODO: keep the regexes in thread local global scope storage

impl CodeLexer {
    pub fn new() -> Self {
        CodeLexer { indent: 0 }
    }

    fn lex_indents(&mut self, reader: &mut impl Reader) -> Vec<Tokens> {
        let mut line_indent = 0;
        while let Match(_) = reader.matches("\\t") {
            line_indent += 1;
        }
        let mut tokens: Vec<Tokens> = Vec::with_capacity(8);
        if line_indent < self.indent {
            if let Match(_) = reader.matches(r"end") {
                // If this is followed by an 'end' keyword, then that 'end' is redundant.
                tokens.push(Tokens::EndBlock(EndBlockToken::new(true, true)));
            } else {
                tokens.push(Tokens::EndBlock(EndBlockToken::new(true, false)));
            }
            for _ in line_indent..(self.indent - 1) {
                // This line is dedented, make end tokens.
                tokens.push(Tokens::EndBlock(EndBlockToken::new(true, false)));
            }
        } else {
            for _ in self.indent..line_indent {
                // This line is indented, make start tokens.
                // TODO: increasing indent by more than one should be a warning
                tokens.push(Tokens::StartBlock(StartBlockToken::new()));
            }
        }
        self.indent = line_indent;
        tokens
    }

    fn token_and_indents(&mut self, reader: &mut impl Reader, token: Tokens) -> SubLexerResult {
        let mut tokens: TokenVec = smallvec![token];
        // This is a new line, so there may be indents.
        tokens.extend(self.lex_indents(reader));
        SubLexerResult::Result(tokens)
    }
}

impl SubLexer for CodeLexer {
    fn lex_pass(&mut self, reader: &mut impl Reader) -> SubLexerResult {
        use self::SubLexerResult::*;

        // End of line continuation
        if let Match(_) = reader.matches(r"\.\.\.") {
            // Line continuation has no token, it just continues on the next line, ignoring indents (for now).
            if let Match(_) = reader.matches(r"\n\r?\t*") {
                // There should always be a newline after continuations, so that they can be ignored together.
            } else {
                // The rest of this line is unparsable.
                if let Match(word) = reader.matches("[^\\n]*\\n\\r?") {
                    // This is a new line, so there may be indents.
                    return self.token_and_indents(reader, Tokens::Unlexable(UnlexableToken::new(word)));
                } else {
                    // TODO: I don't know yet how to deal with '...' followed by end-of-file
                    panic!()
                }
            }
        }
        // Newlines
        if let Match(_) = reader.matches("\\n\\r?") {
            // Newline WITHOUT line continuation.
            // This is a new line, so there may be indents.
            return self.token_and_indents(reader, Tokens::EndStatement(EndStatementToken::new_end_line()));
        }
        // End of statement
        if let Match(_) = reader.matches(";") {
            // Semicolon, which ends a statement.
            if let Match(_) = reader.matches("\\n\\r?") {
                // If semicolon is followed by a newline, it is redundant. Deal with indents (but ignore the newline itself).
                return self.token_and_indents(reader, Tokens::EndStatement(EndStatementToken::new_semicolon()));
            } else {
                return SubLexerResult::single(Tokens::EndStatement(EndStatementToken::new_semicolon()));
            }
        }
        //
        // Indentation done; do the rest of lexing.
        //
        // Parse identifiers and keywords. This assumes that keywords are a subset of identifiers.
        if let Match(word) = reader.matches(IdentifierToken::subpattern()) {
            // TODO: maybe turn identifier into keyword to avoid a string copy? kind of elaborate...
            if let Ok(keyword) = KeywordToken::from_str(&word) {
                return SubLexerResult::single(Tokens::Keyword(keyword));
            }
            return SubLexerResult::single(Tokens::Identifier(IdentifierToken::from_str(&word).unwrap()));
        }
        // Literal
        if let Match(_) = reader.matches("[a-z]?\"") {
            return Delegate(Box::new(StringLexer::new_double_quoted()));
        }
        if let Match(nr) = reader.matches(LiteralToken::subpattern_int()) {
            let value = LiteralToken::parse_int(nr);
            return SubLexerResult::single(Tokens::Literal(LiteralToken::Int(value)));
        }
        if let Match(nr) = reader.matches(LiteralToken::subpattern_real()) {
            let value = LiteralToken::parse_real(nr);
            return SubLexerResult::single(Tokens::Literal(LiteralToken::Real(value)));
        }

        // Association (before operator)
        if let Match(token) = reader.matches(&AssociationToken::subpattern()) {
            debug_assert!(token.ends_with('='));
            if token.char_len() > 1 {
                match AssociationToken::from_str(&charsliceto(token, -1)) {
                    Ok(association) => return SubLexerResult::single(Tokens::Association(association)),
                    Err(msg) => panic!(format!("Invalid association prefix: {}", msg)),
                }
            } else {
                return SubLexerResult::single(Tokens::Association(AssociationToken::from_unprefixed()));
            }
        }
        // Operator (after association)
        if let Match(token) = reader.matches(OperatorToken::subpattern()) {
            return SubLexerResult::single(Tokens::Operator(OperatorToken::from_str(&token).unwrap()));
        }
        // Grouping symbols
        if let Match(_) = reader.matches(r"\(") {
            return SubLexerResult::single(Tokens::ParenthesisOpen(ParenthesisOpenToken::new()));
        }
        if let Match(_) = reader.matches(r"\)") {
            return SubLexerResult::single(Tokens::ParenthesisClose(ParenthesisCloseToken::new()));
        }

        // If the code gets here, it did not recognize the text as any token
        match reader.matches(r"[^\s]+") {
            Match(word) => SubLexerResult::single(Tokens::Unlexable(UnlexableToken::new(word))),
            NoMatch => panic!("Do not know how to proceed with parsing"),
            EOF => {
                if self.indent < 0 {
                    return SubLexerResult::End;
                }
                let mut tokens = smallvec![Tokens::EndStatement(EndStatementToken::new_end_line())];
                for _ in 0..self.indent {
                    // This line is dedented, make end tokens.
                    tokens.push(Tokens::EndBlock(EndBlockToken::new(true, false)));
                }
                self.indent = -1;
                SubLexerResult::Result(tokens)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::lexing::util::test_util::assert_text_to_tokens;
    use crate::token::Tokens;
    use crate::token::tokens::EndStatementToken;
    use crate::token::tokens::KeywordToken;

    #[test]
    fn test_lexing_individual() {
        assert_text_to_tokens(
            "if",
            vec![
                Tokens::Keyword(KeywordToken::from_str("if").unwrap()),
                Tokens::EndStatement(EndStatementToken::new_end_line()),
            ],
        );
        // todo: more
    }
}
