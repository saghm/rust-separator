use Separatable;

macro_rules! separated_uint {
    ($string:expr) => {{
        let mut place = $string.len();
        let mut output = String::new();
        let mut later_loop = false;

        for ch in $string.chars() {
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

impl Separatable for u16 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_uint!(string)
    }
}

impl Separatable for u32 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_uint!(string)
    }
}

impl Separatable for u64 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_uint!(string)
    }
}
