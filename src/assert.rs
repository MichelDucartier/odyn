#[macro_export]
macro_rules! assert_eq_u8 {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!(
                "assertion failed: `(left == right)`\nleft: \n{},\n\nright: \n{}",
                format!("{:#010b}", $left),
                format!("{:#010b}", $right),
            );
        }
    };
}
