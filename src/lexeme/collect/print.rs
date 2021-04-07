use std::fmt::Write;

use crate::lexeme::Lexeme;
use crate::common::debug::ToText;

pub fn print_lexemes(lexemes: &Vec<Lexeme>) -> String {
    let mut indent = 0;
    let mut text = String::with_capacity(lexemes.len() * 8);
    for lexeme in lexemes {
        match lexeme {
            Lexeme::Association(association) => write!(text, "{}", association.to_text()).unwrap(),
            Lexeme::Identifier(identifier) => write!(text, "{}", identifier.name).unwrap(),
            Lexeme::Keyword(keyword) => write!(text, "{}", keyword.to_text()).unwrap(),
            Lexeme::Literal(literal) => write!(text, "{}", literal.to_text()).unwrap(),
            Lexeme::Operator(operator) => write!(text, "{}", operator.to_text()).unwrap(),
            Lexeme::ParenthesisOpen(_) => write!(text, "(").unwrap(),
            Lexeme::ParenthesisClose(_) => write!(text, ")").unwrap(),
            Lexeme::BracketOpen(_) => write!(text, "[").unwrap(),
            Lexeme::BracketClose(_) => write!(text, "]").unwrap(),
            Lexeme::StartBlock(_) => {
                indent += 1;
                print_indented_nl(&mut text, indent);
            },
            Lexeme::EndBlock(_) => {
                indent -= 1;
                print_indented_nl(&mut text, indent);
            },
            Lexeme::Colon(_) => write!(text, ":").unwrap(),
            Lexeme::Comma(_) => write!(text, ",").unwrap(),
            Lexeme::Ellipsis(_) => write!(text, "...").unwrap(),
            Lexeme::Period(_) => write!(text, ".").unwrap(),
            Lexeme::Newline(_) => {},
            Lexeme::Unlexable(unlexable) => write!(text, "?{}?", unlexable.to_text()).unwrap(),
        }
        if !text.ends_with("\n") && !text.ends_with("\t") {
            write!(text, " ").unwrap()
        }
    }
    text.shrink_to_fit();
    text
}

fn print_indented_nl(buffer: &mut String, indent: u32) {
    write!(buffer, "\n").unwrap();
    (0..indent).for_each(|_| write!(buffer, "\t").unwrap());
}

#[cfg(test)]
mod tests {
    use crate::lexeme::collect::for_test::builder;

    use super::*;
    use crate::common::codeparts::operator::Symbol::{GE, EQ};

    #[test]
    fn smoke_test() {
        let lexemes = builder()
            .keyword("use")
            .identifier("std.text")
            .keyword("as")
            .identifier("txt")
            .newline()
            .keyword("main")
            .colon()
            .start_block()
            .keyword("if")
            .literal_int(2)
            .operator(GE)
            .literal_int(1)
            .colon()
            .newline()
            .start_block()
            .start_block()
            .keyword("while")
            .literal_int(0)
            .operator(EQ)
            .literal_int(0)
            .colon()
            .newline()
            .start_block()
            .keyword("if")
            .literal_text("hi")
            .operator(EQ)
            .literal_text("hi")
            .colon()
            .newline()
            .identifier("txt")
            .parenthesis_open()
            .literal_int(42)
            .parenthesis_close()
            .end_block()
            .end_block()
            .keyword("let")
            .identifier("x")
            .assignment()
            .literal_int(37)
            .end_block()
            .build();
        let text = print_lexemes(&lexemes);
        let expected = "use std.text as txt  main : \n\tif 2 >= 1 :  \n\t\t\n\t\t\twhile 0 == 0 :  \n\t\t\t\tif hi == hi :  txt ( 42 ) \n\t\t\t\n\t\tlet x = 37 \n\t";
        assert_eq!(text, expected);
    }
}
