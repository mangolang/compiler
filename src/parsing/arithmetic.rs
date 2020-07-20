use crate::lexing::util::lex_list::LexLis;
use crate::lexeme::Lexemes;

pub fn parse_addition(lex: LexList) -> Lexemes {
    //TODO @mark:
    let lhs = parse_multiplication(lex);
    unimplemented!()

//    lex
//    val lhsMultiplication = parseMultiplication(listener, lexemes)
//    val maybeOperator = lexemes.peek()
//    if (maybeOperator is OperatorLexeme && maybeOperator.isAddSub) {
//        /* Attempt to parse `Multiplication ("+" | "-") Multiplication`. */
//        lexemes.take()
//        val rhsMultiplication = parseExpression(listener, lexemes)
//        return ConcreteBinaryOperation(
//            lhsMultiplication,
//            ConcreteBinaryOperator(maybeOperator),
//            rhsMultiplication
//        )
//    }
//    /* Parsing `Multiplication ("+" | "-") Multiplication` failed, just use Multiplication. */
//    return lhsMultiplication
}

pub fn parse_multiplication(lex: LexList) {}

pub fn parse_unary(lex: LexList) {}
