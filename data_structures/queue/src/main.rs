use std::collections::HashMap;

struct Queue {
    front: i32,
    back: i32,
    storage: HashMap<i32, i32>
}

impl Queue {
    fn new() -> Queue {
        Queue {
            front: 0,
            back: 0,
            storage: HashMap::new(),
        }
    }
    fn enqueue(&mut self, val: i32) -> () {
        self.storage.insert(self.back, val);
        self.back = self.back + 1;
    }

    fn dequeue(&mut self) -> Option<i32> {
        let (valid, val) = match self.storage.remove(&self.front) {
            Some(num) => (true, num),
            None => (false, 0),
        };

        if valid {
            self.front = self.front + 1;
            Some(val)
        } else {
            None
        }
    }

    fn size_of(&self) -> i32 {
        self.back - self.front
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Queue;

    #[test]
    fn test() {
        let mut queue = Queue::new();
        queue.enqueue(5);
        queue.enqueue(6);
        queue.dequeue();
        queue.enqueue(3);
        assert_eq!(queue.size_of(), 2);
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.size_of(), 0);
        queue.enqueue(1);
        assert_eq!(queue.size_of(), 1);
    }
}