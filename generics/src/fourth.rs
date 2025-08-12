enum Either<L,R>{
    Left(L),
    Right(R)
}

impl<L,R> Either<L,R>{
    fn left(self)->Option<L>{
        match self {
            Either::Left(l) => Some(l),
            _ => None
        }
    }
    fn right(self)->Option<R>{
        match self {
            Either::Right(r) => Some(r),
            _ => None
        }
    }
    fn is_left(&self)->bool{
        match self{
            Either::Left(_) => true,
            _ => false
        }
    }
    fn is_right(&self)->bool{
        match self{
            Either::Right(_) => true,
            _ => false
        }
    }
}