struct Container<T>{
    value: T
}

impl<T> Container<T>{
    fn new(value: T) -> Container<T>{
        Container{value}
    }
    fn get(&self) -> &T{
        &self.value
    }
    fn set(&mut self, value: T){
        self.value = value;
    }
    fn is_empty(&self) -> bool{
        false
    }
}