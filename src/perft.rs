use rayon::prelude::*;

use super::*;

pub fn perft(game: &mut Game, depth: u8, print: bool) -> u128 {
    let moves = generate_moves(game, MoveTypes::All);

    if depth == 1 {
        return moves.legal_values(game).len() as u128;
    }

    if depth > 2 {
        moves.par_iter().map(|m| {
            let mut copy = *game;
    
            if make_move(&mut copy, &m) {
                let r = perft(&mut copy, depth - 1, false);
    
                if print {
                    println!("{}{}: {}", SQUARE_STRINGS[m.from_square() as usize], SQUARE_STRINGS[m.to_square() as usize], r)
                }
    
                r
            }
            else { 0 }
        }).sum()
    }
    else {
        moves.iter().map(|m| {
            let mut copy = *game;
    
            if make_move(&mut copy, &m) {
                let r = perft(&mut copy, depth - 1, false);
    
                if print {
                    println!("{}{}: {}", SQUARE_STRINGS[m.from_square() as usize], SQUARE_STRINGS[m.to_square() as usize], r)
                }
    
                r
            }
            else { 0 }
        }).sum()
    }
}