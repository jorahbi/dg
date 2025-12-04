pub trait FromWith<T, P> {
    fn from_with(value: T, param: P) -> Self;
}
