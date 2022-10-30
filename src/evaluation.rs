use super::*;

pub const MATERIAL_WEIGHTS: [i32; 12] = [100, 300, 350, 500, 1000, 10000, -100, -300, -350, -500, -1000, -10000];

// pawn positional score
pub const PAWN_SCORES: [i32; 64] = 
[
    90,  90,  90,  90,  90,  90,  90,  90,
    30,  30,  30,  40,  40,  30,  30,  30,
    20,  20,  20,  30,  30,  30,  20,  20,
    10,  10,  10,  20,  20,  10,  10,  10,
     5,   5,  10,  20,  20,   5,   5,   5,
     0,   0,   0,   5,   5,   0,   0,   0,
     0,   0,   0, -10, -10,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0
];

// knight positional score
pub const KNIGHT_SCORES: [i32; 64] = 
[
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,  10,  10,   0,   0,  -5,
    -5,   5,  20,  20,  20,  20,   5,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,   5,  20,  10,  10,  20,   5,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5, -10,   0,   0,   0,   0, -10,  -5
];

// bishop positional score
pub const BISHOP_SCORES: [i32; 64] = 
[
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   0,  10,  10,   0,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,  10,   0,   0,   0,   0,  10,   0,
     0,  30,   0,   0,   0,   0,  30,   0,
     0,   0, -10,   0,   0, -10,   0,   0

];

// rook positional score
pub const ROOK_SCORES: [i32; 64] = 
[
    50,  50,  50,  50,  50,  50,  50,  50,
    50,  50,  50,  50,  50,  50,  50,  50,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,   0,  20,  20,   0,   0,   0

];

// king positional score
pub const KING_SCORES: [i32; 64] = 
[
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   5,   5,   5,   5,   0,   0,
     0,   5,   5,  10,  10,   5,   5,   0,
     0,   5,  10,  20,  20,  10,   5,   0,
     0,   5,  10,  20,  20,  10,   5,   0,
     0,   0,   5,  10,  10,   5,   0,   0,
     0,   5,   5,  -5,  -5,   0,   5,   0,
     0,   0,   5,   0, -15,   0,  10,   0
];

// mirror positional score tables for opposite side
pub const MIRRORED: [usize; 64] = 
[
	56, 57, 58, 59, 60, 61, 62, 63,
	48, 49, 50, 51, 52, 53, 54, 55,
	40, 41, 42, 43, 44, 45, 46, 47,
	32, 33, 34, 35, 36, 37, 38, 39,
	24, 25, 26, 27, 28, 29, 30, 31,
	16, 17, 18, 19, 20, 21, 22, 23,
	8,  9,  10, 11, 12, 13, 14, 15,
	0,  1,  2,  3,  4,  5,  6,  7
];

///[attacker][victim]
pub const MVV_LVA: [[i32; 12]; 12] = [
    [105, 205, 305, 405, 505, 605,  105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604,  104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603,  103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602,  102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601,  101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600,  100, 200, 300, 400, 500, 600],
    
    [105, 205, 305, 405, 505, 605,  105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604,  104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603,  103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602,  102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601,  101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600,  100, 200, 300, 400, 500, 600],
];


const FILE_MASKS: [u64; 64] = generate_file_masks();
const RANK_MASKS: [u64; 64] = generate_rank_masks();
const ISOLATED_MASKS: [u64; 64] = generate_isolated_pawn_masks();

const WHITE_PASSED_PAWN_MASKS: [u64; 64] = generate_white_passed_pawn_masks();
const BLACK_PASSED_PAWN_MASKS: [u64; 64] = generate_black_passed_pawn_masks();

const STACKED_PAWN_PENALTY: i32 = -10;
const ISOLATED_PAWN_PENALTY: i32 = -10;
const PASSED_WHITE_PAWN_BONUS: [i32; 8] = [ 0, 10, 30, 50, 75, 100, 150, 200 ];
const PASSED_BLACK_PAWN_BONUS: [i32; 8] = [ 200, 150, 100, 75, 50, 30, 10, 0 ]; 
const SEMI_OPEN_FILE_SCORE: i32 = 10;
const OPEN_FILE_SCORE: i32 = 15;
const PROTECTED_KING_BONUS: i32 = 5;

/*const KNIGHT_UNIT: u32 = 4;
const BISHOP_UNIT: u32 = 6;
const ROOK_UNIT: u32 = 7;
const QUEEN_UNIT: u32 = 13;

const KNIGHT_MOB: i32 = 4;
const BISHOP_MOB: i32 = 5;
const ROOK_MOB: i32 = 3;
const QUEEN_MOB: f32 = 1.5;*/


pub fn evaluate(game: &Game) -> i32 {
    let mut score: i32 = 0;

    let mut stacked_pawns;

    for bb in 0..12 {
        let mut board = game.bitboards[bb];
        while !board.is_empty() {
            let square = board.extract_bit();
            score += MATERIAL_WEIGHTS[bb];
            match bb {
                //White pawns
                0  => {
                    score += PAWN_SCORES[square as usize];

                    //Stacked pawn penalty
                    stacked_pawns = game.get_piece_bitboard(Piece::WhitePawn)
                                        .and(Bitboard::from_u64(FILE_MASKS[square as usize]))
                                        .pop_count();
                    if stacked_pawns > 1 {
                        score += stacked_pawns as i32 * STACKED_PAWN_PENALTY;
                    }

                    //Isolated pawn penalty
                    if game.get_piece_bitboard(Piece::WhitePawn)
                            .and(Bitboard::from_u64(ISOLATED_MASKS[square as usize]))
                            .is_empty() {
                        score += ISOLATED_PAWN_PENALTY;
                    }

                    //Passed pawn penalty
                    if game.get_piece_bitboard(Piece::BlackPawn)
                           .and(Bitboard::from_u64(WHITE_PASSED_PAWN_MASKS[square as usize]))
                           .is_empty() {
                        score += PASSED_WHITE_PAWN_BONUS[LOOKUP_RANK[square as usize]];
                    }
                },
                //White knight
                1  => {
                    score += KNIGHT_SCORES[square as usize];

                    //Mobility
                    //score += (get_knight_attack_table(square).pop_count() - KNIGHT_UNIT) as i32 * KNIGHT_MOB;
                    score += get_knight_attack_table(square).pop_count() as i32;
                },
                //White bishops
                2  => {
                    score += BISHOP_SCORES[square as usize];

                    //Mobility
                    //score += (get_bishop_attack_table(square, game.all_occupancies).pop_count() - BISHOP_UNIT) as i32 * BISHOP_MOB;
                    score += (get_bishop_attack_table(square, game.all_occupancies).pop_count()) as i32;

                },
                //White Rooks
                3  => {
                    score += ROOK_SCORES[square as usize];

                    //Semi open file bonus
                    if game.get_piece_bitboard(Piece::WhitePawn)
                           .and(Bitboard::from_u64(FILE_MASKS[square as usize]))
                           .is_empty() {
                        score += SEMI_OPEN_FILE_SCORE;
                    }

                    //Open file bonus
                    if (game.get_piece_bitboard(Piece::WhitePawn)
                            .or(game.get_piece_bitboard(Piece::BlackPawn)))
                                .and(Bitboard::from_u64(FILE_MASKS[square as usize])).is_empty() {
                        score += OPEN_FILE_SCORE;
                    }

                    //Mobility
                    //score += (get_rook_attack_table(square, game.all_occupancies).pop_count() - ROOK_UNIT) as i32 * ROOK_MOB;
                    score += get_rook_attack_table(square, game.all_occupancies).pop_count() as i32;
                },
                //White queen
                4 => {
                    //Mobility
                    //score += ((get_queen_attack_table(square, game.all_occupancies).pop_count() - QUEEN_UNIT) as f32 * QUEEN_MOB) as i32;
                    score += get_queen_attack_table(square, game.all_occupancies).pop_count() as i32;
                },
                //White king
                5  => {
                    score += KING_SCORES[square as usize];

                    //Semi open file penalty
                    if game.get_piece_bitboard(Piece::WhitePawn)
                           .and(Bitboard::from_u64(FILE_MASKS[square as usize]))
                           .is_empty() {
                        score -= SEMI_OPEN_FILE_SCORE;
                    }

                    //Open file penalty
                    if (game.get_piece_bitboard(Piece::WhitePawn)
                            .or(game.get_piece_bitboard(Piece::BlackPawn)))
                                .and(Bitboard::from_u64(FILE_MASKS[square as usize])).is_empty() {
                        score -= OPEN_FILE_SCORE;
                    }

                    //King safety
                    score += get_king_attack_table(square).and(game.white_occupancies).pop_count() as i32 * PROTECTED_KING_BONUS;
                },
                //Black pawns
                6  => {
                    score -= PAWN_SCORES[MIRRORED[square as usize]];
                    
                    //Stacked pawn penalty
                    stacked_pawns = game.get_piece_bitboard(Piece::BlackPawn)
                                        .and(Bitboard::from_u64(FILE_MASKS[square as usize]))
                                        .pop_count();
                    if stacked_pawns > 1 {
                        score -= stacked_pawns as i32 * STACKED_PAWN_PENALTY;
                    }

                    //Isolated pawn penalty
                    if game.get_piece_bitboard(Piece::BlackPawn)
                           .and(Bitboard::from_u64(ISOLATED_MASKS[square as usize]))
                           .is_empty() {
                        score -= ISOLATED_PAWN_PENALTY;
                    }

                    //Passed pawn penalty
                    if game.get_piece_bitboard(Piece::WhitePawn)
                           .and(Bitboard::from_u64(BLACK_PASSED_PAWN_MASKS[square as usize]))
                           .is_empty() {
                        score -= PASSED_BLACK_PAWN_BONUS[LOOKUP_RANK[square as usize]];
                    }
                },
                //Black knight
                7  => {
                    score -= KNIGHT_SCORES[MIRRORED[square as usize]];

                    //Mobility
                    //score -= (get_knight_attack_table(square).pop_count() - KNIGHT_UNIT) as i32 * KNIGHT_MOB;
                    score -= get_knight_attack_table(square).pop_count() as i32;
                },
                //Black bishop
                8  => {
                    score -= BISHOP_SCORES[MIRRORED[square as usize]];

                    //Mobility
                    //score -= (get_bishop_attack_table(square, game.all_occupancies).pop_count() - BISHOP_UNIT) as i32 * BISHOP_MOB;
                    score -= get_bishop_attack_table(square, game.all_occupancies).pop_count() as i32;
                },
                //Black rooks
                9  => {
                    score -= ROOK_SCORES[MIRRORED[square as usize]];

                    //Semi open file bonus
                    if game.get_piece_bitboard(Piece::BlackPawn)
                           .and(Bitboard::from_u64(FILE_MASKS[square as usize]))
                           .is_empty() {
                        score -= SEMI_OPEN_FILE_SCORE;
                    }

                    //Open file bonus
                    if (game.get_piece_bitboard(Piece::BlackPawn)
                            .or(game.get_piece_bitboard(Piece::WhitePawn)))
                                .and(Bitboard::from_u64(FILE_MASKS[square as usize])).is_empty() {
                        score -= OPEN_FILE_SCORE;
                    }

                    //Mobility
                    //score -= (get_rook_attack_table(square, game.all_occupancies).pop_count() - ROOK_UNIT) as i32 * ROOK_MOB;
                    score -= get_rook_attack_table(square, game.all_occupancies).pop_count() as i32;
                },
                //Black queen
                10 => {
                    //Mobility
                    //score -= ((get_queen_attack_table(square, game.all_occupancies).pop_count() - QUEEN_UNIT) as f32 * QUEEN_MOB) as i32;
                    score -= get_queen_attack_table(square, game.all_occupancies).pop_count() as i32;
                }
                //Black king
                11 => {
                    score -= KING_SCORES[MIRRORED[square as usize]];

                    //Semi open file penalty
                    if game.get_piece_bitboard(Piece::BlackPawn)
                           .and(Bitboard::from_u64(FILE_MASKS[square as usize]))
                           .is_empty() {
                        score += SEMI_OPEN_FILE_SCORE;
                    }

                    //Open file penalty
                    if (game.get_piece_bitboard(Piece::BlackPawn)
                            .or(game.get_piece_bitboard(Piece::WhitePawn)))
                                .and(Bitboard::from_u64(FILE_MASKS[square as usize])).is_empty() {
                        score += OPEN_FILE_SCORE;
                    }

                    //King safety
                    score -= get_king_attack_table(square).and(game.black_occupancies).pop_count() as i32 * PROTECTED_KING_BONUS;
                },
                _ => unreachable!()
            };
        }
    }

    if game.active_player == Color::White { score } else { -score }
}

const fn generate_file_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;
            

            let mut i = 0;
            while i < 8 {
                mask |= (1 << f) << i*8;
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
                mask |= (1 << i) << 8*f;
                i += 1;
            }

            masks[r * 8 + f] = mask;

            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_isolated_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;

            if f > 0 {
                mask |= FILE_MASKS[r*8+f - 1]
            }
            if f < 7 {
                mask |= FILE_MASKS[r*8+f + 1]
            }

            masks[r*8+f] = mask;
            
            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_white_passed_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;

            mask |= FILE_MASKS[r*8+f];

            if f > 0 {
                mask |= FILE_MASKS[r*8+f - 1]
            }
            if f < 7 {
                mask |= FILE_MASKS[r*8+f + 1]
            }
            //For all ranks lower
            let mut rr = 7;
            while rr > r {
                mask ^= RANK_MASKS[rr*8] & mask;
                rr -= 1;
            }
            masks[r*8+f] = mask;
            
            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_black_passed_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;

            mask |= FILE_MASKS[r*8+f];

            if f > 0 {
                mask |= FILE_MASKS[r*8+f - 1]
            }
            if f < 7 {
                mask |= FILE_MASKS[r*8+f + 1]
            }
            //For all ranks lower
            let mut rr = 0;
            while rr < r {
                mask ^= RANK_MASKS[rr*8] & mask;
                rr += 1;
            }
            masks[r*8+f] = mask;
            
            f += 1;
        }
        r += 1;
    }

    masks
}

#[cfg(test)]
mod eval_tests {
    use crate::*;

    #[test]
    pub fn eval () {
        let game = Game::new_from_fen("6k1/ppppprbp/8/8/8/8/PPPPPRBP/6K1 w - - 0 1 ").unwrap();
        game.pretty_print();
        println!("{}", evaluate(&game));
    }
}