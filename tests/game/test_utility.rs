use odyn::game::utility::{self, square_to_string, string_to_square};

#[test]
fn test_string_to_square_h4() {
    let res = string_to_square("h4");
    assert_eq!(Some((4, 7)), res);
}

#[test]
fn test_square_to_string_h4() {
    let res = square_to_string(4, 7);
    assert_eq!("h4", res);
}
