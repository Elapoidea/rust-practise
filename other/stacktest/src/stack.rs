pub struct Stack<T> {
    pub elements: Vec<T>,
    capacity: Option<u32>,
}

impl<T> Stack<T> {
    pub fn new(capacity: Option<u32>) -> Stack<T> {
        Stack { elements: vec![], capacity }
    }

    pub fn push(&mut self, datum: T) {
        self.elements.push(datum);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn len(&self) -> u32 {
        self.elements.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        return match self.elements.len() {
            0 => true,
            _ => false
        }
    }

    pub fn is_full(&self) -> bool {
        match self.capacity {
            Some(n) => self.len() == n,
            None => false
        }
    }

    pub fn peek(self) -> Option<T> {
        self.elements[self.len() as usize]
    }
}
