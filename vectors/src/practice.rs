fn _5(){
    let numbers = vec![10, 20, 30];
    let result = numbers.get(5);
    match result {
        Some(value) => println!("Got value : {}", value),
        None => println!("None"),
    }
}

fn _6(){
    let mut seasons: Vec<i32> = Vec::with_capacity(4);// ваш код для емкости 4
}

fn _7(){
    let mut pizza_toppings = vec!["Pepperoni".to_string()];
    let topping = &mut pizza_toppings[0];// получите изменяемую ссылку
    // добавьте " Extra" к строке
    topping.push_str(" Extra");

}

fn _8(){
    let mut numbers = vec![1, 3, 4];
    // вставьте число 2 между 1 и 3
    numbers.insert(1,2)
}
fn safe_pop<T>(vec: &mut Vec<T>) -> Option<T>// ???
{
    vec.pop()
    // ваша реализация
}