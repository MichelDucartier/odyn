use std::usize;

use const_for::const_for;
use ilog::IntLog;
use lazy_static::lazy_static;
use rand::Rng;

use crate::game::utility;

pub struct RookLookup {
    pub lookup: Vec<Vec<u64>>,
    pub magics: Vec<u64>,
}

pub fn hash_board(blockers: u64, magic: u64, remaining_bits: usize) -> usize {
    let mult = u128::from(blockers) * u128::from(magic);
    let mask = (1 << remaining_bits) - 1;

    ((mult >> (64 - remaining_bits)) & mask).try_into().unwrap()
}

const fn rook_rank_attack(rook_index: u32, occupancy: u8) -> u8 {
    let mut left_index = rook_index;
    while left_index > 0 && ((occupancy >> left_index) & 0x1 == 0) {
        left_index -= 1;
    }

    let mut right_index = rook_index;
    while right_index < 7 && ((occupancy >> right_index) & 0x1 == 0) {
        right_index += 1;
    }

    let high_ones = ((1_u16 << (right_index + 1)) - 1) as u8;
    let low_ones: u8 = (1 << (left_index)) - 1;

    ((high_ones - low_ones) & 0xff) as u8
}

const fn generate_rank_lookup() -> [[u8; 64]; 8] {
    let mut lookup_table = [[0; 64]; 8];

    const_for!(rook_index in 0..7 => {
        const_for!(occupancy in 0..64 => {
            let relevant_occupancy: u8 = (occupancy << 1) & 0x7e;

            lookup_table[rook_index][occupancy as usize] = rook_rank_attack(rook_index as u32, relevant_occupancy);
        });
    });

    lookup_table
}

fn generate_magic_rook_lt() -> RookLookup {
    const LOOKUP_SIZE: usize = 8192;
    let remaining_bits = LOOKUP_SIZE.log2();

    let mut lookup_table = vec![vec![0; LOOKUP_SIZE]; 64];
    let mut rook_magics = vec![0; 64];

    for rook_index in 0..64 {
        let lookup_index: usize = rook_index.try_into().unwrap();

        let (rook_row, rook_col) = utility::index_to_square(rook_index);
        let relevant_subset = utility::rook_rank_to_board(0x7e, 0x7e, rook_index);

        let blockers_list = utility::enumerate_subsets(relevant_subset);

        assert!(blockers_list.len() <= 4096);

        // Loop until a perfect magic number is found
        loop {
            lookup_table[lookup_index] = vec![0; LOOKUP_SIZE];
            let mut exists_collision = false;

            // Generate new magic number with few 0 bits
            let magic: u64 = fastrand::u64(..) | fastrand::u64(..) | fastrand::u64(..);

            for blockers in blockers_list.iter() {
                // If rook is in the blockers then it's impossible => skip it
                if ((1 << rook_index) & *blockers) != 0 {
                    continue;
                }

                let blockers_hash = hash_board(*blockers, magic, remaining_bits);

                // Compute rook attack
                let (row_block, col_block) = utility::board_to_rook_ranks(*blockers, rook_index);
                let row_attack = rook_rank_attack_fast(col_block, rook_row);
                let col_attack = rook_rank_attack_fast(row_block, rook_col);
                let attack = utility::rook_rank_to_board(col_attack, row_attack, rook_index);

                // If there is a hash collision, then it's not a perfect hashtable so we break and
                // start over
                if lookup_table[lookup_index][blockers_hash] != 0
                    && attack != lookup_table[lookup_index][blockers_hash]
                {
                    exists_collision = true;
                    break;
                }

                // Store attack in lookup table
                lookup_table[lookup_index][blockers_hash] = attack;
            }

            // No collision found => perfect magic number!
            if !exists_collision {
                let magic_index: usize = rook_index.try_into().unwrap();
                rook_magics[magic_index] = magic;
                break;
            }
        }
    }

    RookLookup {
        lookup: lookup_table,
        magics: rook_magics,
    }
}

fn rook_rank_attack_fast(occupancy: u8, rook_index: u32) -> u8 {
    let rook_index = usize::try_from(rook_index).unwrap();
    let occupancy = usize::try_from((occupancy >> 1) & 0b00111111).unwrap();
    RANK_ATTACK_LOOKUP[rook_index][occupancy]
}

const RANK_ATTACK_LOOKUP: [[u8; 64]; 8] = generate_rank_lookup();

lazy_static! {
    pub static ref ROOK_LOOKUP: RookLookup = generate_magic_rook_lt();
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_u8, game::magic::rook_rank_attack_fast};

    #[test]
    fn test_rook_rank_attack1() {
        let rook_index = 4;
        let occupancy = 0b01001001;

        let expected = 0b01111000;

        assert_eq_u8!(expected, rook_rank_attack_fast(occupancy, rook_index));
    }

    #[test]
    fn test_rook_rank_attack2() {
        let rook_index = 6;
        let occupancy = 0b00001000;

        let expected = 0b11111000;

        assert_eq_u8!(expected, rook_rank_attack_fast(occupancy, rook_index))
    }

    #[test]
    fn test_rook_rank_attack3() {
        let rook_index = 0;
        let occupancy = 0b00000110;

        let expected = 0b00000011;

        assert_eq_u8!(expected, rook_rank_attack_fast(occupancy, rook_index));
    }

    #[test]
    fn test_rook_rank_attack_full() {
        let rook_index = 0;
        let occupancy = 0;

        let expected = 0b11111111;
        assert_eq_u8!(expected, rook_rank_attack_fast(rook_index, occupancy));
    }
}
