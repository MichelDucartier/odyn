use ilog::IntLog;

use super::bitboard::Bitboard;
use super::magic::ROOK_LOOKUP;
use crate::constants::A_FILE;
use crate::constants::H_FILE;
use crate::constants::RANK_1;
use crate::constants::RANK_8;
use crate::game::magic;
use crate::game::utility;

pub fn generate_knight_moves(knight_board: u64) -> u64 {
    let l1 = (knight_board >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2 = (knight_board >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1 = (knight_board << 1) & 0xfefefefefefefefe;
    let r2 = (knight_board << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    return (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);
}

pub fn generate_king_moves(king_board: u64) -> u64 {
    let left_moves = utility::west_one(king_board);
    let right_moves = utility::east_one(king_board);

    let mut attacks = king_board | left_moves | right_moves;
    attacks |= utility::south_one(attacks) | utility::north_one(attacks);

    return attacks;
}

pub fn generate_rook_moves(rook_board: u64, occupancy: u64) -> u64 {
    let mut remaining = rook_board;

    let mut attacks: u64 = 0;

    while remaining != 0 {
        let rook_index: u32 = remaining.log2().try_into().unwrap();
        let lookup_index: usize = rook_index.try_into().unwrap();

        let relevant_blockers =
            utility::relevant_rook_blocking(occupancy, rook_index) & !(1 << rook_index);

        let magic = ROOK_LOOKUP.magics[lookup_index];

        println!("{rook_index}");
        println!("Blocking:\n{}", utility::format_bitboard(relevant_blockers));

        let hash: usize = magic::hash_board(relevant_blockers, magic, 13);
        attacks |= magic::ROOK_LOOKUP.lookup[lookup_index][hash];

        println!(
            "Attack:\n{}\n",
            utility::format_bitboard(magic::ROOK_LOOKUP.lookup[lookup_index][hash])
        );

        remaining &= !(1 << rook_index);
    }

    attacks
}
