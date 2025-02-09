pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const WHITE_ID: u8 = 1;
pub const BLACK_ID: u8 = 0;

pub const EMPTY_ID: u8 = 0;
pub const PAWN_ID: u8 = 1;
pub const KNIGHT_ID: u8 = 2;
pub const BISHOP_ID: u8 = 3;
pub const ROOK_ID: u8 = 4;
pub const QUEEN_ID: u8 = 5;
pub const KING_ID: u8 = 6;

pub const A_FILE_MASK: u64 = 0x0101010101010101;
pub const H_FILE_MASK: u64 = 0x8080808080808080;
pub const RANK_8_MASK: u64 = 0xff00000000000000;
pub const RANK_1_MASK: u64 = 0x00000000000000ff;

pub const RANK_1_INDEX: u32 = 7;
pub const RANK_2_INDEX: u32 = 6;
pub const RANK_3_INDEX: u32 = 5;
pub const RANK_4_INDEX: u32 = 4;
pub const RANK_5_INDEX: u32 = 3;
pub const RANK_6_INDEX: u32 = 2;
pub const RANK_7_INDEX: u32 = 1;
pub const RANK_8_INDEX: u32 = 0;

pub const FILE_H_INDEX: u32 = 7;
pub const FILE_G_INDEX: u32 = 6;
pub const FILE_F_INDEX: u32 = 5;
pub const FILE_E_INDEX: u32 = 4;
pub const FILE_D_INDEX: u32 = 3;
pub const FILE_C_INDEX: u32 = 2;
pub const FILE_B_INDEX: u32 = 1;
pub const FILE_A_INDEX: u32 = 0;
