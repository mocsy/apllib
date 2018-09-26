
use std;
use std::ops::Div;

pub fn divide<T> (x : T, divisor : T) -> T
    where T: Div + From<<T as std::ops::Div>::Output> {
    T::from(x/divisor)
}

pub fn divide_map<T> (iterable : T, divisors : T) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: std::ops::Div +
    From<<<T as std::iter::IntoIterator>::Item as std::ops::Div>::Output> {
    iterable.into_iter().zip(divisors).map(|(i,j)| divide(i, j)).collect()
}

#[test]
/// (?t)÷t←8⍴10000
fn apl_rand_in_range_0_1() {
    use monadic::{roll_map};
    // Instead of 8 (⍴)Reshape 10000
    let inp = 10000.;
    let tenthousands = std::iter::repeat(inp).take(8).collect::<Vec<_>>();

    // (?t)÷t
    let res = divide_map(roll_map(tenthousands.clone()), tenthousands);
    for r in res {
        assert!(r < 1.);
    }
}