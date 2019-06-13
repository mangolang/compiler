use ::pest::Parser;
//use crate::pestparser::Error;
use ::pest::error::Error;
use ::pest::iterators::Pairs;
use ::pest::iterators::Tokens;

#[derive(Parser)]
#[grammar = "pestparser/mango.pest"]
pub struct MangoParser;

pub fn parse_string(source: &str) -> Result<Tokens<Rule>, Error<Rule>> {
    Ok(MangoParser::parse(Rule::number, source)?.tokens())
}

#[test]
fn test_parse_string() {
    assert_eq!(Rule::number, parse_string("-3.1415").unwrap());
}
