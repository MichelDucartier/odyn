mod constants;
mod game;

use game::utility;

use crate::game::bitboard;

fn main() {
    let bboard = bitboard::Bitboard::from_fen(&constants::START_FEN, " ");

    if let Some((row, col)) = utility::string_to_square("e4") {
        println!("row: {}, col: {}", row, col);
    }
}
