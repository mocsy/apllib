
use std;
use std::ops::{Add, Sub, Mul, Div};
use num_traits::pow::Pow;
// use crate::dyadic::{map_n_and_n, map_n_and_one};

/// Add y+x
/// Adds y and x
/// 
/// ```
/// assert_eq!(8.4, apllib::add(8.0, 0.4));
/// assert_eq!(8, apllib::add(3, 5));
/// ```
pub fn add<T>(lhs: T, rhs:T) -> T
    where T: Add + From<<T as Add>::Output> {
    T::from(lhs+rhs)
}

/// Map Add over any pair of Iterables
/// 
/// ```
/// let nums = vec![10,20,30];
/// let fives = vec![5,5,5];
/// assert_eq!(vec![15,25,35], apllib::add_map(nums, fives));
/// ```
pub fn add_map<T>(lhs: T, rhs:T) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Add +
    From<<<T as IntoIterator>::Item as Add>::Output> {
    map_n_and_n!(add, lhs, rhs)
}

/// Map Add over any Iterable
/// 
/// ```
/// let nums = vec![10,20,30];
/// let five = 5;
/// assert_eq!(vec![15,25,35], apllib::add_all(nums, five));
/// ```
pub fn add_all<T>(lhs: T, rhs: <T as IntoIterator>::Item) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Add +
    From<<<T as IntoIterator>::Item as Add>::Output>,
    <T as IntoIterator>::Item: Clone {
    map_n_and_one!(add, lhs, rhs)
}

/// Subtract y-x
/// Subtracts y and x
/// 
/// ```
/// assert_eq!(8.0, apllib::subtract(8.4, 0.4));
/// assert_eq!(-2, apllib::subtract(3, 5));
/// ```
pub fn subtract<T>(lhs: T, rhs:T) -> T
    where T: Sub + From<<T as Sub>::Output> {
    T::from(lhs-rhs)
}

/// Map Subtract over any pair of Iterables
/// 
/// ```
/// let nums = vec![10,20,30];
/// let fives = vec![5,5,5];
/// assert_eq!(vec![5,15,25], apllib::subtract_map(nums, fives));
/// ```
pub fn subtract_map<T>(lhs: T, rhs:T) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Sub +
    From<<<T as IntoIterator>::Item as Sub>::Output> {
    map_n_and_n!(subtract, lhs, rhs)
}

/// Map Subtract over any Iterable
/// 
/// ```
/// let nums = vec![10,20,30];
/// let five = 5;
/// assert_eq!(vec![5,15,25], apllib::subtract_all(nums, five));
/// ```
pub fn subtract_all<T>(lhs: T, rhs: <T as IntoIterator>::Item) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Sub +
    From<<<T as IntoIterator>::Item as Sub>::Output>,
    <T as IntoIterator>::Item: Clone {
    map_n_and_one!(subtract, lhs, rhs)
}

/// Multiply y×x
/// Multiplies y with x
/// 
/// ```
/// assert_eq!(3.2, apllib::multiply(8.0, 0.4));
/// assert_eq!(15, apllib::multiply(3, 5));
/// ```
pub fn multiply<T>(lhs: T, rhs:T) -> T
    where T: Mul + From<<T as Mul>::Output> {
    T::from(lhs*rhs)
}

/// Map Multiply over any pair of Iterables
/// 
/// ```
/// let nums = vec![10,20,30];
/// let fives = vec![5,5,5];
/// assert_eq!(vec![50,100,150], apllib::multiply_map(nums, fives));
/// ```
pub fn multiply_map<T>(lhs: T, rhs:T) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Mul +
    From<<<T as IntoIterator>::Item as Mul>::Output> {
    map_n_and_n!(multiply, lhs, rhs)
}

/// Map Multiply over any Iterable
/// 
/// ```
/// let nums = vec![10,20,30];
/// let five = 5;
/// assert_eq!(vec![50,100,150], apllib::multiply_all(nums, five));
/// ```
pub fn multiply_all<T>(lhs: T, rhs: <T as IntoIterator>::Item) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Mul +
    From<<<T as IntoIterator>::Item as Mul>::Output>,
    <T as IntoIterator>::Item: Clone {
    map_n_and_one!(multiply, lhs, rhs)
}

/// Divide y÷x
/// Divide y with x
/// 
/// ```
/// assert_eq!(6, apllib::divide(30, 5));
/// assert_eq!(64.4912, apllib::divide(322.456, 5.0));
/// ```
pub fn divide<T> (y : T, divisor : T) -> T
    where T: Div + From<<T as Div>::Output> {
    T::from(y/divisor)
}

/// Map Divide over any pair of Iterables
/// 
/// ```
/// let nums = vec![10,20,30];
/// let fives = vec![5,5,5];
/// assert_eq!(vec![2,4,6], apllib::divide_map(nums, fives));
/// ```
pub fn divide_map<T> (iterable : T, divisors : T) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Div +
    From<<<T as IntoIterator>::Item as Div>::Output> {
    map_n_and_n!(divide, iterable, divisors)
}

/// Map Divide over any Iterable
/// 
/// ```
/// let nums = vec![10,20,30];
/// let five = 5;
/// assert_eq!(vec![2,4,6], apllib::divide_all(nums, five));
/// ```
pub fn divide_all<T>(lhs: T, rhs: <T as IntoIterator>::Item) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Div +
    From<<<T as IntoIterator>::Item as Div>::Output>,
    <T as IntoIterator>::Item: Clone {
    map_n_and_one!(divide, lhs, rhs)
}

/// Power y*x
/// y to the power x
/// 
/// ```
/// assert_eq!(100.0, apllib::power(10.0, 2.0));
/// assert_eq!(30.25, apllib::power(5.5, 2.0));
/// ```
pub fn power<T> (y : T, x : T) -> T
    where T: Pow<T> + From<<T as Pow<T>>::Output> {
    T::from(y.pow(x))
}

/// Map Power over any pair of Iterables
/// 
/// ```
/// let nums = vec![10.0,20.0,30.0];
/// let fives = vec![2.0,2.0,2.0];
/// assert_eq!(vec![100.0,400.0,900.0], apllib::power_map(nums, fives));
/// ```
pub fn power_map<T> (lhs : T, rhs : T) -> T
    where T:IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Pow<<T as IntoIterator>::Item> +
    From<<<T as IntoIterator>::Item as Pow<<T as IntoIterator>::Item>>::Output> {
    map_n_and_n!(power, lhs, rhs)
}

/// Map Power over any Iterable
/// 
/// ```
/// let nums = vec![10.0,20.0,30.0];
/// let five = 2.0;
/// assert_eq!(vec![100.0,400.0,900.0], apllib::power_all(nums, five));
/// ```
pub fn power_all<T>(lhs: T, rhs: <T as std::iter::IntoIterator>::Item) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Pow<<T as IntoIterator>::Item> +
    From<<<T as IntoIterator>::Item as Pow<<T as IntoIterator>::Item>>::Output>,
    <T as IntoIterator>::Item: Clone {
    map_n_and_one!(power, lhs, rhs)
}

#[test]
/// (?t)÷t←8⍴10000
fn apl_rand_in_range_0_1() {
    use crate::monadic::{roll_map};
    use crate::nonscalar::reshape;
    // Instead of 8 (⍴)Reshape 10000
    let tenthousands: Vec<f64> = reshape(8,10000.);

    // (?t)÷t
    let res = divide_map(roll_map(tenthousands.clone()), tenthousands);
    for r in res {
        assert!(r < 1.);
    }
}