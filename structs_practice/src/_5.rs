struct User {
    name: String,
    email: String,
    age: Option<u32>,
}

impl User {
    fn new(name: String, email: String) -> Self {
        User{
            name,
            email,
            age: None,
        }
    }

    fn set_age(&mut self, age: u32) -> &mut Self {
        self.age = Some(age);
        self
    }
}