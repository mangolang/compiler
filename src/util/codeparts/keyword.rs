use crate::util::strtype::StrType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as fResult;
use std::str::FromStr;
use crate::common::error::{MangoResult, MangoErr};
use std::borrow::Cow;

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

impl Keyword {
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

    fn from_str(symbol_txt: &str) -> Result<Self, String> {
        use self::Keyword::*;
        match symbol_txt {
            "let" => Ok(Let),
            "mut" => Ok(Mut),
            "if" => Ok(If),
            "for" => Ok(For),
            "while" => Ok(While),
            "fun" => Ok(Function),
            "return" => Ok(Return),

            "abstract" => Ok(Reserved("abstract".to_owned())),
            "alias" => Ok(Reserved("alias".to_owned())),
            "all" => Ok(Reserved("all".to_owned())),
            "and" => Ok(Reserved("and".to_owned())),
            "annotation" => Ok(Reserved("annotation".to_owned())),
            "any" => Ok(Reserved("any".to_owned())),
            "as" => Ok(Reserved("as".to_owned())),
            "assert" => Ok(Reserved("assert".to_owned())),
            "async" => Ok(Reserved("async".to_owned())),
            "auto" => Ok(Reserved("auto".to_owned())),
            "await" => Ok(Reserved("await".to_owned())),
            "become" => Ok(Reserved("become".to_owned())),
            "bool" => Ok(Reserved("bool".to_owned())),
            "box" => Ok(Reserved("box".to_owned())),
            "break" => Ok(Reserved("break".to_owned())),
            "by" => Ok(Reserved("by".to_owned())),
            "byte" => Ok(Reserved("byte".to_owned())),
            "catch" => Ok(Reserved("catch".to_owned())),
            "class" => Ok(Reserved("class".to_owned())),
            "closed" => Ok(Reserved("closed".to_owned())),
            "companion" => Ok(Reserved("companion".to_owned())),
            "const" => Ok(Reserved("const".to_owned())),
            "constructor" => Ok(Reserved("constructor".to_owned())),
            "continue" => Ok(Reserved("continue".to_owned())),
            "data" => Ok(Reserved("data".to_owned())),
            "debug" => Ok(Reserved("debug".to_owned())),
            "def" => Ok(Reserved("def".to_owned())),
            "default" => Ok(Reserved("default".to_owned())),
            "defer" => Ok(Reserved("defer".to_owned())),
            "del" => Ok(Reserved("del".to_owned())),
            "delegate" => Ok(Reserved("delegate".to_owned())),
            "delegates" => Ok(Reserved("delegates".to_owned())),
            "delete" => Ok(Reserved("delete".to_owned())),
            "derive" => Ok(Reserved("derive".to_owned())),
            "deriving" => Ok(Reserved("deriving".to_owned())),
            "do" => Ok(Reserved("do".to_owned())),
            "double" => Ok(Reserved("double".to_owned())),
            "dynamic" => Ok(Reserved("dynamic".to_owned())),
            "elementwise" => Ok(Reserved("elementwise".to_owned())),
            "elif" => Ok(Reserved("elif".to_owned())),
            "end" => Ok(Reserved("end".to_owned())),
            "enum" => Ok(Reserved("enum".to_owned())),
            "eval" => Ok(Reserved("eval".to_owned())),
            "except" => Ok(Reserved("except".to_owned())),
            "extends" => Ok(Reserved("extends".to_owned())),
            "extern" => Ok(Reserved("extern".to_owned())),
            "false" => Ok(Reserved("false".to_owned())),
            "family" => Ok(Reserved("family".to_owned())),
            "field" => Ok(Reserved("field".to_owned())),
            "final" => Ok(Reserved("final".to_owned())),
            "finally" => Ok(Reserved("finally".to_owned())),
            "float" => Ok(Reserved("float".to_owned())),
            "fn" => Ok(Reserved("fn".to_owned())),
            "get" => Ok(Reserved("get".to_owned())),
            "global" => Ok(Reserved("global".to_owned())),
            "goto" => Ok(Reserved("goto".to_owned())),
            "impl" => Ok(Reserved("impl".to_owned())),
            "implements" => Ok(Reserved("implements".to_owned())),
            "import" => Ok(Reserved("import".to_owned())),
            "in" => Ok(Reserved("in".to_owned())),
            "init" => Ok(Reserved("init".to_owned())),
            "int" => Ok(Reserved("int".to_owned())),
            "interface" => Ok(Reserved("interface".to_owned())),
            "internal" => Ok(Reserved("internal".to_owned())),
            "intersect" => Ok(Reserved("intersect".to_owned())),
            "intersection" => Ok(Reserved("intersection".to_owned())),
            "is" => Ok(Reserved("is".to_owned())),
            "it" => Ok(Reserved("it".to_owned())),
            "lambda" => Ok(Reserved("lambda".to_owned())),
            "lateinit" => Ok(Reserved("lateinit".to_owned())),
            "lazy" => Ok(Reserved("lazy".to_owned())),
            "local" => Ok(Reserved("local".to_owned())),
            "loop" => Ok(Reserved("loop".to_owned())),
            "macro" => Ok(Reserved("macro".to_owned())),
            "match" => Ok(Reserved("match".to_owned())),
            "module" => Ok(Reserved("module".to_owned())),
            "move" => Ok(Reserved("move".to_owned())),
            "NaN" => Ok(Reserved("NaN".to_owned())),
            "native" => Ok(Reserved("native".to_owned())),
            "new" => Ok(Reserved("new".to_owned())),
            "nill" => Ok(Reserved("nill".to_owned())),
            "none" => Ok(Reserved("none".to_owned())),
            "null" => Ok(Reserved("null".to_owned())),
            "object" => Ok(Reserved("object".to_owned())),
            "open" => Ok(Reserved("open".to_owned())),
            "operator" => Ok(Reserved("operator".to_owned())),
            "or" => Ok(Reserved("or".to_owned())),
            "out" => Ok(Reserved("out".to_owned())),
            "override" => Ok(Reserved("override".to_owned())),
            "package" => Ok(Reserved("package".to_owned())),
            "param" => Ok(Reserved("param".to_owned())),
            "pass" => Ok(Reserved("pass".to_owned())),
            "private" => Ok(Reserved("private".to_owned())),
            "public" => Ok(Reserved("public".to_owned())),
            "pure" => Ok(Reserved("pure".to_owned())),
            "raise" => Ok(Reserved("raise".to_owned())),
            "real" => Ok(Reserved("real".to_owned())),
            "rec" => Ok(Reserved("rec".to_owned())),
            "reified" => Ok(Reserved("reified".to_owned())),
            "sealed" => Ok(Reserved("sealed".to_owned())),
            "select" => Ok(Reserved("select".to_owned())),
            "self" => Ok(Reserved("self".to_owned())),
            "set" => Ok(Reserved("set".to_owned())),
            "sizeof" => Ok(Reserved("sizeof".to_owned())),
            "static" => Ok(Reserved("static".to_owned())),
            "struct" => Ok(Reserved("struct".to_owned())),
            "super" => Ok(Reserved("super".to_owned())),
            "switch" => Ok(Reserved("switch".to_owned())),
            "sync" => Ok(Reserved("sync".to_owned())),
            "synchronized" => Ok(Reserved("synchronized".to_owned())),
            "tailrec" => Ok(Reserved("tailrec".to_owned())),
            "this" => Ok(Reserved("this".to_owned())),
            "throw" => Ok(Reserved("throw".to_owned())),
            "throws" => Ok(Reserved("throws".to_owned())),
            "to" => Ok(Reserved("to".to_owned())),
            "trait" => Ok(Reserved("trait".to_owned())),
            "transient" => Ok(Reserved("transient".to_owned())),
            "true" => Ok(Reserved("true".to_owned())),
            "try" => Ok(Reserved("try".to_owned())),
            "type" => Ok(Reserved("type".to_owned())),
            "unsafe" => Ok(Reserved("unsafe".to_owned())),
            "unite" => Ok(Reserved("unite".to_owned())),
            "union" => Ok(Reserved("union".to_owned())),
            "until" => Ok(Reserved("until".to_owned())),
            "use" => Ok(Reserved("use".to_owned())),
            "val" => Ok(Reserved("val".to_owned())),
            "var" => Ok(Reserved("var".to_owned())),
            "vararg" => Ok(Reserved("vararg".to_owned())),
            "virtual" => Ok(Reserved("virtual".to_owned())),
            "volatile" => Ok(Reserved("volatile".to_owned())),
            "when" => Ok(Reserved("when".to_owned())),
            "where" => Ok(Reserved("where".to_owned())),
            "with" => Ok(Reserved("with".to_owned())),
            "xor" => Ok(Reserved("xor".to_owned())),
            "yield" => Ok(Reserved("yield".to_owned())),

            _ => Err(format!("Unknown keywords: '{}'", symbol_txt).into()),
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
