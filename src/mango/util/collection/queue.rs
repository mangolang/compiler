/// A one-ended queue. See also [Stack].
/// This is just a wrapper around vec so nobody pushes or pops the wrong end.
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: Vec::with_capacity(16),
        }
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Moves all the elements from a vector into the queue.
    pub fn append(&mut self, mut other: Vec<T>) {
        self.items.append(&mut other);
    }
}
