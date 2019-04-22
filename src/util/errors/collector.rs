use crate::util::errors::CodeProblem;
use crate::util::errors::Context;
use crate::util::strtype::Msg;
use derive_new::new;
use std::slice;

#[derive(new, Debug)]
pub struct ProblemCollector {
    // Note that Vec is already heap-allocated, no need for box.
    #[new(value = "vec![]")]
    problems: Vec<CodeProblem>,
}

impl ProblemCollector {
    pub fn error(&mut self, description: Msg, context: Context) -> &mut CodeProblem {
        let problem = CodeProblem::error(description, context);
        self.problems.push(problem);
        self.problems.last_mut().unwrap()
    }

    pub fn warning(&mut self, description: Msg, context: Context) -> &mut CodeProblem {
        let problem = CodeProblem::warning(description, context);
        self.problems.push(problem);
        self.problems.last_mut().unwrap()
    }

    pub fn debug(&mut self, description: Msg, context: Context) -> &mut CodeProblem {
        let problem = CodeProblem::debug(description, context);
        self.problems.push(problem);
        self.problems.last_mut().unwrap()
    }
}

impl<'a> IntoIterator for &'a ProblemCollector {
    type Item = &'a CodeProblem;
    type IntoIter = slice::Iter<'a, CodeProblem>;

    fn into_iter(self) -> Self::IntoIter {
        self.problems.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::ProblemCollector;
    use crate::util::errors::Context;
    use crate::util::strtype::Msg;
    use crate::util::strtype::StrType;

    #[test]
    fn test_iter_collector() {
        let mut collector = ProblemCollector::new();
        collector.error(Msg::copy_new("test problem").unwrap(), Context::new("test context".to_string()));
        let cnt = collector.into_iter().count();
        assert_eq!(1, cnt, "No item in ProblemCollector");
        assert_eq!(cnt, collector.into_iter().count(), "Failed to iterate over ProblemCollector twice")
    }

    #[test]
    fn test_new_problem() {
        let mut collector = ProblemCollector::new();
        collector.error(Msg::copy_new("test problem").unwrap(), Context::new("test context".to_string()));
        collector.warning(Msg::copy_new("test problem").unwrap(), Context::new("test context".to_string()));
        collector.debug(Msg::copy_new("test problem").unwrap(), Context::new("test context".to_string()));
    }
}
