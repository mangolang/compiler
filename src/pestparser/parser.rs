use ::pest::Parser;
//use crate::pestparser::Error;
use pest::error::Error;
use ::pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "pestparser/mango.pest"]
pub struct MangoParser;

pub fn parse_string(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    MangoParser::parse(Rule::field, source)
}

#[test]
fn test_parse_string() {
    MangoParser::parse(Rule::field, "-3.1415");
}
