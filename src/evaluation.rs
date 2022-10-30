use super::*;

/*const FILE_MASKS: [u64; 64] = generate_file_masks();
const RANK_MASKS: [u64; 64] = generate_rank_masks();*/
/*const ISOLATED_MASKS: [u64; 64] = generate_isolated_masks();
const PASSED_PAWN_MASKS: [u64; 64] = generate_passed_pawn_masks();*/


pub fn evaluate(game: &Game) -> i32 {
    let mut score: i32 = 0;

    for bb in 0..12 {
        let mut board = game.bitboards[bb];
        while !board.is_empty() {
            let square = board.extract_bit();
            score += MATERIAL_WEIGHTS[bb];
            score += match bb {
                0  => PAWN_SCORES[square as usize],
                1  => KNIGHT_SCORES[square as usize],
                2  => BISHOP_SCORES[square as usize],
                3  => ROOK_SCORES[square as usize],
                //No queen values,
                5  => KING_SCORES[square as usize],

                6  => -PAWN_SCORES[MIRRORED[square as usize]],
                7  => -KNIGHT_SCORES[MIRRORED[square as usize]],
                8  => -BISHOP_SCORES[MIRRORED[square as usize]],
                9  => -ROOK_SCORES[MIRRORED[square as usize]],
                //No queen values,
                11 => -KING_SCORES[MIRRORED[square as usize]],

                _ => 0
            };
        }
    }

    if game.active_player == Color::White { score } else { -score }
}
*/
const fn generate_file_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;
            

            let mut i = 0;
            while i < 8 {
                mask = mask | ((1 << f) << i*8);
                i += 1;
            }

            masks[r * 8 + f] = mask;

            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_rank_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;
            
            let mut i = 0;
            while i < 8 {
                mask = mask | (1 << i) << 8*f;
                i += 1;
            }

            masks[r * 8 + f] = mask;

            f += 1;
        }
        r += 1;
    }

    masks
}
*/
/*const fn generate_isolated_masks() -> [u64; 64] {

}

const fn generate_passed_pawn_masks() -> [u64; 64] {

}*/