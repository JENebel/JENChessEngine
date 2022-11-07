use rayon::prelude::*;

use super::*;

pub fn perft(pos: &mut Position, depth: u8, rep_table: &mut RepetitionTable, print: bool) -> u128 {
    let mut moves = MoveGenerator::initialize(pos, MoveTypes::All);

    if depth == 0 {
        return 1
    }

    let mut count = 0;

    loop {
        let m = moves.get_next_move(false);
        if m == NULL_MOVE { break; }

        let mut copy = *pos;

        if copy.make_move(&m, rep_table) {
            let r = perft(&mut copy, depth - 1, rep_table, false);
            count += r;

            if print {
                println!("{}{}: {}", SQUARE_STRINGS[m.from_square() as usize], SQUARE_STRINGS[m.to_square() as usize], r)
            }

            rep_table.move_back();
        }
    }

    count


    //todo!()
    /*
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
    }*/
}
