use separator::Separatable;

#[test]
fn nine() {
    let i = 9 as u16;
    assert_eq!("9".to_owned(), i.separated_string());
}

#[test]
fn ninety() {
    let i = 90 as u16;
    assert_eq!("90".to_owned(), i.separated_string());
}

#[test]
fn nine_hundred() {
    let i = 900 as u16;
    assert_eq!("900".to_owned(), i.separated_string());
}

#[test]
fn nine_thousand() {
    let i = 9000 as u16;
    assert_eq!("9,000".to_owned(), i.separated_string());
}
