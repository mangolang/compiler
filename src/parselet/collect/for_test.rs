
use crate::io::slice::SourceSlice;
use crate::lexeme::identifier::SimpleIdentifierLexeme;
use crate::lexeme::lexemes::literal::TextLiteralLexeme;
use crate::parselet::body::code_body::CodeBodyParselet;
use crate::parselet::files::import::ImportParselet;
use crate::parselet::signature::function::FunctionParselet;
use crate::parselet::signature::parameters::{ParametersParselet, ParamParselets, TypedValueParselet};
use crate::parselet::signature::test_parselet::TestParselet;
use crate::parselet::signature::typ::TypeParselet;
use crate::parselet::signature::test_parselet::TestName;
use crate::lexeme::{FQIdentifierLexeme, Lexeme};
use crate::parselet::signature::entrypoint::EntryPointParselet;

pub fn import(fqn: impl AsRef<str>) -> ImportParselet {
    ImportParselet::new(FQIdentifierLexeme::from_str(fqn.as_ref(), SourceSlice::mock()).unwrap(), None)
}

pub fn import_alias(fqn: impl AsRef<str>, alias: impl AsRef<str>) -> ImportParselet {
    ImportParselet::new(
        FQIdentifierLexeme::from_str(fqn.as_ref(), SourceSlice::mock()).unwrap(),
        Some(SimpleIdentifierLexeme::from_str(alias.as_ref(), SourceSlice::mock()).unwrap()),
    )
}

pub fn entrypoint(name: Option<&str>, body: impl Into<Vec<Lexeme>>) -> EntryPointParselet {
    EntryPointParselet::new(
        name.map(|n| SimpleIdentifierLexeme::from_str(n, SourceSlice::mock()).unwrap()),
        CodeBodyParselet::new(body),
    )
}

pub fn function(name: impl AsRef<str>, params: impl Into<ParamParselets>, return_name: impl AsRef<str>, body: impl Into<Vec<Lexeme>>) -> FunctionParselet {
    FunctionParselet::new(
        SimpleIdentifierLexeme::from_str(name, SourceSlice::mock()).unwrap(),
        ParametersParselet::new(params.into()),
        TypeParselet::new(SimpleIdentifierLexeme::from_str(return_name, SourceSlice::mock()).unwrap()),
        CodeBodyParselet::new(body),
    )
}

pub fn text_test(name: impl AsRef<str>, body: impl Into<Vec<Lexeme>>) -> TestParselet {
    TestParselet::new(
        TestName::from(TextLiteralLexeme::new(name, SourceSlice::mock())),
        CodeBodyParselet::new(body),
    )
}

pub fn param(name: impl AsRef<str>, typ: impl AsRef<str>) -> TypedValueParselet {
    TypedValueParselet::new(
        SimpleIdentifierLexeme::from_str(name.as_ref(), SourceSlice::mock()).unwrap(),
        TypeParselet::new(SimpleIdentifierLexeme::from_str(typ.as_ref(), SourceSlice::mock()).unwrap()),
    )
}
