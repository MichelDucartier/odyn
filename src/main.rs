use odyn::constants;
use odyn::game::utility;

use odyn::game::bitboard;

fn main() {
    // let x = 0xFF00000000000000;
    // let x = 0x8040201008040201;
    // let x: u64 = 0x0102040810204080;
    // let x = 0xCCCCCCCCCCCCCCCC;
    // let x = 0xF0F0F0F0F0F0F0F0;
    // let x = 0xff << 56;
    // println!("{}\n", utility::format_bitboard(x));
    // println!(
    //     "{}",
    //     utility::format_bitboard(utility::pseudo_rotate_45_anticlockwise(x))
    // )
    let x: u32 = 16;
    println!("{}", x & 0b111000);
}
