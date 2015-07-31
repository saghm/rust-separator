use Separatable;

impl Separatable for f32 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_float!(string)
    }
}

impl Separatable for f64 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_float!(string)
    }
}
