use ilog::IntLog;

use crate::constants;
use crate::constants::A_FILE_MASK;
use crate::constants::B_FILE_MASK;
use crate::constants::C_FILE_MASK;
use crate::constants::D_FILE_MASK;
use crate::constants::F_FILE_MASK;
use crate::constants::G_FILE_MASK;
use crate::constants::H_FILE_MASK;
use crate::constants::RANK_1_INDEX;
use crate::constants::RANK_1_MASK;
use crate::constants::RANK_2_MASK;
use crate::constants::RANK_3_INDEX;
use crate::constants::RANK_6_INDEX;
use crate::constants::RANK_7_MASK;
use crate::constants::RANK_8_INDEX;
use crate::constants::RANK_8_MASK;
use crate::game::magic;
use crate::game::utility;
use crate::game::utility::rook_rank_to_board;

use super::bitboard;
use super::utility::bishop_mask;

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

pub fn generate_king_castle(color: u8, flags: u8, occupancy: u64) -> u64 {
    let king_castle;
    let queen_castle;
    let piece_offset;
    let mut queen_side_occupancy = occupancy & (B_FILE_MASK | C_FILE_MASK | D_FILE_MASK);
    let mut king_side_occupancy = occupancy & (F_FILE_MASK | G_FILE_MASK);

    if color == constants::WHITE_ID {
        king_castle = (flags >> bitboard::WKCASTLE_F_INDEX) & 0b1;
        queen_castle = (flags >> bitboard::WQCASTLE_F_INDEX) & 0b1;
        piece_offset = utility::square_to_index(RANK_1_INDEX, 0);
        king_side_occupancy = king_side_occupancy & RANK_1_MASK;
        queen_side_occupancy = queen_side_occupancy & RANK_1_MASK;
    } else {
        king_castle = (flags >> bitboard::BKCASTLE_F_INDEX) & 0b1;
        queen_castle = (flags >> bitboard::BQCASTLE_F_INDEX) & 0b1;
        piece_offset = utility::square_to_index(RANK_8_INDEX, 0);
        king_side_occupancy = king_side_occupancy & RANK_8_MASK;
        queen_side_occupancy = queen_side_occupancy & RANK_8_MASK;
    }
    let mut rank_castling_moves = 0;
    if king_side_occupancy == 0 {
        rank_castling_moves |= king_castle << 6;
    }
    if queen_side_occupancy == 0 {
        rank_castling_moves |= queen_castle << 6;
    }

    u64::from(rank_castling_moves) << piece_offset
}

pub fn generate_rook_moves(rook_board: u64, occupancy: u64) -> u64 {
    generate_sliding_moves(
        rook_board,
        occupancy,
        &|blockers, index| blockers & rook_rank_to_board(0x7e, 0x7e, index),
        &magic::ROOK_LOOKUP,
    )
}

pub fn generate_bishop_moves(bishop_board: u64, occupancy: u64) -> u64 {
    generate_sliding_moves(
        bishop_board,
        occupancy,
        &|blockers, index| {
            blockers
                & bishop_mask(index)
                & !A_FILE_MASK
                & !H_FILE_MASK
                & !RANK_1_MASK
                & !RANK_8_MASK
        },
        &magic::BISHOP_LOOKUP,
    )
}

pub fn generate_queen_moves(queen_board: u64, occupancy: u64) -> u64 {
    generate_bishop_moves(queen_board, occupancy) | generate_rook_moves(queen_board, occupancy)
}

pub fn generate_pawn_moves(pawn_board: u64, occupancy: u64, color: u8) -> u64 {
    if color == constants::WHITE_ID {
        let temp = pawn_board & !RANK_8_MASK;
        let start_moves = (pawn_board & RANK_2_MASK) >> 16;
        return (start_moves | (temp >> 8)) & !occupancy;
    }

    let temp = pawn_board & !RANK_1_MASK;
    let start_moves = (pawn_board & RANK_7_MASK) << 16;
    (start_moves | (temp << 8)) & !occupancy
}

pub fn generate_pawn_attacks(pawn_board: u64, color: u8, en_passant: u8) -> u64 {
    if color == constants::WHITE_ID {
        let temp = pawn_board & !RANK_8_MASK;
        let en_passant_attacks = (en_passant as u64) << RANK_6_INDEX;
        return ((temp & !H_FILE_MASK) >> 7) | ((temp & !A_FILE_MASK) >> 9) | en_passant_attacks;
    }

    let temp = pawn_board & !RANK_1_MASK;
    let en_passant_attacks = (en_passant as u64) << RANK_3_INDEX;
    ((temp & !H_FILE_MASK) << 7) | ((temp & !A_FILE_MASK) << 9) | en_passant_attacks
}

fn generate_sliding_moves(
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

        let relevant_blockers = blocker_generator(occupancy, piece_index) & !(1 << piece_index);

        let magic = magic_lookup.magics[lookup_index];

        let hash: usize = magic::hash_board(relevant_blockers, magic, 13);
        attacks |= magic_lookup.lookup[lookup_index][hash];

        remaining &= !(1 << piece_index);
    }

    attacks
}
