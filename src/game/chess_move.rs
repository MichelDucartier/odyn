#[derive(Clone)]
pub struct Move {
    pub start_index: u32,
    pub end_index: u32,
}

/// MSB
/// 1 bit      | 1 bit  | 3 bits                    | 1 bit            | 3 bits           
/// -------------------------------------------------------------------------------------
/// en-passant | castle | captured piece identifier | color identifier | piece identifier

pub const PIECE_INDEX: u16 = 0;
pub const COLOR_INDEX: u16 = 3;
pub const CAPTURED_PIECE_INDEX: u16 = 4;
pub const CASTLE_INDEX: u16 = 7;
pub const EN_PASSANT_INDEX: u16 = 8;

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
