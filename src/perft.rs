use rayon::prelude::*;

use super::*;

pub fn perft(game: &mut Game, depth: u8, print: bool) -> u128 {
    let moves = generate_moves(game);

    if depth == 1 {
        return moves.len() as u128;
    }

    moves.values().par_iter().map(|m| {
        let mut copy = game.clone();

        make_move(&mut copy, &m);
        let r = perft(&mut copy, depth - 1, false);

        if print {
            println!("{}{}: {}", SQUARE_STRINGS[m.from_square() as usize], SQUARE_STRINGS[m.to_square() as usize], r)
        }

        r
    }).sum()
}