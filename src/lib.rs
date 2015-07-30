pub mod unsigned_int;
pub mod signed_int;

pub trait Separatable {
    fn separated_string(&self) -> String;
}
