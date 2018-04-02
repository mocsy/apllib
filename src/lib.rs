//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}

extern crate euclid;
extern crate rand;
use std::ops::Mul;
use euclid::num::Ceil;
use euclid::num::Floor;
// log, exp need trait as well
use std::ops::Neg;
use rand::{Rng, thread_rng};

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
pub fn sign<T> (x : T) -> T
    where T: PartialOrd + Default + From<i8> {
    if x < T::default() { return T::from(-1) }
    T::from(1)
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
/// A random number
///
/// ```
/// assert_ne!(8.0, apllib::roll(8.0));
/// assert_ne!(128.0, apllib::roll(128.0));
/// assert_ne!(123456789.0, apllib::roll(123456789.0));
/// ```
pub fn roll (x: f64) -> f64 {
    thread_rng().gen_range::<f64>(0.0, x)
}
