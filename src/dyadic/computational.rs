use num_traits::pow::Pow;
use std::ops::Rem;

/// Circle y∘x
/// Calls a circle function y on x
///
/// ## Notation for the Circle Functions
/// |    A+ Expression     |   Meaning   ||    A+ Expression     |   Meaning   |
/// |----------------------|-------------||----------------------|-------------|
/// | `sinarccos ∘x or 0∘x | (1-x*2)*0.5 ||                      |             |
/// |       `sin ∘x or 1∘x |    sin x    ||   `arcsin ∘x or -1∘x |   arcsin x  |
/// |       `cos ∘x or 2∘x |    cos x    ||   `arccos ∘x or -2∘x |   arccos x  |
/// |       `tan ∘x or 3∘x |    tan x    ||   `arctan ∘x or -3∘x |   arctan x  |
/// | `secarctan ∘x or 4∘x | (1+x*2)*0.5 ||`tanarcsec ∘x or -4∘x |(-1+x*2)*0.5 |
/// |      `sinh ∘x or 5∘x |    sinh x   ||  `arcsinh ∘x or -5∘x |  arcsinh x  |
/// |      `cosh ∘x or 6∘x |    cosh x   ||  `arccosh ∘x or -6∘x |  arccosh x  |
/// |      `tanh ∘x or 7∘x |    tanh x   ||  `arctanh ∘x or -7∘x |  arctanh x  |
/// where * is power
/// ```
/// assert_eq!(0.714142842854285, apllib::circle(0, 0.7));
/// assert_eq!(0.644217687237691, apllib::circle(1, 0.7));
/// assert_eq!(0.7648421872844885, apllib::circle(2, 0.7));
/// assert_eq!(0.8422883804630794, apllib::circle(3, 0.7));
/// assert_eq!(1.2206555615733703, apllib::circle(4, 0.7));
/// assert_eq!(0.7585837018395334, apllib::circle(5, 0.7));
/// assert_eq!(1.255169005630943, apllib::circle(6, 0.7));
/// assert_eq!(0.6043677771171635, apllib::circle(7, 0.7));
///
/// assert_eq!(0.775397496610753, apllib::circle(-1, 0.7));
/// assert_eq!(0.7953988301841436, apllib::circle(-2, 0.7));
/// assert_eq!(0.6107259643892086, apllib::circle(-3, 0.7));
/// assert_eq!(1.3747727084867518, apllib::circle(-4, 1.7));
/// assert_eq!(0.6526665660823557, apllib::circle(-5, 0.7));
/// assert_eq!(1.123230982587296, apllib::circle(-6, 1.7));
/// assert_eq!(0.8673005276940531, apllib::circle(-7, 0.7));
///
/// assert!(apllib::circle(-4, 0.7).is_nan());
/// assert!(apllib::circle(-6, 0.7).is_nan());
/// assert!(apllib::circle(8, 0.7).is_nan());
/// assert!(apllib::circle(-8, 0.7).is_nan());
/// ```
pub fn circle(lhs: i8, rhs: f64) -> f64 {
    match lhs {
        0 => {
            let i: f64 = 1.0 - rhs.pow(2.0);
            i.pow(0.5)
        }
        1 => rhs.sin(),
        2 => rhs.cos(),
        3 => rhs.tan(),
        4 => {
            let i: f64 = 1f64 + rhs.pow(2);
            i.pow(0.5)
        }
        5 => rhs.sinh(),
        6 => rhs.cosh(),
        7 => rhs.tanh(),
        -1 => rhs.asin(),
        -2 => rhs.acos(),
        -3 => rhs.atan(),
        -4 => {
            let i: f64 = -1f64 + rhs.pow(2);
            i.pow(0.5)
        }
        -5 => {
            if rhs == std::f64::NEG_INFINITY {
                return std::f64::NEG_INFINITY;
            }
            let i: f64 = 1f64 + rhs.pow(2);
            let j: f64 = rhs + i.sqrt();
            crate::natural_log(j)
        }
        -6 => {
            let i: f64 = (rhs - 1f64).sqrt();
            let j: f64 = (rhs + 1f64).sqrt();
            crate::natural_log(rhs + i * j)
        }
        -7 => {
            let i: f64 = crate::natural_log(1f64 + rhs);
            let j: f64 = crate::natural_log(1f64 - rhs);
            0.5 * (i - j)
        }
        _ => std::f64::NAN
    }
}

pub fn circle_map(lhs: i8, rhs: &[f64]) -> Vec<f64> {
    rhs.iter().map(|i| circle(lhs, *i)).collect()
}
pub fn circle_map32(lhs: i8, rhs: &[f32]) -> Vec<f32> {
    rhs.iter().map(|i| circle(lhs, (*i).into()) as f32).collect()
}


/// Residue y|x
/// remainder when x is divided by y
/// 0|x is 0
/// ```
/// assert_eq!(1, apllib::residue(31, 5));
/// ```
pub fn residue<T>(lhs: T, rhs: T) -> T
    where T: Rem + From<<T as Rem>::Output> {
    T::from(lhs%rhs)
}

/// Map Residue over any pair of Iterables
/// 
/// ```
/// let nums = vec![11,22,33];
/// let fives = vec![5,5,5];
/// assert_eq!(vec![1,2,3], apllib::residue_map(nums, fives));
/// ```
pub fn residue_map<T>(lhs: T, rhs:T) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Rem +
    From<<<T as IntoIterator>::Item as Rem>::Output> {
    map_n_and_n!(residue, lhs, rhs)
}

/// Map Residue over any Iterable
/// 
/// ```
/// let nums = vec![11,22,33];
/// let five = 5;
/// assert_eq!(vec![1,2,3], apllib::residue_all(nums, five));
/// ```
pub fn residue_all<T>(lhs: T, rhs: <T as IntoIterator>::Item) -> T
    where T: IntoIterator +
    std::iter::FromIterator<<T as IntoIterator>::Item>,
    <T as IntoIterator>::Item: Rem +
    From<<<T as IntoIterator>::Item as Rem>::Output>,
    <T as IntoIterator>::Item: Clone {
    map_n_and_one!(residue, lhs, rhs)
}

