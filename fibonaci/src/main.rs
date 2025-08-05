use std::collections::HashMap;
fn main() {
    let mut cache: HashMap<u32, u32> = HashMap::new();
    println!("{}", fibonacci_cache(45, &mut cache))
}

fn fibonacci_cache(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    if n == 0 || n == 1 {
       return  n
    };
    if let Some(value) = cache.get(&n) {
       return  *value
    };
    let result = fibonacci_cache(n-1, cache) + fibonacci_cache(n-2, cache);
    cache.insert(n, result);
    result
}
