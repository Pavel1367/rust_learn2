#[derive(Debug, Copy, Clone)]
enum MyOption<T>{
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn unwrap(self) -> T {
        if let MyOption::Some(x) = self {
            x
        }else {
            panic!("Crazy shit");
        }
    }
    fn unwrap_or(self, other: T) -> T {
        if let MyOption::Some(x) = self {
            x
        }else{
            other
        }
    }
}