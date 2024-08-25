use ilog::IntLog;

use crate::constants::A_FILE;
use crate::constants::H_FILE;
use crate::constants::RANK_1;
use crate::constants::RANK_8;
use crate::game::magic;
use crate::game::utility;

use super::utility::bishop_mask;
use super::utility::rook_mask;

pub fn generate_knight_moves(knight_board: u64) -> u64 {
    let l1 = (knight_board >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2 = (knight_board >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1 = (knight_board << 1) & 0xfefefefefefefefe;
    let r2 = (knight_board << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
}

pub fn generate_king_moves(king_board: u64) -> u64 {
    let left_moves = utility::west_one(king_board);
    let right_moves = utility::east_one(king_board);

    let mut attacks = king_board | left_moves | right_moves;
    attacks |= utility::south_one(attacks) | utility::north_one(attacks);

    attacks
}

pub fn generate_rook_moves(rook_board: u64, occupancy: u64) -> u64 {
    generate_sliding_moves(
        rook_board,
        occupancy,
        // &utility::relevant_rook_blocking,
        &|blockers, index| blockers & rook_mask(index),
        &magic::ROOK_LOOKUP,
    )
}

pub fn generate_bishop_moves(bishop_board: u64, occupancy: u64) -> u64 {
    generate_sliding_moves(
        bishop_board,
        occupancy,
        &|blockers, index| blockers & bishop_mask(index),
        &magic::BISHOP_LOOKUP,
    )
}

pub fn generate_sliding_moves(
    piece_board: u64,
    occupancy: u64,
    blocker_generator: &dyn Fn(u64, u32) -> u64,
    magic_lookup: &magic::MagicLookup,
) -> u64 {
    let mut remaining = piece_board;

    let mut attacks: u64 = 0;

    while remaining != 0 {
        let piece_index: u32 = remaining.log2().try_into().unwrap();
        let lookup_index: usize = piece_index.try_into().unwrap();

        let relevant_blockers = blocker_generator(occupancy, piece_index)
            & !(1 << piece_index)
            & !A_FILE
            & !H_FILE
            & !RANK_1
            & !RANK_8;

        let magic = magic_lookup.magics[lookup_index];

        let hash: usize = magic::hash_board(relevant_blockers, magic, 13);
        attacks |= magic_lookup.lookup[lookup_index][hash];

        remaining &= !(1 << piece_index);
    }

    attacks
}
