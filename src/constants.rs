/// Standard chess starting position in FEN.
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Internal color id for white pieces.
pub const WHITE_ID: u8 = 1;
/// Internal color id for black pieces.
pub const BLACK_ID: u8 = 0;

pub const CHESSBOARD_SIZE: i64 = 8;

/// Piece identifiers used throughout move generation and board state.
pub const EMPTY_ID: u8 = 0;
pub const PAWN_ID: u8 = 1;
pub const KNIGHT_ID: u8 = 2;
pub const BISHOP_ID: u8 = 3;
pub const ROOK_ID: u8 = 4;
pub const QUEEN_ID: u8 = 5;
pub const KING_ID: u8 = 6;

pub const A_FILE_MASK: u64 = 0x0101010101010101;
pub const B_FILE_MASK: u64 = 0x0202020202020202;
pub const C_FILE_MASK: u64 = 0x0404040404040404;
pub const D_FILE_MASK: u64 = 0x0808080808080808;
pub const E_FILE_MASK: u64 = 0x1010101010101010;
pub const F_FILE_MASK: u64 = 0x2020202020202020;
pub const G_FILE_MASK: u64 = 0x4040404040404040;
pub const H_FILE_MASK: u64 = 0x8080808080808080;

pub const RANK_1_MASK: u64 = 0xff << (RANK_1_INDEX * 8);
pub const RANK_2_MASK: u64 = 0xff << (RANK_2_INDEX * 8);
pub const RANK_3_MASK: u64 = 0xff << (RANK_3_INDEX * 8);
pub const RANK_4_MASK: u64 = 0xff << (RANK_4_INDEX * 8);
pub const RANK_5_MASK: u64 = 0xff << (RANK_5_INDEX * 8);
pub const RANK_6_MASK: u64 = 0xff << (RANK_6_INDEX * 8);
pub const RANK_7_MASK: u64 = 0xff << (RANK_7_INDEX * 8);
pub const RANK_8_MASK: u64 = 0xff << (RANK_8_INDEX * 8);

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

/// Piece ids iterated when checking attacks.
pub const ALL_PIECES_ID: [u8; 6] = [PAWN_ID, KNIGHT_ID, BISHOP_ID, ROOK_ID, QUEEN_ID, KING_ID];
pub const SLIDING_PIECES_ID: [u8; 3] = [BISHOP_ID, ROOK_ID, QUEEN_ID];
pub const NON_SLIDING_PIECES_ID: [u8; 3] = [PAWN_ID, KNIGHT_ID, KING_ID];

/// Valid promotion piece ids accepted by UCI move parsing.
pub const POSSIBLE_PROMOTION: [u8; 4] = [KNIGHT_ID, BISHOP_ID, ROOK_ID, QUEEN_ID];

/// UCI protocol command keywords.
pub const IS_READY_COMMAND: &str = "isready";
pub const SET_OPTION_COMMAND: &str = "setoption";
pub const DEBUG_COMMAND: &str = "debug";
pub const UCI_COMMAND: &str = "uci";
pub const REGISTER_COMMAND: &str = "register";
pub const UCINEWGAME_COMMAND: &str = "ucinewgame";
pub const POSITION_COMMAND: &str = "position";
pub const GO_COMMAND: &str = "go";
pub const STOP_COMMAND: &str = "stop";
pub const PONDERHIT_COMMAND: &str = "ponderhit";
pub const QUIT_COMMAND: &str = "quit";

/// UCI protocol acknowledgement responses.
pub const READY_OK: &str = "readyok";
pub const UCI_OK: &str = "uciok";

/// Simplistic piece values used by the default evaluator.
pub const PIECE_VALUES: [f32; 7] = [0.0, 1.0, 3.0, 3.0, 5.0, 9.0, f32::INFINITY];

/// Returns the opposite color id.
pub fn opposite(color_id: u8) -> u8 {
    match color_id {
        WHITE_ID => BLACK_ID,
        BLACK_ID => WHITE_ID,
        _ => panic!("Invalid color id passed: {}", color_id),
    }
}
