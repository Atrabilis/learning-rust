pub struct Stack {
    items: Vec<i32>,
}

impl Stack{
    pub fn new() -> Stack{
        Self{
            items: Vec::new()
        }
    }
    pub fn push(&mut self, item: i32){
        self.items.push(item);
    }
    pub fn pop(&mut self) -> Option<i32>{
        self.items.pop()
    }
    pub fn peek(&self) -> Option<&i32>{
        if self.is_empty(){
            return None;
        }
        self.items.last()
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
    use super::Stack;

    #[test]
    fn creates_empty_stack() {
        let stack = Stack::new();

        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn pushes_value() {
        let mut stack = Stack::new();

        stack.push(10);

        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn pops_last_inserted_value() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn pop_removes_value() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);
        stack.pop();

        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&10));
    }

    #[test]
    fn returns_none_when_popping_empty_stack() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peeks_last_value_without_removing_it() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);

        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn returns_none_when_peeking_empty_stack() {
        let stack = Stack::new();

        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn can_push_after_popping() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.pop();
        stack.push(20);

        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn handles_i32_limits() {
        let mut stack = Stack::new();

        stack.push(i32::MIN);
        stack.push(i32::MAX);

        assert_eq!(stack.pop(), Some(i32::MAX));
        assert_eq!(stack.pop(), Some(i32::MIN));
    }
}
