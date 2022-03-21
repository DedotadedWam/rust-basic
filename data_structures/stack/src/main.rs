use std::collections::HashMap;
pub struct Stack {
    storage: HashMap<i32, i32>,
    size: i32,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            storage: HashMap::new(),
            size: 0,
        }
    }

    fn size_of(&self) -> i32 {
        self.size
    }

    fn push(&mut self, value: i32) -> () {
        self.size = self.size + 1;
        self.storage.insert(self.size, value);
    }

    fn pop(&mut self) -> Option<i32> {
        let (valid, value) = match self.storage.remove(&self.size) {
            Some(number) => (true, number),
            None => (false, 0),
        };

        if valid {
            self.size = &self.size - 1;
            Some(value)
        } else {
            None
        }
    }

    fn peek(&self) -> i32 {
        *self.storage.get(&self.size).unwrap()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(5);
    stack.push(6);
    stack.pop();
    let top = stack.peek();
    let big = stack.size_of();
    println!("Top should be 5 and is {}, Size should be 1 and is {}", top, big);
}

#[cfg(test)]
mod tests {
    use crate::Stack;

    #[test]
    fn test() {
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(6);
        stack.pop();
        stack.push(3);
        assert_eq!(stack.size_of(), 2);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.size_of(), 0);
        stack.push(1);
        assert_eq!(stack.peek(), 1);
        assert_eq!(stack.peek(), 1)
    }
}