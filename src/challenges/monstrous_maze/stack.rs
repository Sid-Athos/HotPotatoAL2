//! Structure Pile

pub struct Stack<T> {
    pub elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { elements: vec![] }
    }

    pub fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

}