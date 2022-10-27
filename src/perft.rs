use rayon::prelude::*;

use super::*;

pub fn perft(game: &mut Game, depth: u8, print: bool) -> u128 {
    let moves = generate_moves(game, MoveTypes::All);

    if depth == 1 {
        return moves.bulk_count(game) as u128;
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

pub fn debug_perft(game: &mut Game, depth: u8) -> u128 {
    let moves = generate_moves(game, MoveTypes::All);

    if depth == 1 {
        return moves.bulk_count(game) as u128;
    }

    moves.iter().map(|m| {
        let mut copy = *game;

        if make_move(&mut copy, &m) {
            let r = debug_perft(&mut copy, depth - 1);

            if copy.zobrist_hash != copy.make_zobrist_hash() {
                game.pretty_print();
                copy.pretty_print();
                println!("{}\t{:#0x}", m.to_uci(), copy.zobrist_hash);
                process::exit(0);
            }

            r
        }
        else { 0 }
    }).sum()
}