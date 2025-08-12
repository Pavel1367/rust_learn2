struct Stack<T>(Vec<T>);

impl<T> Stack<T>{
    fn new() -> Stack<T>{
        Stack(Vec::new())
    }
    fn push(&mut self, item: T){
        self.0.push(item);
    }
    fn pop(&mut self) -> Option<T>{
        self.0.pop()
    }
    fn len(&self) -> usize{
        self.0.len()
    }

}