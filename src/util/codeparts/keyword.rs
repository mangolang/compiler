use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as fResult;
use ::std::str::FromStr;

use ::lazy_static::lazy_static;

use crate::common::error::{MangoErr, MangoResult};
use crate::token::Token;
use crate::util::strtype::StrType;

/// The different operator codeparts that are recognized.
// TODO: reserve a lot of keywords; easier to remove than add (compatibility)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Keyword {
    Let,
    Mut,
    If,
    For,
    While,
    Function,
    Return,
    Reserved(String),
}

lazy_static! {
    // Note: keywords must follow the same rules as identifiers, or the lexer will
    // not recognize them. For example, no multi-word keywords.
    pub static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut keywords = HashMap::with_capacity(150);
        keywords.insert("let", Keyword::Let);
        keywords.insert("mut", Keyword::Mut);
        keywords.insert("if", Keyword::If);
        keywords.insert("for", Keyword::For);
        keywords.insert("while", Keyword::While);
        keywords.insert("fun", Keyword::Function);
        keywords.insert("return", Keyword::Return);
        keywords.insert("abstract", Keyword::Reserved("abstract".to_owned()));
        keywords.insert("alias", Keyword::Reserved("alias".to_owned()));
        keywords.insert("all", Keyword::Reserved("all".to_owned()));
        keywords.insert("and", Keyword::Reserved("and".to_owned()));
        keywords.insert("annotation", Keyword::Reserved("annotation".to_owned()));
        keywords.insert("any", Keyword::Reserved("any".to_owned()));
        keywords.insert("as", Keyword::Reserved("as".to_owned()));
        keywords.insert("assert", Keyword::Reserved("assert".to_owned()));
        keywords.insert("async", Keyword::Reserved("async".to_owned()));
        keywords.insert("auto", Keyword::Reserved("auto".to_owned()));
        keywords.insert("await", Keyword::Reserved("await".to_owned()));
        keywords.insert("become", Keyword::Reserved("become".to_owned()));
        keywords.insert("bool", Keyword::Reserved("bool".to_owned()));
        keywords.insert("box", Keyword::Reserved("box".to_owned()));
        keywords.insert("break", Keyword::Reserved("break".to_owned()));
        keywords.insert("by", Keyword::Reserved("by".to_owned()));
        keywords.insert("byte", Keyword::Reserved("byte".to_owned()));
        keywords.insert("catch", Keyword::Reserved("catch".to_owned()));
        keywords.insert("class", Keyword::Reserved("class".to_owned()));
        keywords.insert("closed", Keyword::Reserved("closed".to_owned()));
        keywords.insert("companion", Keyword::Reserved("companion".to_owned()));
        keywords.insert("const", Keyword::Reserved("const".to_owned()));
        keywords.insert("constructor", Keyword::Reserved("constructor".to_owned()));
        keywords.insert("continue", Keyword::Reserved("continue".to_owned()));
        keywords.insert("data", Keyword::Reserved("data".to_owned()));
        keywords.insert("debug", Keyword::Reserved("debug".to_owned()));
        keywords.insert("def", Keyword::Reserved("def".to_owned()));
        keywords.insert("default", Keyword::Reserved("default".to_owned()));
        keywords.insert("defer", Keyword::Reserved("defer".to_owned()));
        keywords.insert("del", Keyword::Reserved("del".to_owned()));
        keywords.insert("delegate", Keyword::Reserved("delegate".to_owned()));
        keywords.insert("delegates", Keyword::Reserved("delegates".to_owned()));
        keywords.insert("delete", Keyword::Reserved("delete".to_owned()));
        keywords.insert("derive", Keyword::Reserved("derive".to_owned()));
        keywords.insert("deriving", Keyword::Reserved("deriving".to_owned()));
        keywords.insert("do", Keyword::Reserved("do".to_owned()));
        keywords.insert("double", Keyword::Reserved("double".to_owned()));
        keywords.insert("dynamic", Keyword::Reserved("dynamic".to_owned()));
        keywords.insert("elementwise", Keyword::Reserved("elementwise".to_owned()));
        keywords.insert("elif", Keyword::Reserved("elif".to_owned()));
        keywords.insert("end", Keyword::Reserved("end".to_owned()));
        keywords.insert("enum", Keyword::Reserved("enum".to_owned()));
        keywords.insert("eval", Keyword::Reserved("eval".to_owned()));
        keywords.insert("except", Keyword::Reserved("except".to_owned()));
        keywords.insert("extends", Keyword::Reserved("extends".to_owned()));
        keywords.insert("extern", Keyword::Reserved("extern".to_owned()));
        keywords.insert("false", Keyword::Reserved("false".to_owned()));
        keywords.insert("family", Keyword::Reserved("family".to_owned()));
        keywords.insert("field", Keyword::Reserved("field".to_owned()));
        keywords.insert("final", Keyword::Reserved("final".to_owned()));
        keywords.insert("finally", Keyword::Reserved("finally".to_owned()));
        keywords.insert("float", Keyword::Reserved("float".to_owned()));
        keywords.insert("fn", Keyword::Reserved("fn".to_owned()));
        keywords.insert("get", Keyword::Reserved("get".to_owned()));
        keywords.insert("global", Keyword::Reserved("global".to_owned()));
        keywords.insert("goto", Keyword::Reserved("goto".to_owned()));
        keywords.insert("impl", Keyword::Reserved("impl".to_owned()));
        keywords.insert("implements", Keyword::Reserved("implements".to_owned()));
        keywords.insert("import", Keyword::Reserved("import".to_owned()));
        keywords.insert("in", Keyword::Reserved("in".to_owned()));
        keywords.insert("init", Keyword::Reserved("init".to_owned()));
        keywords.insert("int", Keyword::Reserved("int".to_owned()));
        keywords.insert("interface", Keyword::Reserved("interface".to_owned()));
        keywords.insert("internal", Keyword::Reserved("internal".to_owned()));
        keywords.insert("intersect", Keyword::Reserved("intersect".to_owned()));
        keywords.insert("intersection", Keyword::Reserved("intersection".to_owned()));
        keywords.insert("is", Keyword::Reserved("is".to_owned()));
        keywords.insert("it", Keyword::Reserved("it".to_owned()));
        keywords.insert("lambda", Keyword::Reserved("lambda".to_owned()));
        keywords.insert("lateinit", Keyword::Reserved("lateinit".to_owned()));
        keywords.insert("lazy", Keyword::Reserved("lazy".to_owned()));
        keywords.insert("local", Keyword::Reserved("local".to_owned()));
        keywords.insert("loop", Keyword::Reserved("loop".to_owned()));
        keywords.insert("macro", Keyword::Reserved("macro".to_owned()));
        keywords.insert("mango", Keyword::Reserved("mango".to_owned()));
        keywords.insert("match", Keyword::Reserved("match".to_owned()));
        keywords.insert("module", Keyword::Reserved("module".to_owned()));
        keywords.insert("move", Keyword::Reserved("move".to_owned()));
        keywords.insert("NaN", Keyword::Reserved("NaN".to_owned()));
        keywords.insert("native", Keyword::Reserved("native".to_owned()));
        keywords.insert("new", Keyword::Reserved("new".to_owned()));
        keywords.insert("nill", Keyword::Reserved("nill".to_owned()));
        keywords.insert("none", Keyword::Reserved("none".to_owned()));
        keywords.insert("null", Keyword::Reserved("null".to_owned()));
        keywords.insert("object", Keyword::Reserved("object".to_owned()));
        keywords.insert("open", Keyword::Reserved("open".to_owned()));
        keywords.insert("operator", Keyword::Reserved("operator".to_owned()));
        keywords.insert("or", Keyword::Reserved("or".to_owned()));
        keywords.insert("out", Keyword::Reserved("out".to_owned()));
        keywords.insert("override", Keyword::Reserved("override".to_owned()));
        keywords.insert("package", Keyword::Reserved("package".to_owned()));
        keywords.insert("param", Keyword::Reserved("param".to_owned()));
        keywords.insert("pass", Keyword::Reserved("pass".to_owned()));
        keywords.insert("private", Keyword::Reserved("private".to_owned()));
        keywords.insert("public", Keyword::Reserved("public".to_owned()));
        keywords.insert("pure", Keyword::Reserved("pure".to_owned()));
        keywords.insert("raise", Keyword::Reserved("raise".to_owned()));
        keywords.insert("real", Keyword::Reserved("real".to_owned()));
        keywords.insert("rec", Keyword::Reserved("rec".to_owned()));
        keywords.insert("reified", Keyword::Reserved("reified".to_owned()));
        keywords.insert("sealed", Keyword::Reserved("sealed".to_owned()));
        keywords.insert("select", Keyword::Reserved("select".to_owned()));
        keywords.insert("self", Keyword::Reserved("self".to_owned()));
        keywords.insert("set", Keyword::Reserved("set".to_owned()));
        keywords.insert("sizeof", Keyword::Reserved("sizeof".to_owned()));
        keywords.insert("static", Keyword::Reserved("static".to_owned()));
        keywords.insert("struct", Keyword::Reserved("struct".to_owned()));
        keywords.insert("super", Keyword::Reserved("super".to_owned()));
        keywords.insert("switch", Keyword::Reserved("switch".to_owned()));
        keywords.insert("sync", Keyword::Reserved("sync".to_owned()));
        keywords.insert("synchronized", Keyword::Reserved("synchronized".to_owned()));
        keywords.insert("tailrec", Keyword::Reserved("tailrec".to_owned()));
        keywords.insert("this", Keyword::Reserved("this".to_owned()));
        keywords.insert("throw", Keyword::Reserved("throw".to_owned()));
        keywords.insert("throws", Keyword::Reserved("throws".to_owned()));
        keywords.insert("to", Keyword::Reserved("to".to_owned()));
        keywords.insert("trait", Keyword::Reserved("trait".to_owned()));
        keywords.insert("transient", Keyword::Reserved("transient".to_owned()));
        keywords.insert("true", Keyword::Reserved("true".to_owned()));
        keywords.insert("try", Keyword::Reserved("try".to_owned()));
        keywords.insert("type", Keyword::Reserved("type".to_owned()));
        keywords.insert("unsafe", Keyword::Reserved("unsafe".to_owned()));
        keywords.insert("unite", Keyword::Reserved("unite".to_owned()));
        keywords.insert("union", Keyword::Reserved("union".to_owned()));
        keywords.insert("until", Keyword::Reserved("until".to_owned()));
        keywords.insert("use", Keyword::Reserved("use".to_owned()));
        keywords.insert("val", Keyword::Reserved("val".to_owned()));
        keywords.insert("var", Keyword::Reserved("var".to_owned()));
        keywords.insert("vararg", Keyword::Reserved("vararg".to_owned()));
        keywords.insert("virtual", Keyword::Reserved("virtual".to_owned()));
        keywords.insert("volatile", Keyword::Reserved("volatile".to_owned()));
        keywords.insert("when", Keyword::Reserved("when".to_owned()));
        keywords.insert("where", Keyword::Reserved("where".to_owned()));
        keywords.insert("with", Keyword::Reserved("with".to_owned()));
        keywords.insert("xor", Keyword::Reserved("xor".to_owned()));
        keywords.insert("yield", Keyword::Reserved("yield".to_owned()));
        keywords
    };
}

impl Keyword {
    /// Convert to a keyword, if it is one. For error message, use FromStr.
    fn from_word(txt: &str) -> Option<Self> {
        KEYWORDS.get(txt).cloned()
    }

    pub fn to_str(&self) -> Cow<str> {
        match self {
            Keyword::Let => Cow::from("let"),
            Keyword::Mut => Cow::from("mut"),
            Keyword::If => Cow::from("if"),
            Keyword::For => Cow::from("for"),
            Keyword::While => Cow::from("while"),
            Keyword::Function => Cow::from("function"),
            Keyword::Return => Cow::from("return"),
            Keyword::Reserved(name) => Cow::from(name),
        }
    }
}

impl FromStr for Keyword {
    type Err = String;

    fn from_str(txt: &str) -> Result<Self, String> {
        match Keyword::from_word(txt) {
            Some(word) => Ok(word),
            None => Err(format!("Unknown keywords: '{}'", txt)),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        use self::Keyword::*;
        let temp_for_lifetime: String;
        f.write_str(match self {
            Let => "let",
            Mut => "mut",
            If => "if",
            For => "for",
            While => "while",
            Function => "fun",
            Return => "return",
            Reserved(name) => {
                temp_for_lifetime = format!("?{}?", name);
                &temp_for_lifetime
            }
        })
    }
}
