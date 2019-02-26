
pub fn reshape<T>(lhs: usize, rhs: <T as IntoIterator>::Item) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Clone {
    std::iter::repeat(rhs).take(lhs).collect::<T<>>()
}
