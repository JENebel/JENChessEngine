use super::*;

const MAX_PLY: usize = 64;
const FULL_DEPTH_MOVES: u8 = 4;
const REDUCTION_LIMIT: u8 = 3;


pub fn search(game: &mut Game, depth: u8) -> SearchResult {
    let mut envir = SearchEnv::new();

    let mut score = 0;

    let mut alpha = -50000;
    let mut beta  =  50000;

    let mut current_depth = 1;

    while current_depth <= depth {
        envir.follow_pv = true;
        score = negamax(game, current_depth, alpha, beta, &mut envir);

        //Narrowing aspiration window
        if score <= alpha || score >= beta {
            alpha = -50000;
            beta  =  50000;
            
            continue;
        }
        
        alpha = score - 50;
        beta  = score + 50;

        //println!("{}", envir.pv_lengths[0]);
        print!("info score cp {} depth {} nodes {} pv ", score, current_depth, envir.nodes);
        for i in 0..envir.pv_lengths[0] {
            print!("{} ", envir.pv_table[0][i].to_uci());
        }
        print!("\n");

        current_depth += 1;
    }

    SearchResult::new(envir.pv_table[0][0], envir.nodes, score, depth)
}

fn enable_pv_scoring(moves: &MoveList, envir: &mut SearchEnv) {
    envir.follow_pv = false;

    for i in 0..moves.len() {
        if envir.pv_table[0][envir.ply as usize] == *moves.get(i) {
            envir.score_pv = true;
            envir.follow_pv = true;
        }
    }
}

fn negamax(game: &mut Game, depth: u8, alpha: i32, beta: i32, envir: &mut SearchEnv) -> i32 {
    envir.pv_lengths[envir.ply as usize] = envir.ply as usize;
    
    if depth == 0 {
        //return game.evaluate()
        return quiescence(game, alpha, beta, envir);
    }

    //Dont't go on if reached max ply
    if envir.ply >= MAX_PLY as u8 {
        return game.evaluate();
    }

    let mut moves_searched = 0;
    
    envir.nodes += 1;

    let in_check = game.is_in_check(game.active_player);

    let n_depth = if in_check { depth + 1 } else { depth };

    let mut temp_alpha = alpha;

    //Null move pruning
    if n_depth >= 3 && !in_check && envir.ply > 0 {
        let mut copy = game.clone();
        //Give opponent extra move
        copy.active_player = opposite_color(copy.active_player);

        copy.enpassant_square = Square::None;

        //..., Depth - 1 - R (with R = 2), ...
        let score = -negamax(&mut copy, n_depth - 1 - 2, -beta, -beta + 1, envir);

        //Cut-off
        if score >= beta {
            return beta
        }
    }

    let mut moves = game.generate_moves();

    if envir.follow_pv {
        enable_pv_scoring(&moves, envir)
    }

    moves.sort_moves(game, envir);

    //Mate & Draw
    if moves.len() == 0 {
        if game.is_in_check(game.active_player) {
            return -49000 + envir.ply as i32;
        }
        else {
            return 0;
        }
    }

    for i in 0..moves.len() {
        let m = moves.get(i);
        
        let mut copy = game.clone();
        copy.make_move(m);

        envir.ply += 1;

        let mut score;
        if moves_searched == 0 {
            //Full PV Search
            score = -negamax(&mut copy, n_depth - 1, -beta, -alpha, envir);
        } else {
            //Regular search with LMR

            score = if  moves_searched >= FULL_DEPTH_MOVES && 
                        depth >= REDUCTION_LIMIT &&
                        !in_check &&
                        !m.is_capture() &&
                        !m.promotion() != Piece::None as u8 {
                //Reduced search
                -negamax(&mut copy, n_depth - 2, -temp_alpha - 1, -temp_alpha, envir)

            } else {
                //Ensure a full search
                temp_alpha + 1
            };

            if score > temp_alpha {
                //LMR
                score = -negamax(&mut copy, n_depth - 1, -temp_alpha - 1, -temp_alpha, envir);

                //Check bounds
                if score > temp_alpha && score < beta {
                    //Full search on failure
                    score = -negamax(&mut copy, n_depth - 1, -beta, -temp_alpha, envir);
                }
            }
        }

        envir.ply -= 1;

        moves_searched += 1;

        if score >= beta {
            //Update killer moves
            if !m.is_capture() {
                envir.killer_moves[1][envir.ply as usize] = envir.killer_moves[0][envir.ply as usize];
                envir.killer_moves[0][envir.ply as usize] = Some(*m);
            }

            return beta;
        }

        if score > temp_alpha {
            //Update history move
            if !m.is_capture() {
                envir.history_moves[m.piece() as usize][m.to_square() as usize] += depth as i32
            }

            temp_alpha = score;

            //Insert PV node
            envir.insert_pv_node(*m);
        }
    }
    
    temp_alpha
}

fn quiescence(game: &mut Game, alpha: i32, beta: i32, envir: &mut SearchEnv) -> i32 {
    envir.nodes += 1;

    let eval = game.evaluate();

    if eval >= beta {
        return beta
    }

    let mut temp_alpha = alpha;

    if eval > temp_alpha {
        temp_alpha = eval
    }

    let mut moves = game.generate_moves();
    moves.sort_moves(game, envir);

    for i in 0..moves.len() {
        let m = moves.get(i);

        if !m.is_capture() {
            continue;
        }

        let mut copy = game.clone();
        copy.make_move(m);
        
        envir.ply += 1;

        let score = -quiescence(&mut copy, -beta, -temp_alpha, envir);

        envir.ply -= 1;

        if score >= beta {
            return beta;
        }

        if score > temp_alpha {
            temp_alpha = score;
        }
    }

    temp_alpha
}

pub fn score_move(game: &Game, cmove: Move, envir: &mut SearchEnv) -> i32 {
    if envir.score_pv {
        if envir.pv_table[0][envir.ply as usize] == cmove {
            envir.score_pv = false;
            return 20000;
        }
    }

    let to_sq = cmove.to_square();
    //Captures
    if cmove.is_capture() {
        let start;
        let end;
        let mut taken = 0;
        if game.active_player == Color::White {
            start = Piece::BlackPawn as usize;
            end = Piece::BlackKing as usize;
        }
        else {
            start = Piece::WhitePawn as usize;
            end = Piece::WhiteKing as usize;
        }

        for bb in start..end {
            if game.bitboards[bb].get_bit(to_sq) {
                taken = bb;
                break;
            }
        }

        MVV_LVA[cmove.piece() as usize][taken as usize] + 10000
    }

    //Quiet moves
    else {
        if envir.killer_moves[0][envir.ply as usize] == Some(cmove) {
            9000
        } else if envir.killer_moves[1][envir.ply as usize] == Some(cmove) {
            8000
        }
        else {
            envir.history_moves[cmove.piece() as usize][to_sq as usize]
        }
    }
}

pub struct SearchEnv {
    pub nodes: u32,
    pub ply: u8,
    pub killer_moves: [[Option<Move>; MAX_PLY]; 2],
    pub history_moves: [[i32; 64]; 12],
    pub pv_lengths: [usize; MAX_PLY],
    pub pv_table: [[Move; MAX_PLY]; MAX_PLY],
    pub follow_pv: bool,
    pub score_pv: bool,
}

impl SearchEnv {
    pub fn new() -> Self {
        Self {
            nodes: 0,
            ply: 0,
            killer_moves: [[None; 64]; 2],
            history_moves: [[0 as i32; 64]; 12],
            pv_lengths: [0; 64],
            pv_table: [[NULL_MOVE; 64]; 64],
            follow_pv: false,
            score_pv: false,
        }
    }

    pub fn insert_pv_node(&mut self, cmove: Move) {
        let ply = self.ply as usize;

        self.pv_table[ply][ply] = cmove;
        
        for next_ply in (ply + 1)..self.pv_lengths[ply + 1] {
            self.pv_table[ply][next_ply] = self.pv_table[ply + 1][next_ply];
        }

        self.pv_lengths[ply] = self.pv_lengths[ply + 1];
    }
}


#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    pub fn mvv_lva_test() {
        sbench();
        
        //let game = Game::new_from_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9").unwrap();
        //println!("{}", game.evaluate())
    }

    #[test]
    pub fn negamax() {
        let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        let start = SystemTime::now();
        let depth = 6;
        let result = search(&mut game, depth);
        let duration = start.elapsed().unwrap();
        println!(" Found best move: {} for depth {}. Visited: {} nodes in {}ms", result.best_move.to_uci(), depth, result.nodes_visited, duration.as_millis());
    }
}