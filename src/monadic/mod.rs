mod extractive;
mod arithmetical;
pub use self::extractive::*;
pub use self::arithmetical::*;

use std::ops::Mul;

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
