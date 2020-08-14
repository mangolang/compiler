//TODO @mark:

// #[test]
// fn test_nested_ast_eq() {
//     let twin_one = BinaryOperationParselet::new(
//         Parselets::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
//         OperatorParselet::from_symbol(BinOpSymbol::Plus),
//         Parselets::UnaryOperation(UnaryOperationParselet::new(
//             OperatorParselet::from_symbol(BinOpSymbol::Plus),
//             Parselets::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
//         )),
//     );
//     let twin_two = BinaryOperationParselet::new(
//         Parselets::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
//         OperatorParselet::from_symbol(BinOpSymbol::Plus),
//         Parselets::UnaryOperation(UnaryOperationParselet::new(
//             OperatorParselet::from_symbol(BinOpSymbol::Plus),
//             Parselets::Literal(LiteralParselet::Int(IntLiteralParselet::new(7))),
//         )),
//     );
//     assert_eq!(twin_one, twin_two);
//     // todo: hash not implemented, so not tested yet
//     // assert_eq!(calculate_hash(&twin_one), calculate_hash(&twin_two));
// }
//
// #[test]
// fn test_simple_ast_eq_ne() {
//     let nodes = vec![
//         Parselets::Operator(OperatorParselet::from_symbol(BinOpSymbol::Plus)),
//         Parselets::Literal(LiteralParselet::Int(IntLiteralParselet::new(1))),
//         Parselets::Literal(LiteralParselet::Float(FloatLiteralParselet::new(1.))),
//         Parselets::Literal(LiteralParselet::String(StringLiteralParselet::new("1".to_string()))),
//         Parselets::UnaryOperation(UnaryOperationParselet::new(
//             OperatorParselet::from_symbol(BinOpSymbol::Dash),
//             Parselets::Literal(LiteralParselet::Int(IntLiteralParselet::new(1))),
//         )),
//         Parselets::BinaryOperation(BinaryOperationParselet::new(
//             Parselets::Literal(LiteralParselet::Float(FloatLiteralParselet::new(1.))),
//             OperatorParselet::from_symbol(BinOpSymbol::Plus),
//             Parselets::Literal(LiteralParselet::Int(IntLiteralParselet::new(1))),
//         )),
//         Parselets::Variable(VariableParselet::new(Name::from_valid("my_var"))),
//         Parselets::Assignment(AssignmentParselet::new(
//             VariableParselet::new(Name::from_valid("my_var")),
//             Parselets::Literal(LiteralParselet::String(StringLiteralParselet::new("1".to_string()))),
//         )),
//     ];
//     for (i, left) in nodes.iter().enumerate() {
//         for (j, right) in nodes.iter().enumerate() {
//             if i == j {
//                 assert_eq!(left, right);
//             } else {
//                 assert_ne!(left, right);
//             }
//         }
//     }
// }
//
// #[test]
// fn test_unparseable_equality() {
//     let unp: UnparseableParselet;
//     unp = UnparseableParselet::from_lexemes(vec![
//         Lexemes::Identifier(IdentifierLexeme::from_str("x").unwrap()),
//         Lexemes::Operator(OperatorLexeme::from_str("<").unwrap()),
//         Lexemes::Literal(LiteralLexeme::Int(128)),
//     ]);
//     assert_eq!(unp, unp);
//     let unp2 = UnparseableParselet::from_lexemes(vec![
//         Lexemes::Identifier(IdentifierLexeme::from_str("y").unwrap()),
//         Lexemes::Operator(OperatorLexeme::from_str("<").unwrap()),
//         Lexemes::Literal(LiteralLexeme::Int(128)),
//     ]);
//     assert_ne!(unp, unp2);
// }
