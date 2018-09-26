
extern crate euclid;
extern crate rand;
use std;
use std::ops::Mul;
use self::euclid::num::{Ceil, Floor};
// log, exp need trait as well
use std::ops::Neg;
use self::rand::{Rng, thread_rng};
use self::rand::distributions::uniform::SampleUniform;

/// Sign of value x
/// Signum x
/// ex.: The value of sign(x) is -1 for negative integers, 0 for zero, and 1 for positive integers
/// 
/// ```
/// assert_eq!(-1, apllib::sign(-8));
/// assert_eq!(-1, apllib::sign(-128));
/// assert_eq!(-1, apllib::sign(-123456789));
/// assert_eq!(1, apllib::sign(123456789));
/// ```
/// #[doc(alias = "×")]
pub fn sign<T> (x : T) -> T
    where T: PartialOrd + Default + From<i8> {
    if x < T::default() { return T::from(-1) }
    T::from(1)
}

/// Maps Sign over any Iterable
/// 
/// ```
/// let a = vec![1, 2, 3, -4];
/// let signs = apllib::sign_map(a);
/// assert_eq!(vec![1, 1, 1, -1], signs);
/// 
/// use std::collections::HashSet;
/// 
/// let mut b = HashSet::new();
/// b.insert(1);
/// b.insert(2);
/// b.insert(3);
/// b.insert(-4);
/// let mut s = HashSet::new();
/// s.insert(1);
/// s.insert(-1);
/// let signs = apllib::sign_map(b);
/// assert_eq!(s, signs);
/// ```
/// #[doc(alias = "×")]
pub fn sign_map<T> (iterable : T) -> T
    where T : IntoIterator
    + std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: PartialOrd + Default + From<i8> {
    iterable.into_iter().map(sign).collect()
}

/// Absolute value |x
/// |x is equivalent to x times Sign of x
/// 
/// ```
/// assert_eq!(8, apllib::abs(-8));
/// assert_eq!(128, apllib::abs(-128));
/// assert_eq!(123456789, apllib::abs(-123456789));
/// ```
pub fn abs<T> (x : T) -> T
    where T: PartialOrd + Default + From<i8>
    + Mul + Clone
    + From<<T as std::ops::Mul>::Output>  {
    let s = sign(x.clone());
    T::from(x * s)
}

/// Ceiling x
/// The smallest nonfractional number greater than x
/// Note: no effort made to reproduce near 0 behaviour of a+
///
/// ```
/// assert_eq!(9.0, apllib::ceiling(8.4));
/// assert_eq!(129.0, apllib::ceiling(128.2));
/// assert_eq!(123456790.0, apllib::ceiling(123456789.9));
/// ```
pub fn ceiling<T> (x: T) -> T 
    where T: Ceil + Copy {
    x.ceil()
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

/// Floor x
/// The largest nonfractional number less than x
/// Note: no effort made to reproduce near 0 behaviour of a+
///
/// ```
/// assert_eq!(8.0, apllib::floor(8.4));
/// assert_eq!(128.0, apllib::floor(128.2));
/// assert_eq!(123456789.0, apllib::floor(123456789.9));
/// ```
pub fn floor<T> (x: T) -> T 
    where T: Floor + Copy {
    x.floor()
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

/// Not -x
/// The value is true if abs(x) is 0, false otherwise
///
/// ```
/// assert_eq!(false, apllib::not(8.0));
/// assert_eq!(false, apllib::not(128.0));
/// assert_eq!(false, apllib::not(123456789.0));
/// assert_eq!(true, apllib::not(0));
/// ```
pub fn not<T> (x: T) -> bool
    where T: PartialOrd + Default + From<i8>
    + Mul + Clone
    + From<<T as std::ops::Mul>::Output>  {
    if abs(x) == T::default() { return true }
    false
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
    f64::from(x * std::f64::consts::PI)
}

/// Reciprocal x
/// 1/x
///
/// ```
/// assert_eq!(0.125, apllib::reciprocal(8.0));
/// assert_eq!(0.0078125, apllib::reciprocal(128.0));
/// assert_eq!(0.000000008100000073710001, apllib::reciprocal(123456789.0));
/// ```
pub fn reciprocal (x: f64) -> f64 {
    f64::from(1.0/x)
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
    let rand = thread_rng().gen_range::<T>(small, big);
    T::from(rand)
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
