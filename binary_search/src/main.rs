fn main() {
    let numbers = vec![1,2,3,5,6,7,9,10,12,15,17,63,75,78,89,99,102];
    println!("{:?}", binary_search(&numbers, 102))
}

fn binary_search(sorted_numbers: &Vec<u32>, target: u32)->Option<usize> {
    if sorted_numbers.len() == 0 {
        return None;
    }
    let mut right = sorted_numbers.len()-1;
    let mut left = 0;
    while left <= right{
        let mid = (left+right) / 2;
        match sorted_numbers[mid]{
            num if num == target => {
               return Some(mid)
            },
            num if num < target => left = mid + 1,
            _ => right = mid - 1
        }
    }
    None
}
