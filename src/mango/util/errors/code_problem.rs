use std::fmt::{Display, Formatter};
use std::cmp::Ordering;
use std::fmt;
use mango::util::strtype::Msg;

#[derive(Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Debug,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Context {
    code: String, // todo: change this type when there is a specific one
}

impl Context {
    pub fn new(code: String) -> Context {
        return Context { code: code };
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CodeProblem {
    severity: Severity,
    description: Msg,
    context: Context,
    hints: Vec<Msg>,
}

impl CodeProblem {
    pub fn new(severity: Severity, description: Msg, context: Context) -> CodeProblem {
        return CodeProblem {
            severity: severity,
            description: description,
            context: context,
            hints: vec![],
        };
    }

    pub fn error(description: Msg, context: Context) -> CodeProblem {
        return Self::new(Severity::Error, description, context);
    }

    pub fn warning(description: Msg, context: Context) -> CodeProblem {
        return Self::new(Severity::Warning, description, context);
    }

    pub fn debug(description: Msg, context: Context) -> CodeProblem {
        return Self::new(Severity::Debug, description, context);
    }

    // later: add hints
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                &Severity::Error => "error",
                &Severity::Warning => "warning",
                &Severity::Debug => "debug",
            }
        )
    }
}

impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Severity) -> Option<Ordering> {
        return match self {
            &Severity::Error => match other {
                &Severity::Error => Some(Ordering::Equal),
                &Severity::Warning => Some(Ordering::Greater),
                &Severity::Debug => Some(Ordering::Greater),
            },
            &Severity::Warning => match other {
                &Severity::Error => Some(Ordering::Less),
                &Severity::Warning => Some(Ordering::Equal),
                &Severity::Debug => Some(Ordering::Greater),
            },
            &Severity::Debug => match other {
                &Severity::Error => Some(Ordering::Less),
                &Severity::Warning => Some(Ordering::Less),
                &Severity::Debug => Some(Ordering::Equal),
            },
        };
    }
}

impl Ord for Severity {
    fn cmp(&self, other: &Severity) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl Display for CodeProblem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        // todo: hints and context
        write!(f, "{}: {}", self.severity, self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::Severity;
    use super::Context;
    use super::CodeProblem;
    use mango::util::strtype::Msg;
    use mango::util::strtype::StrType;

    #[test]
    fn test_severity_ord() {
        assert!(Severity::Error > Severity::Warning);
        assert!(Severity::Error > Severity::Debug);
        assert!(Severity::Warning > Severity::Debug);
        assert!(Severity::Error == Severity::Error);
        assert!(Severity::Warning == Severity::Warning);
        assert!(Severity::Debug == Severity::Debug);
    }

    #[test]
    fn test_new_problem() {
        CodeProblem::error(
            Msg::copy_new("test problem").unwrap(),
            Context::new("test context".to_string()),
        );
        CodeProblem::warning(
            Msg::copy_new("test problem").unwrap(),
            Context::new("test context".to_string()),
        );
        CodeProblem::debug(
            Msg::copy_new("test problem").unwrap(),
            Context::new("test context".to_string()),
        );
    }
}
