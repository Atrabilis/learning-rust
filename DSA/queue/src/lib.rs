use std::collections::VecDeque;

pub struct Queue{
    items: VecDeque<i32>,
}

impl Queue{
    pub fn new() -> Self{
        Self{
           items : VecDeque::new() 
        }
    }
    pub fn enqueue(&mut self, value: i32){
        self.items.push_back(value);
    }
    pub fn dequeue(&mut self) -> Option<i32>{
        self.items.pop_front()
    }
    pub fn front(&self) -> Option<&i32>{
        self.items.front()
    }
    pub fn len(&self) -> usize{
        self.items.len()
    }
    pub fn is_empty(&self) -> bool{
        self.items.is_empty()   
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn creates_empty_queue() {
        let queue = Queue::new();

        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn enqueues_value() {
        let mut queue = Queue::new();

        queue.enqueue(10);

        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
    }

    #[test]
    fn dequeues_first_inserted_value() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
    }

    #[test]
    fn dequeue_removes_value() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.dequeue();

        assert_eq!(queue.len(), 1);
        assert_eq!(queue.front(), Some(&20));
    }

    #[test]
    fn returns_none_when_dequeuing_empty_queue() {
        let mut queue = Queue::new();

        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn returns_front_without_removing_it() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);

        assert_eq!(queue.front(), Some(&10));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn returns_none_when_reading_front_of_empty_queue() {
        let queue = Queue::new();

        assert_eq!(queue.front(), None);
    }

    #[test]
    fn can_enqueue_after_dequeuing() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.dequeue();
        queue.enqueue(20);

        assert_eq!(queue.front(), Some(&20));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn preserves_fifo_order_after_multiple_operations() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        assert_eq!(queue.dequeue(), Some(10));

        queue.enqueue(30);

        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
    }

    #[test]
    fn handles_i32_limits() {
        let mut queue = Queue::new();

        queue.enqueue(i32::MIN);
        queue.enqueue(i32::MAX);

        assert_eq!(queue.dequeue(), Some(i32::MIN));
        assert_eq!(queue.dequeue(), Some(i32::MAX));
    }
}