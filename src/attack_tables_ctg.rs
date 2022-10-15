pub const WHITE_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(true);
pub const BLACK_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(false);
pub const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
pub const KING_ATTACKS: [u64; 64] = generate_king_attacks();

pub const ROOK_MASK: [u64; 64] = generate_rook_masks();
pub const ROOK_ATTACK_MASK: [u64; 64] = generate_rook_attack_masks();
pub const BISHOP_MASK: [u64; 64] = generate_bishop_masks();
pub const BISHOP_ATTACK_MASK: [u64; 64] = generate_bishop_attack_masks();

pub const SLIDING_ATTACKS: [u16; 107648] = generate_sliding_attacks().0;
pub const ROOK_OFFSETS: [usize; 64] = generate_sliding_attacks().1;
pub const BISHOP_OFFSETS: [usize; 64] = generate_sliding_attacks().2;

const fn generate_pawn_attacks(color: bool) -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if color {
                if file != 7 { result = result | base >> 7 as u64 }
                if file != 0 { result = result | base >> 9 as u64 }
                
            } else {
                if file != 0 { result = result | base << 7 as u64 }
                if file != 7 { result = result | base << 9 as u64 }
            }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_knight_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 1 && file < 7 { result = result | base >> 15 as u64 }
            if rank > 0 && file < 6 { result = result | base >> 6 as u64 }

            if rank < 7 && file < 6 { result = result | base << 10 as u64 }
            if rank < 6 && file < 7 { result = result | base << 17 as u64 }

            if rank > 1 && file > 0 { result = result | base >> 17 as u64 }
            if rank > 0 && file > 1 { result = result | base >> 10 as u64 }

            if rank < 7 && file > 1 { result = result | base << 6 as u64 }
            if rank < 6 && file > 0 { result = result | base << 15 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_king_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 0 { result = result | base >> 8 as u64 }
            if file > 0 { result = result | base >> 1 as u64 }
            if rank < 7 { result = result | base << 8 as u64 }
            if file < 7 { result = result | base << 1 as u64 }

            if file > 0 && rank > 0 { result = result | base >> 9 as u64 }
            if file < 7 && rank > 0 { result = result | base >> 7 as u64 }
            if file > 0 && rank < 7 { result = result | base << 7 as u64 }
            if file < 7 && rank < 7 { result = result | base << 9 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_rook_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = rook_mask(rank*8+file);
            file += 1;
            index += 1;
        }
        rank += 1;
    }
    mask
}

const fn generate_rook_attack_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = rook_attacks_on_the_fly(rank*8+file, 0);
            file += 1;
            index += 1;
        }
        rank += 1;
    }
    mask
}

const fn generate_bishop_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = bishop_mask(rank*8+file);

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    mask
}

const fn generate_bishop_attack_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = bishop_attacks_on_the_fly(rank*8+file, 0);
            file += 1;
            index += 1;
        }
        rank += 1;
    }
    mask
}

const fn rook_mask(square: u8) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;

    let mut file = square % 8;
    let mut offs = 1;
    while file > 1 {
        result |= base >> offs;
        offs += 1;
        file -= 1;
    }

    file = square % 8;
    offs = 1;
    while file < 6 {
        result |= base << offs;
        offs += 1;
        file += 1;
    }

    let mut rank = square / 8;
    let mut offs = 8;
    while rank > 1 {
        result |= base >> offs;
        offs += 8;
        rank -= 1;
    }

    rank = square / 8;
    offs = 8;
    while rank < 6 {
        result |= base << offs;
        offs += 8;
        rank += 1;
    }

    result
}

const fn bishop_mask(square: u8) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;
    let rank = square / 8;
    let file = square % 8;

    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    //Down-Right
    while t_rank < 6 && t_file < 6 {
        offs += 9;

        result |= base << offs;

        t_rank += 1;
        t_file += 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Down-Left
    while t_rank < 6 && t_file > 1 {
        offs += 7;

        result |= base << offs;

        t_rank += 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Left
    while t_rank > 1 && t_file > 1 {
        offs += 9;

        result |= base >> offs;

        t_rank -= 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Right
    while t_rank > 1 && t_file < 6 {
        offs += 7;

        result |= base >> offs;

        t_rank -= 1;
        t_file += 1;
    }

    result
}

//returns tuple: (Attack_table, Rook_offsets, bishop_offsets)
const fn generate_sliding_attacks() -> ([u16; 107648], [usize; 64], [usize; 64]) {
    let mut attacks: [u16; 107648] = [0; 107648];
    let mut rook_offsets: [usize; 64] = [0; 64];
    let mut bishop_offsets: [usize; 64] = [0; 64];

    let mut current_offset: u32 = 0;

    //ROOKS
    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let square = rank * 8 + file;
            rook_offsets[square as usize] = current_offset as usize;
            let number_of_occupancies = (2 as u16).pow(ROOK_MASK[square as usize].count_ones()) as u32;

            let mut occ_index: u32 = 0;
            while occ_index < number_of_occupancies {
                let occ = set_occupancy(occ_index, ROOK_MASK[square as usize]);
                attacks[(current_offset + occ_index as u32) as usize] = const_pext(rook_attacks_on_the_fly(square, occ), ROOK_ATTACK_MASK[square as usize]);
                occ_index += 1;
            }
            
            current_offset += number_of_occupancies as u32;
            
            file += 1;
        }
    rank += 1;
    }
    //OFFSET HER: 104600 i believe
    //Bishops
    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let square = rank * 8 + file;
            bishop_offsets[square as usize] = current_offset as usize;
            let number_of_occupancies = (2 as u16).pow(BISHOP_MASK[square as usize].count_ones()) as u32;

            let mut occ_index: u32 = 0;
            while occ_index < number_of_occupancies {
                let occ = set_occupancy(occ_index, BISHOP_MASK[square as usize]);
                attacks[(current_offset + occ_index as u32) as usize] = const_pext(bishop_attacks_on_the_fly(square, occ), BISHOP_ATTACK_MASK[square as usize]);
                occ_index += 1;
            }
            
            current_offset += number_of_occupancies as u32;
            
            file += 1;
        }
    rank += 1;
    }

    (attacks, rook_offsets, bishop_offsets)
}

const fn const_pext(bits: u64, mask: u64) -> u16 {
    let mut mask = mask as i128;
    let mut res: u16 = 0;

    let mut bb = 1;
    while mask != 0  {
        if bits as i128 & mask & -mask != 0 {
            res |= bb;
        }
        mask &= mask - 1;
        bb += bb
    }
    res as u16
}

const fn rook_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;

    let mut file = square % 8;
    let mut offs = 0;
    //LEFT
    while file > 0 {
        offs += 1;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        file -= 1;
    }

    file = square % 8;
    offs = 0;
    //RIGHT
    while file < 7 {
        offs += 1;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        file += 1;
    }

    let mut rank = square / 8;
    let mut offs = 0;
    //UP
    while rank > 0 {
        offs += 8;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        rank -= 1;
    }

    rank = square / 8;
    offs = 0;
    //DOWN
    while rank < 7 {
        offs += 8;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        rank += 1;
    }

    result
}

const fn bishop_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;
    let rank = square / 8;
    let file = square % 8;

    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    //Down-Right
    while t_rank < 7 && t_file < 7 {
        offs += 9;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        t_rank += 1;
        t_file += 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Down-Left
    while t_rank < 7 && t_file > 0 {
        offs += 7;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        t_rank += 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Left
    while t_rank > 0 && t_file > 0 {
        offs += 9;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        t_rank -= 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Right
    while t_rank > 0 && t_file < 7 {
        offs += 7;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        t_rank -= 1;
        t_file += 1;
    }

    result
}

const fn set_occupancy(index: u32, attack_mask: u64) -> u64 {
    let mut occ = 0;

    let mut mask = attack_mask;

    let bits_in_mask = attack_mask.count_ones();
    let mut count: u16 = 0;
    let mut square;
    while count < bits_in_mask as u16 {
        //least significant 1 bit
        square = mask.trailing_zeros();

        //unset the bit
        mask ^= 1 << square;

        if (index & (1 << count)) != 0 {
            occ |= 1 << (square);
        }

        count += 1;
    }
    occ
}