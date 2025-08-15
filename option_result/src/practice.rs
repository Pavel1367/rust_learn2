fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b > 0.0{
        Ok(a/b)
    }else{
        Err("division by zero")
    }
}


fn parse_number(s: &str) -> Option<i32> {
    // Ваш код здесь - используйте parse() и обработайте ошибку
    let number = s.parse::<i32>();
    if let Ok(n) = number {
        Some(n)
    }else{
        None
    }
}

fn process_user_input(input: &str) -> Option<String> {
    // Если строка не пустая, верните её в верхнем регистре
    // Ваш код здесь
    if input.is_empty(){
        None
    }else{
        Some(String::from(input.to_uppercase()))
    }
}