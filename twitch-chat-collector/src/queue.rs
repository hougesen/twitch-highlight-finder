/// Temporary queue data structure implementation.
/// Probably gonna switch to a VecDeque instead.

pub struct Queue<T>(Vec<T>);

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue(vec![])
    }

    pub fn from(vec: Vec<T>) -> Self {
        Queue(vec)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn enqueue(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.0.is_empty() {
            return Some(self.0.remove(self.0.len() - 1));
        }

        None
    }

    /// TODO: figure out a way to move data that isn't O(n)
    pub fn dequeue_all(&mut self) -> Vec<T> {
        let mut items: Vec<T> = Vec::with_capacity(self.size());

        while !self.0.is_empty() {
            items.push(self.0.remove(self.0.len() - 1));
        }

        items
    }
}
