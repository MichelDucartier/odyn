use odyn::game::utility;

pub fn format_bitboard(bitboard: u64) -> String {
    let mut s = "".to_owned();

    for i in 0..64 {
        let bit = utility::extract_bit(bitboard, i);

        if (i % 8) == 0 && i != 0 {
            s.push_str("\n");
        }

        s.push_str(&bit.to_string());
    }

    return s;
}

macro_rules! assert_eq_bitboard {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!(
                "assertion failed: `(left == right)`\nleft: \n{},\n\nright: \n{}",
                crate::common::formatting::format_bitboard($left),
                crate::common::formatting::format_bitboard($right)
            );
        }
    };
}

pub(crate) use assert_eq_bitboard;
