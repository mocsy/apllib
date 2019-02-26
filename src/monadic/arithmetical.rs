use std::ops::Neg;
use rand::{Rng, thread_rng};
use rand::distributions::uniform::SampleUniform;

/// Reciprocal x
/// 1/x
///
/// ```
/// assert_eq!(0.125, apllib::reciprocal(8.0));
/// assert_eq!(0.0078125, apllib::reciprocal(128.0));
/// assert_eq!(0.000000008100000073710001, apllib::reciprocal(123456789.0));
/// ```
pub fn reciprocal (x: f64) -> f64 {
    1.0/x
}

/// Negate -x
/// The result is x negated, ex.: 0-x fot Int types
///
/// ```
/// assert_eq!(-8.0, apllib::negate(8.0));
/// assert_eq!(-128.0, apllib::negate(128.0));
/// assert_eq!(-123456789.0, apllib::negate(123456789.0));
/// ```
pub fn negate<T> (x: T) -> T
    where T: Neg + Copy
    + From<<T as std::ops::Neg>::Output> {
    T::from(x.neg())
}

/// Identity +x
/// The result is identical to x
/// Note: a funny a+ function
///
/// ```
/// assert_eq!(8.0, apllib::idenity(8.0));
/// assert_eq!(128.0, apllib::floor(128.0));
/// assert_eq!(123456789.0, apllib::floor(123456789.0));
/// ```
pub fn idenity<T> (x: T) -> T
    where T: Copy {
    x
}

/// Exponential *x
/// e (2.71828...) to the power x
/// Note: no idea how correct these numbers are, but should be close
/// 
/// ```
/// assert_eq!(7.3890560989306495, apllib::exponential(2));
/// assert_eq!(54.59815003314423, apllib::exponential(4));
/// assert_eq!(403.428793492735, apllib::exponential(6));
/// ```
pub fn exponential (x: i32) -> f64 {
    let e = std::f64::consts::E;
    e.powi(x)
}

/// Natural log x
/// The logarithm of x to the base e (2.718281828...)
///
/// ```
/// assert_eq!(2.0794415416798357, apllib::natural_log(8.0));
/// assert_eq!(4.852030263919617, apllib::natural_log(128.0));
/// assert_eq!(18.63140176616802, apllib::natural_log(123456789.0));
/// ```
pub fn natural_log (x: f64) -> f64 {
    x.log(std::f64::consts::E)
}

/// Pi times x
/// Pi (3.14159...) times x
///
/// ```
/// assert_eq!(25.132741228718345, apllib::pi_times(8.0));
/// assert_eq!(402.1238596594935, apllib::pi_times(128.0));
/// assert_eq!(387850941.3581852, apllib::pi_times(123456789.0));
/// ```
pub fn pi_times (x: f64) -> f64 {
    x * std::f64::consts::PI
}

/// Roll x
/// A random number from default (usually 0) to (x-1) if x < default
/// or (x-1) to default otherwise
///
/// ```
/// assert!(8.1 > apllib::roll(8.0));
/// assert!(128.1 > apllib::roll(128.0));
/// assert!(123456789.1 > apllib::roll(123456789.0));
/// assert!(-4.1 < apllib::roll(-4) as f32);
/// ```
pub fn roll<T> (x: T) -> T
    where T: PartialOrd + SampleUniform + Default  {
    let (small, big);
    if x < T::default() {
        small = x;
        big = T::default();
    } else {
        small = T::default();
        big = x;
    }
    thread_rng().gen_range::<T>(small, big)
}

/// Maps Roll over any Iterable
/// 
/// ```
/// let a = vec![1, 1, 1, -4];
/// let rolls = apllib::roll_map(a);
/// for r in rolls {
///     assert!(1.1 > r as f32);
/// }
/// ```
/// #[doc(alias = "?")]
pub fn roll_map<T> (iterable : T) -> T
    where T : IntoIterator
    + std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: PartialOrd + SampleUniform + Default {
    iterable.into_iter().map(roll).collect()
}
