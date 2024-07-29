use odyn::game::utility::{
    extract_bit, index_to_square, square_to_index, square_to_string, string_to_square,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};

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

#[test]
fn test_square_to_index_f3() {
    let res = square_to_index(3, 5);
    assert_eq!(29, res);
}

#[test]
fn test_index_to_square_b1() {
    let res = index_to_square(57);
    assert_eq!((7, 1), res);
}

#[test]
fn test_extract_bit_with_1() {
    let res = extract_bit(1 << 42, 42);
    assert_eq!(1, res);
}

#[test]
fn test_extract_bit_with_0() {
    let mut rng = StdRng::seed_from_u64(42);
    let r = rng.next_u64();
    let res = extract_bit(r & (0 << 61), 61);
    assert_eq!(0, res);
}
