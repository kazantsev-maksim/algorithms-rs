use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    items: LinkedList<T>,
}

impl<T> Queue<T> {

    pub fn new() -> Self {
        Self {
            items: LinkedList::new()
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.items.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.items.back()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn drain(&mut self) {
        self.items.clear()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue() {
        let mut queue: Queue<usize> = Queue::default();

        assert!(queue.is_empty());
        queue.enqueue(8);
        queue.enqueue(16);
        assert!(!queue.is_empty());
        assert_eq!(queue.size(), 2);

        assert_eq!(queue.peek_front(), Some(&8));
        assert_eq!(queue.peek_back(), Some(&16));

        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.peek_front(), Some(&16));
        assert_eq!(queue.peek_back(), Some(&16));

        queue.drain();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.dequeue(), None);
    }
}