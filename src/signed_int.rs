use Separatable;

macro_rules! separated_int {
    ($string:expr) => {{
        let mut output = String::new();
        let magnitude = if $string.starts_with('-') {
            output.push('-');
            (&$string)[1..].to_owned()
        } else {
            $string
        };

        let mut place = magnitude.len();
        let mut later_loop = false;

        for ch in magnitude.chars() {
            if later_loop && place % 3 == 0 {
                output.push(',');
            }

            output.push(ch);
            later_loop = true;
            place -= 1;
        };

        output
    }};
}

impl Separatable for i16 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_int!(string)
    }
}

impl Separatable for i32 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_int!(string)
    }
}

impl Separatable for i64 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_int!(string)
    }
}
