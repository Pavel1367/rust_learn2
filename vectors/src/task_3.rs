fn find_second_largest(mut vec: Vec<i64>) -> Option<i64> {
    if vec.len() < 2 {
      return None
    };
    vec.sort_by(|a, b| b.cmp(a));
    vec.dedup();
    if let Some(second_largest) = vec.get(1) {
        Some(*second_largest)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_second_largest_positive() {
        let input = vec![1, 5, 6, 7, 2, 4, 5, 5, 6, 8, 4, 2, 3, 4, 6, 8, 10];
        let correct_result = Some(8);
        assert_eq!(find_second_largest(input), correct_result);
    }

    #[test]
    fn test_find_second_largest_negative() {
        let input = vec![1];
        let correct_result = None;
        assert_eq!(find_second_largest(input), correct_result);
    }
}
