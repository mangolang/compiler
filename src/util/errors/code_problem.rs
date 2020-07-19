use ::derive_new::new;
use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::fmt::{Display, Formatter};

use crate::common::error::ErrMsg;

#[derive(Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Debug,
}

#[derive(new, Debug, Eq, PartialEq)]
pub struct Context {
    code: String, // todo: change this type when there is a specific one
}

#[derive(new, Debug, Eq, PartialEq)]
pub struct CodeProblem {
    severity: Severity,
    //TODO @mark: move this to ErrMsg
    description: Msg,
    context: Context,
    #[new(value = "vec![]")]
    hints: Vec<Msg>,
}

impl CodeProblem {
    pub fn error(description: ErrMsg, context: Context) -> CodeProblem {
        Self::new(Severity::Error, description, context)
    }

    pub fn warning(description: ErrMsg, context: Context) -> CodeProblem {
        Self::new(Severity::Warning, description, context)
    }

    pub fn debug(description: ErrMsg, context: Context) -> CodeProblem {
        Self::new(Severity::Debug, description, context)
    }

    // later: add hints
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Debug => "debug",
            }
        )
    }
}

impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Severity) -> Option<Ordering> {
        match self {
            Severity::Error => match other {
                Severity::Error => Some(Ordering::Equal),
                Severity::Warning => Some(Ordering::Greater),
                Severity::Debug => Some(Ordering::Greater),
            },
            Severity::Warning => match other {
                Severity::Error => Some(Ordering::Less),
                Severity::Warning => Some(Ordering::Equal),
                Severity::Debug => Some(Ordering::Greater),
            },
            Severity::Debug => match other {
                Severity::Error => Some(Ordering::Less),
                Severity::Warning => Some(Ordering::Less),
                Severity::Debug => Some(Ordering::Equal),
            },
        }
    }
}

impl Ord for Severity {
    fn cmp(&self, other: &Severity) -> Ordering {
        self.partial_cmp(other).unwrap()
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
    use crate::util::strtype::StrType;

    use super::CodeProblem;
    use super::Context;
    use super::Severity;

    #[test]
    fn test_severity_ord() {
        assert!(Severity::Error > Severity::Warning);
        assert!(Severity::Error > Severity::Debug);
        assert!(Severity::Warning > Severity::Debug);
        assert!(Severity::Error != Severity::Warning);
        assert!(Severity::Warning != Severity::Debug);
    }

    #[test]
    fn test_new_problem() {
        CodeProblem::error(Msg::new("test problem").unwrap(), Context::new("test context".to_string()));
        CodeProblem::warning(Msg::new("test problem").unwrap(), Context::new("test context".to_string()));
        CodeProblem::debug(Msg::new("test problem").unwrap(), Context::new("test context".to_string()));
    }
}
