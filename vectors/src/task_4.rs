fn find_all_element_indices<T>(vec: &Vec<T>, target: T)-> Vec<usize> where T: PartialEq  {
    let mut result: Vec<usize> = Vec::new();
    for (index,elem) in vec.iter().enumerate() {
        if *elem == target{
            result.push(index)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_to_find_target_indices_nums(){
        let vec = vec![1,2,2,2,5,6,4,2,3,4,6,7,8,8,9];
        let result = vec![1,2,3,7];
        assert_eq!(find_all_element_indices(&vec, vec[1]), result);
    }
    #[test]
    fn has_to_find_target_indices_strings_slices(){
        let vec = vec!["abrakadabra", "smth", "lol", "lol"];
        let result = vec![2,3];
        assert_eq!(find_all_element_indices(&vec, "lol"), result);
    }
    #[test]
    fn has_to_find_target_indices_strings(){
        let vec = vec![String::from("abrakadabra"),String::from( "smth"), String::from("lol"), String::from("lol")];
        let result = vec![2,3];
        assert_eq!(find_all_element_indices(&vec, "lol".to_string()), result);
    }
    #[derive(PartialEq)]
    struct Person{
        name: String,
        age: i32,
    }
    #[test]
    fn has_to_find_target_indices_structures(){

        let vec = vec![Person{name:"Pavel".to_string(), age: 24},Person{name:"Nikita".to_string(), age: 20}];
        let result = vec![0];
        assert_eq!(find_all_element_indices(&vec, Person{name:"Pavel".to_string(), age: 24}), result);
    }
    #[test]
    fn should_not_find_target_indices_structures(){
        let vec = vec![Person{name:"Pavel".to_string(), age: 24},Person{name:"Nikita".to_string(), age: 20}];
        let result = vec![];
        assert_eq!(find_all_element_indices(&vec, Person{name:"abrakadabra".to_string(), age: 24}), result);
    }
}
