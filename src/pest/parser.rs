use pest::Parser;

pub fn parse_string(source: &str) {
    unimplemented!();
}

#[derive(Parser)]
#[grammar = "mango.pest"]
pub struct MangoParser;
