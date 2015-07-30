use separator::Separatable;

#[test]
fn negative_nine_thousand() {
    let i = -9000 as i16;
    assert_eq!("-9,000".to_owned(), i.separated_string());
}

#[test]
fn negative_nine_hundred() {
    let i = -900 as i16;
    assert_eq!("-900".to_owned(), i.separated_string());
}

#[test]
fn negative_ninety() {
    let i = -90 as i16;
    assert_eq!("-90".to_owned(), i.separated_string());
}

#[test]
fn negative_nine() {
    let i = -9 as i16;
    assert_eq!("-9".to_owned(), i.separated_string());
}

#[test]
fn nine() {
    let i = 9 as i16;
    assert_eq!("9".to_owned(), i.separated_string());
}

#[test]
fn ninety() {
    let i = 90 as i16;
    assert_eq!("90".to_owned(), i.separated_string());
}

#[test]
fn nine_hundred() {
    let i = 900 as i16;
    assert_eq!("900".to_owned(), i.separated_string());
}

#[test]
fn nine_thousand() {
    let i = 9000 as i16;
    assert_eq!("9,000".to_owned(), i.separated_string());
}
