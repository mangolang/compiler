use std::collections::VecDeque;

/// A one-ended queue. See also [Stack].
/// This is just a wrapper around vec so nobody pushes or pops the wrong end.
#[derive(Default)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: VecDeque::with_capacity(16),
        }
    }

    pub fn push(&mut self, value: T) {
        self.items.push_back(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Moves all the elements from a vector into the queue.
    pub fn append<I: Iterator<Item=T>>(&mut self, other: I) {
        for item in other {
            self.items.push_back(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue() {
        let mut queue: Queue<i32> = Queue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(1, queue.pop().unwrap());
        assert_eq!(2, queue.pop().unwrap());
        assert!(queue.pop().is_none());
    }
}
