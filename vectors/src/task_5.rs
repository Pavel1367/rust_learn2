use std::mem;

fn group_chunks<T>(vec:Vec<T>, chunk_size:usize) ->Vec<Vec<T>> {
    let mut chunks : Vec<Vec<T>>= Vec::new();
    let mut chunk:Vec<T> = Vec::new();
    let last_index = vec.len() - 1;
    for (index, elem) in vec.into_iter().enumerate(){
        chunk.push(elem);
        if chunk.len() == chunk_size || index == last_index{
            let chunk_to_push = mem::take(&mut chunk);
            chunks.push(chunk_to_push);
        }
    }

    chunks
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_basic_chunks() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let result = group_chunks(vec, 3);
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_uneven_chunks() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = group_chunks(vec, 2);
        let expected = vec![vec![1, 2], vec![3, 4], vec![5]];
        assert_eq!(result, expected);
    }
}
