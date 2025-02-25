use core::fmt;
use std::fmt::{Display, Formatter};

use crate::game::utility;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Default)]
pub struct Move {
    pub start_index: u32,
    pub end_index: u32,
    pub promotion_piece: u8,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let start_string = utility::index_to_string(self.start_index);
        let end_string = utility::index_to_string(self.end_index);

        write!(f, "Move {} -> {}", start_string, end_string)
    }
}

impl Move {
    pub fn new(start_index: u32, end_index: u32, promotion_piece: u8) -> Move {
        Move {
            start_index,
            end_index,
            promotion_piece,
        }
    }

    pub fn new_no_promotion(start_index: u32, end_index: u32) -> Move {
        Move {
            start_index,
            end_index,
            promotion_piece: 0,
        }
    }
}

/// MSB
/// 1 bit        | 1 bit      | 1 bit  | 3 bits            | 1 bit    | 3 bits           
/// ----------------------------------------------------------------------------
/// promotion ID | en-passant | castle | captured piece ID | color ID | piece ID

pub const PIECE_INDEX: u16 = 0;
pub const COLOR_INDEX: u16 = 3;
pub const CAPTURED_PIECE_INDEX: u16 = 4;
pub const CASTLE_INDEX: u16 = 7;
pub const EN_PASSANT_INDEX: u16 = 8;
pub const PROMOTION_INDEX: u16 = 9;

pub fn get_promotion_flag(flags: u16) -> bool {
    flags & (1 << PROMOTION_INDEX) != 0
}

pub fn get_en_passant_flag(flags: u16) -> bool {
    flags & (1 << EN_PASSANT_INDEX) != 0
}

pub fn get_castle_flag(flags: u16) -> bool {
    flags & (1 << CASTLE_INDEX) != 0
}

pub fn get_color_flag(flags: u16) -> u8 {
    ((flags >> COLOR_INDEX) & 0b1) as u8
}

pub fn get_piece_flag(flags: u16) -> u8 {
    ((flags >> PIECE_INDEX) & 0b111) as u8
}

pub fn get_captured_piece_flag(flags: u16) -> u8 {
    ((flags >> CAPTURED_PIECE_INDEX) & 0b111) as u8
}
