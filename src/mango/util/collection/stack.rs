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
