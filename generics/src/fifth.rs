fn group<T>(list: Vec<T>)-> Vec<(T,T)>{
    let mut iter = list.into_iter();
    let mut result = Vec::new();

    while let (Some(a), Some(b)) = (iter.next(), iter.next()){
        result.push((a,b));
    }
    result

}