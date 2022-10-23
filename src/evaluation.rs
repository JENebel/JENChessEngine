use super::*;

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