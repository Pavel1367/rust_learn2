fn first<T>(vec: Vec<T>) -> Option<T> {
    vec.into_iter().next()
}