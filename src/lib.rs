#[macro_use]
mod macros;

pub mod float;
pub mod unsigned_int;
pub mod signed_int;

pub use float::FixedPlaceSeparatable;

/// Used for numbers that can be printed with separators for the thousands places.
pub trait Separatable {

    /// Converts the number to a string with thousands separator.
    fn separated_string(&self) -> String;
}
