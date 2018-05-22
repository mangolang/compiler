use std::collections::VecDeque;

/// A one-ended queue.
/// This is just a wrapper around deque so nobody pushes or pops the wrong end.
pub struct Queue<T> {
    deque: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            deque: VecDeque::with_capacity(16),
        }
    }

    pub fn push(&mut self, value: T) {
        self.deque.push_back(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.deque.pop_front()
    }
}
