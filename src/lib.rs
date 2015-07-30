pub mod unsigned_int;
pub mod signed_int;

/// Used for numbers that can be printed with separators for the thousands places.
pub trait Separatable {

    /// Converts the number to a string with thousands separator.
    fn separated_string(&self) -> String;
}
