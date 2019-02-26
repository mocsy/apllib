use std;
use std::ops::{Mul};
use euclid::num::{Ceil, Floor};

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
