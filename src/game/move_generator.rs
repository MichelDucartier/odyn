use super::bitboard::Bitboard;
use crate::game::utility;

impl Bitboard {
    pub fn generate_knight_moves(&self) -> u64 {
        let l1 = (self.knight_board >> 1) & 0x7f7f7f7f7f7f7f7f;
        let l2 = (self.knight_board >> 2) & 0x3f3f3f3f3f3f3f3f;
        let r1 = (self.knight_board << 1) & 0xfefefefefefefefe;
        let r2 = (self.knight_board << 2) & 0xfcfcfcfcfcfcfcfc;
        let h1 = l1 | r1;
        let h2 = l2 | r2;
        return (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);
    }

    pub fn generate_king_moves(&self) -> u64 {
        let left_moves = utility::west_one(self.king_board);
        let right_moves = utility::east_one(self.king_board);

        let mut attacks = self.king_board | left_moves | right_moves;
        attacks |= utility::south_one(attacks) | utility::north_one(attacks);

        return attacks;
    }
}
