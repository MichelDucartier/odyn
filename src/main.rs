use odyn::constants;
use odyn::game::utility;

use odyn::game::bitboard;

fn main() {
    let bboard = bitboard::Bitboard::from_fen(&constants::START_FEN, " ");

    if let Some((row, col)) = utility::string_to_square("e4") {
        println!("row: {}, col: {}", row, col);
    }
}
