pub mod uint;

pub trait Separatable {
    fn separated_string(&self) -> String;
}
