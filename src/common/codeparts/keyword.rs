use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as fResult;
use ::std::str::FromStr;

use ::lazy_static::lazy_static;

use crate::common::error::{ErrMsg, MsgResult};

/// The different operator codeparts that are recognized.
// TODO: reserve a lot of keywords; easier to remove than add (compatibility)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Keyword {
    Let,
    Mut,
    If,
    For,
    While,
    In,
    Function,
    Return,
    Assert,
    Entrypoint,
    Reserved(String),
}

lazy_static! {
    // Note: keywords must follow the same rules as identifiers, or the lexer will
    // not recognize them. For example, no multi-word keywords.
    pub static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut keywords = HashMap::with_capacity(150);
        assert!(keywords.insert("let", Keyword::Let).is_none());
        assert!(keywords.insert("mut", Keyword::Mut).is_none());
        assert!(keywords.insert("if", Keyword::If).is_none());
        assert!(keywords.insert("for", Keyword::For).is_none());
        assert!(keywords.insert("while", Keyword::While).is_none());
        assert!(keywords.insert("in", Keyword::In).is_none());
        assert!(keywords.insert("fun", Keyword::Function).is_none());
        assert!(keywords.insert("return", Keyword::Return).is_none());
        assert!(keywords.insert("assert", Keyword::Assert).is_none());
        assert!(keywords.insert("abstract", Keyword::Reserved("abstract".to_owned())).is_none());
        assert!(keywords.insert("alias", Keyword::Reserved("alias".to_owned())).is_none());
        assert!(keywords.insert("all", Keyword::Reserved("all".to_owned())).is_none());
        assert!(keywords.insert("and", Keyword::Reserved("and".to_owned())).is_none());
        assert!(keywords.insert("annotation", Keyword::Reserved("annotation".to_owned())).is_none());
        assert!(keywords.insert("any", Keyword::Reserved("any".to_owned())).is_none());
        assert!(keywords.insert("as", Keyword::Reserved("as".to_owned())).is_none());
        assert!(keywords.insert("async", Keyword::Reserved("async".to_owned())).is_none());
        assert!(keywords.insert("auto", Keyword::Reserved("auto".to_owned())).is_none());
        assert!(keywords.insert("await", Keyword::Reserved("await".to_owned())).is_none());
        assert!(keywords.insert("become", Keyword::Reserved("become".to_owned())).is_none());
        assert!(keywords.insert("bool", Keyword::Reserved("bool".to_owned())).is_none());
        assert!(keywords.insert("box", Keyword::Reserved("box".to_owned())).is_none());
        assert!(keywords.insert("break", Keyword::Reserved("break".to_owned())).is_none());
        assert!(keywords.insert("by", Keyword::Reserved("by".to_owned())).is_none());
        assert!(keywords.insert("byte", Keyword::Reserved("byte".to_owned())).is_none());
        assert!(keywords.insert("catch", Keyword::Reserved("catch".to_owned())).is_none());
        assert!(keywords.insert("class", Keyword::Reserved("class".to_owned())).is_none());
        assert!(keywords.insert("closed", Keyword::Reserved("closed".to_owned())).is_none());
        assert!(keywords.insert("companion", Keyword::Reserved("companion".to_owned())).is_none());
        assert!(keywords.insert("const", Keyword::Reserved("const".to_owned())).is_none());
        assert!(keywords.insert("constructor", Keyword::Reserved("constructor".to_owned())).is_none());
        assert!(keywords.insert("continue", Keyword::Reserved("continue".to_owned())).is_none());
        assert!(keywords.insert("data", Keyword::Reserved("data".to_owned())).is_none());
        assert!(keywords.insert("debug", Keyword::Reserved("debug".to_owned())).is_none());
        assert!(keywords.insert("def", Keyword::Reserved("def".to_owned())).is_none());
        assert!(keywords.insert("default", Keyword::Reserved("default".to_owned())).is_none());
        assert!(keywords.insert("defer", Keyword::Reserved("defer".to_owned())).is_none());
        assert!(keywords.insert("del", Keyword::Reserved("del".to_owned())).is_none());
        assert!(keywords.insert("delegate", Keyword::Reserved("delegate".to_owned())).is_none());
        assert!(keywords.insert("delegates", Keyword::Reserved("delegates".to_owned())).is_none());
        assert!(keywords.insert("delete", Keyword::Reserved("delete".to_owned())).is_none());
        assert!(keywords.insert("derive", Keyword::Reserved("derive".to_owned())).is_none());
        assert!(keywords.insert("deriving", Keyword::Reserved("deriving".to_owned())).is_none());
        assert!(keywords.insert("do", Keyword::Reserved("do".to_owned())).is_none());
        assert!(keywords.insert("double", Keyword::Reserved("double".to_owned())).is_none());
        assert!(keywords.insert("dynamic", Keyword::Reserved("dynamic".to_owned())).is_none());
        assert!(keywords.insert("elementwise", Keyword::Reserved("elementwise".to_owned())).is_none());
        assert!(keywords.insert("elif", Keyword::Reserved("elif".to_owned())).is_none());
        assert!(keywords.insert("end", Keyword::Reserved("end".to_owned())).is_none());
        assert!(keywords.insert("enum", Keyword::Reserved("enum".to_owned())).is_none());
        assert!(keywords.insert("eval", Keyword::Reserved("eval".to_owned())).is_none());
        assert!(keywords.insert("except", Keyword::Reserved("except".to_owned())).is_none());
        assert!(keywords.insert("extends", Keyword::Reserved("extends".to_owned())).is_none());
        assert!(keywords.insert("extern", Keyword::Reserved("extern".to_owned())).is_none());
        assert!(keywords.insert("false", Keyword::Reserved("false".to_owned())).is_none());
        assert!(keywords.insert("family", Keyword::Reserved("family".to_owned())).is_none());
        assert!(keywords.insert("field", Keyword::Reserved("field".to_owned())).is_none());
        assert!(keywords.insert("final", Keyword::Reserved("final".to_owned())).is_none());
        assert!(keywords.insert("finally", Keyword::Reserved("finally".to_owned())).is_none());
        assert!(keywords.insert("float", Keyword::Reserved("float".to_owned())).is_none());
        assert!(keywords.insert("fn", Keyword::Reserved("fn".to_owned())).is_none());
        assert!(keywords.insert("get", Keyword::Reserved("get".to_owned())).is_none());
        assert!(keywords.insert("global", Keyword::Reserved("global".to_owned())).is_none());
        assert!(keywords.insert("goto", Keyword::Reserved("goto".to_owned())).is_none());
        assert!(keywords.insert("impl", Keyword::Reserved("impl".to_owned())).is_none());
        assert!(keywords.insert("implements", Keyword::Reserved("implements".to_owned())).is_none());
        assert!(keywords.insert("import", Keyword::Reserved("import".to_owned())).is_none());
        assert!(keywords.insert("init", Keyword::Reserved("init".to_owned())).is_none());
        assert!(keywords.insert("int", Keyword::Reserved("int".to_owned())).is_none());
        assert!(keywords.insert("interface", Keyword::Reserved("interface".to_owned())).is_none());
        assert!(keywords.insert("internal", Keyword::Reserved("internal".to_owned())).is_none());
        assert!(keywords.insert("intersect", Keyword::Reserved("intersect".to_owned())).is_none());
        assert!(keywords.insert("intersection", Keyword::Reserved("intersection".to_owned())).is_none());
        assert!(keywords.insert("is", Keyword::Reserved("is".to_owned())).is_none());
        assert!(keywords.insert("it", Keyword::Reserved("it".to_owned())).is_none());
        assert!(keywords.insert("lambda", Keyword::Reserved("lambda".to_owned())).is_none());
        assert!(keywords.insert("lateinit", Keyword::Reserved("lateinit".to_owned())).is_none());
        assert!(keywords.insert("lazy", Keyword::Reserved("lazy".to_owned())).is_none());
        assert!(keywords.insert("local", Keyword::Reserved("local".to_owned())).is_none());
        assert!(keywords.insert("loop", Keyword::Reserved("loop".to_owned())).is_none());
        assert!(keywords.insert("macro", Keyword::Reserved("macro".to_owned())).is_none());
        assert!(keywords.insert("mango", Keyword::Reserved("mango".to_owned())).is_none());
        assert!(keywords.insert("match", Keyword::Reserved("match".to_owned())).is_none());
        assert!(keywords.insert("module", Keyword::Reserved("module".to_owned())).is_none());
        assert!(keywords.insert("move", Keyword::Reserved("move".to_owned())).is_none());
        assert!(keywords.insert("NaN", Keyword::Reserved("NaN".to_owned())).is_none());
        assert!(keywords.insert("native", Keyword::Reserved("native".to_owned())).is_none());
        assert!(keywords.insert("new", Keyword::Reserved("new".to_owned())).is_none());
        assert!(keywords.insert("nill", Keyword::Reserved("nill".to_owned())).is_none());
        assert!(keywords.insert("none", Keyword::Reserved("none".to_owned())).is_none());
        assert!(keywords.insert("null", Keyword::Reserved("null".to_owned())).is_none());
        assert!(keywords.insert("object", Keyword::Reserved("object".to_owned())).is_none());
        assert!(keywords.insert("open", Keyword::Reserved("open".to_owned())).is_none());
        assert!(keywords.insert("operator", Keyword::Reserved("operator".to_owned())).is_none());
        assert!(keywords.insert("or", Keyword::Reserved("or".to_owned())).is_none());
        assert!(keywords.insert("out", Keyword::Reserved("out".to_owned())).is_none());
        assert!(keywords.insert("override", Keyword::Reserved("override".to_owned())).is_none());
        assert!(keywords.insert("package", Keyword::Reserved("package".to_owned())).is_none());
        assert!(keywords.insert("param", Keyword::Reserved("param".to_owned())).is_none());
        assert!(keywords.insert("pass", Keyword::Reserved("pass".to_owned())).is_none());
        assert!(keywords.insert("private", Keyword::Reserved("private".to_owned())).is_none());
        assert!(keywords.insert("public", Keyword::Reserved("public".to_owned())).is_none());
        assert!(keywords.insert("pure", Keyword::Reserved("pure".to_owned())).is_none());
        assert!(keywords.insert("raise", Keyword::Reserved("raise".to_owned())).is_none());
        assert!(keywords.insert("real", Keyword::Reserved("real".to_owned())).is_none());
        assert!(keywords.insert("rec", Keyword::Reserved("rec".to_owned())).is_none());
        assert!(keywords.insert("reified", Keyword::Reserved("reified".to_owned())).is_none());
        assert!(keywords.insert("sealed", Keyword::Reserved("sealed".to_owned())).is_none());
        assert!(keywords.insert("select", Keyword::Reserved("select".to_owned())).is_none());
        assert!(keywords.insert("self", Keyword::Reserved("self".to_owned())).is_none());
        assert!(keywords.insert("set", Keyword::Reserved("set".to_owned())).is_none());
        assert!(keywords.insert("sizeof", Keyword::Reserved("sizeof".to_owned())).is_none());
        assert!(keywords.insert("static", Keyword::Reserved("static".to_owned())).is_none());
        assert!(keywords.insert("struct", Keyword::Reserved("struct".to_owned())).is_none());
        assert!(keywords.insert("super", Keyword::Reserved("super".to_owned())).is_none());
        assert!(keywords.insert("switch", Keyword::Reserved("switch".to_owned())).is_none());
        assert!(keywords.insert("sync", Keyword::Reserved("sync".to_owned())).is_none());
        assert!(keywords.insert("synchronized", Keyword::Reserved("synchronized".to_owned())).is_none());
        assert!(keywords.insert("tailrec", Keyword::Reserved("tailrec".to_owned())).is_none());
        assert!(keywords.insert("this", Keyword::Reserved("this".to_owned())).is_none());
        assert!(keywords.insert("throw", Keyword::Reserved("throw".to_owned())).is_none());
        assert!(keywords.insert("throws", Keyword::Reserved("throws".to_owned())).is_none());
        assert!(keywords.insert("to", Keyword::Reserved("to".to_owned())).is_none());
        assert!(keywords.insert("trait", Keyword::Reserved("trait".to_owned())).is_none());
        assert!(keywords.insert("transient", Keyword::Reserved("transient".to_owned())).is_none());
        assert!(keywords.insert("true", Keyword::Reserved("true".to_owned())).is_none());
        assert!(keywords.insert("try", Keyword::Reserved("try".to_owned())).is_none());
        assert!(keywords.insert("type", Keyword::Reserved("type".to_owned())).is_none());
        assert!(keywords.insert("unsafe", Keyword::Reserved("unsafe".to_owned())).is_none());
        assert!(keywords.insert("unite", Keyword::Reserved("unite".to_owned())).is_none());
        assert!(keywords.insert("union", Keyword::Reserved("union".to_owned())).is_none());
        assert!(keywords.insert("until", Keyword::Reserved("until".to_owned())).is_none());
        assert!(keywords.insert("use", Keyword::Reserved("use".to_owned())).is_none());
        assert!(keywords.insert("val", Keyword::Reserved("val".to_owned())).is_none());
        assert!(keywords.insert("var", Keyword::Reserved("var".to_owned())).is_none());
        assert!(keywords.insert("vararg", Keyword::Reserved("vararg".to_owned())).is_none());
        assert!(keywords.insert("virtual", Keyword::Reserved("virtual".to_owned())).is_none());
        assert!(keywords.insert("volatile", Keyword::Reserved("volatile".to_owned())).is_none());
        assert!(keywords.insert("when", Keyword::Reserved("when".to_owned())).is_none());
        assert!(keywords.insert("where", Keyword::Reserved("where".to_owned())).is_none());
        assert!(keywords.insert("with", Keyword::Reserved("with".to_owned())).is_none());
        assert!(keywords.insert("xor", Keyword::Reserved("xor".to_owned())).is_none());
        assert!(keywords.insert("yield", Keyword::Reserved("yield".to_owned())).is_none());
        keywords
    };
}

impl Keyword {
    /// Convert to a keyword, if it is one. For error message, use FromStr.
    pub fn from_word(txt: &str) -> Option<Self> {
        KEYWORDS.get(txt).cloned()
    }

    pub fn to_str(&self) -> Cow<str> {
        match self {
            Keyword::Let => Cow::from("let"),
            Keyword::Mut => Cow::from("mut"),
            Keyword::If => Cow::from("if"),
            Keyword::For => Cow::from("for"),
            Keyword::While => Cow::from("while"),
            Keyword::In => Cow::from("in"),
            Keyword::Function => Cow::from("function"),
            Keyword::Return => Cow::from("return"),
            Keyword::Assert => Cow::from("assert"),
            Keyword::Entrypoint => Cow::from("main"),
            Keyword::Reserved(name) => Cow::from(name),
        }
    }
}

impl FromStr for Keyword {
    type Err = ErrMsg;

    fn from_str(txt: &str) -> MsgResult<Self> {
        match Keyword::from_word(txt) {
            Some(word) => Ok(word),
            None => Err(ErrMsg::new(format!("Unknown keywords: '{}'", txt))),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        f.write_str(self.to_str().as_ref())
    }
}
