use ::pest::Parser;
//use crate::pestparser::Error;
use ::pest::error::Error;
use ::pest::iterators::Tokens;
use ::pest_derive::Parser;


#[derive(Parser)]
#[grammar = "pestparser/mango.pest"]
pub struct MangoParser;

pub fn parse_string(source: &str) -> Result<Tokens<Rule>, Error<Rule>> {
    dbg!(MangoParser::parse(Rule::number, source)?);
    unimplemented!()
    //Ok(MangoParser::parse(Rule::number, source)?.next().into_inner())
}

#[test]
fn test_parse_string() {
    parse_string("-3.1415\n42\n\n");
    // assert_eq!(Rule::file, parse_string("-3.1415").unwrap());
    //unimplemented!()
}
