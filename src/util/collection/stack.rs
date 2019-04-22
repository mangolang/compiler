use std::collections::VecDeque;

/// A one-ended stack. See also [Queue].
/// This is just a wrapper around deque so nobody pushes or pops the wrong end.
pub struct Stack<T> {
    items: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: VecDeque::with_capacity(16),
        }
    }

    pub fn push(&mut self, value: T) {
        self.items.push_back(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_back()
    }

    pub fn borrow_mut(&mut self) -> Option<&mut T> {
        self.items.back_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(2, stack.pop().unwrap());
        assert_eq!(1, stack.pop().unwrap());
        assert!(stack.pop().is_none());
    }
}
