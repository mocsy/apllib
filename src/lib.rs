#[macro_use]
mod macros;

pub mod monadic;
pub use monadic::*;

pub mod dyadic;
pub use dyadic::*;

pub mod nonscalar;
pub use nonscalar::*;