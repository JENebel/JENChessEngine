//use rand::{Rng};

use rand::Rng;

use super::*;

pub const MAX_PLY: usize = 64;
pub const FULL_DEPTH_MOVES: u8 = 4;
pub const REDUCTION_LIMIT: u8 = 3;
pub const MATE_VALUE: i32 = 49000; //Score returned on mate
pub const MATE_BOUND: i32 = 48000; //Lower bound for mating score
const INFINITY: i32 = 50000;

const INPUT_POLL_INTERVAL: u64 = 16383; //Node interval to check if search aborted

pub struct SearchResult {
    pub best_move: Move,
    pub nodes_visited: u64,
    pub score: i32,
    pub depth: u8,
    pub reached_max_ply: bool,
    pub tt_hits: u32
}

impl SearchResult {
    pub fn new(cmove: Move, nodes: u64, score: i32, depth: u8, reached_max_ply: bool, tt_hits: u32) -> Self {
        Self { best_move: cmove, nodes_visited: nodes, score: score, depth: depth, reached_max_ply: reached_max_ply, tt_hits: tt_hits }
    }
}

pub struct Searcher<'a> {
    pub pos: Position,
    pub nodes: u64,
    pub tt_hits: u32,
    pub ply: u8,
    pub stopping: bool,
    io_receiver: &'a IoWrapper,
    pub start_time: SystemTime,
    max_time: i64,
    depth_reached: u8, 

    //Data structures. Shared by threads in future
    pub killer_moves: &'a mut [[Move; MAX_PLY]; 2],
    pub history_moves: &'a mut [[u8; 64]; 12],
    pub repetition_table: &'a mut RepetitionTable,
    pub tt: &'a mut TranspositionTable,
}

impl <'a>Searcher<'a> {
    ///Returns a random legal move
    pub fn find_random_move(pos: &mut Position) {
        let moves = MoveGenerator::all_moves(pos);
        let rand = rand::thread_rng().gen_range(0..moves.len());
        print!("bestmove {}\n", moves[rand]);
    }

    ///Initializes a new searcher
    pub fn new(pos: Position, tt: &mut TranspositionTable, max_time: Option<i64>, io_receiver: &'a IoWrapper) -> Self {
        Self {
            pos,
            nodes: 0,
            ply: 0,
            stopping: false,
            io_receiver,
            start_time: SystemTime::now(),
            max_time: max_time.unwrap_or(-1),
            tt_hits: 0,
            depth_reached: 1,

            killer_moves: &mut [[NULL_MOVE; MAX_PLY]; 2],
            history_moves: &mut [[0; 64]; 12],
            repetition_table: &mut RepetitionTable::new(),
            tt
        }
    }

    ///Checks if the search has been aborted
    fn poll_input(&mut self) {
        if (self.max_time != -1 && self.start_time.elapsed().unwrap().as_millis() as i64 >= self.max_time) || self.io_receiver.try_read_line().is_some() {
            self.stopping = true;
            return;
        }
    }

    ///Start a search, max_time = -1 for no limit
    pub fn search(&mut self, pos: &mut Position, depth: i8, tt: &mut TranspositionTable) -> SearchResult {
        let mut score = 0;

        let mut alpha = -INFINITY;
        let mut beta  =  INFINITY;

        let max_depth = if depth == -1 { MAX_PLY as u8 } else { depth as u8 };

        while self.depth_reached <= max_depth as u8 {
            score = self.negamax(self.depth_reached, alpha, beta);

            if self.stopping { break }

            //Narrowing aspiration window
            if score <= alpha || score >= beta {
                alpha = -INFINITY;
                beta  =  INFINITY;

                self.depth_reached += 1;

                continue;
            }

            alpha = score - 50;
            beta  = score + 50;

            if score >= -MATE_VALUE && score < -MATE_BOUND {
                print!("info score mate {} depth {} nodes {} time {} pv ", -(score + MATE_VALUE) / 2 - 1, self.depth_reached, self.nodes, self.start_time.elapsed().unwrap().as_millis());
            }
            else if score <= MATE_VALUE && score > MATE_BOUND {
                print!("info score mate {} depth {} nodes {} time {} pv ", (MATE_VALUE - score) / 2 + 1, self.depth_reached, self.nodes, self.start_time.elapsed().unwrap().as_millis());
            }
            else {
                print!("info score cp {} depth {} nodes {} time {} pv", score, self.depth_reached, self.nodes, self.start_time.elapsed().unwrap().as_millis());
            }

            self.print_pv_line();
            print!("\n");
            
            self.depth_reached += 1;
        }

        let best_move = tt.probe_best_move(pos.zobrist_hash);

        print!("bestmove {}\n", best_move);

        SearchResult::new(best_move, self.nodes, score, self.depth_reached - 1, !self.stopping, self.tt_hits)
    }

    fn print_pv_line(&self) -> Vec<Move>{
        let old_pos = self.pos;
        let mut r_t = RepetitionTable::new();

        let mut line = Vec::new();
        while line.len() < self.depth_reached as usize - 1 {
            let entry = self.tt.probe_pv_move(self.pos.zobrist_hash);
            match entry {
                Some(pv) => { 
                    line.push(pv);
                    self.pos.make_move(&pv, &mut r_t);
                },
                _ => break
            }
        }

        self.pos = old_pos;

        line
    }

    #[inline]
    fn negamax(&mut self, depth: u8, alpha: i32, beta: i32) -> i32 {
        
        let is_pv_node = (beta - alpha) > 1;

        let mut score;
        if self.ply != 0 && !is_pv_node {
            score = self.tt.probe_score(self.pos.zobrist_hash, depth, alpha, beta, self.ply);
            if score != UNKNOWN_SCORE {
                self.tt_hits += 1;
                return score;
            }
        }

        if self.ply > 0 && self.repetition_table.is_now_in_threefold_repetition() {
            return 0;
        }

        //Dont't go on if reached max ply
        if self.ply >= MAX_PLY as u8 - 1  {
            return self.pos.evaluate();
        }

        if self.nodes & INPUT_POLL_INTERVAL == 0 {
            self.poll_input()
        }

        if depth == 0 || self.pos.half_moves == 100 {
            //return evaluate(game)
            return self.quiescence(alpha, beta);
        }

        let mut hash_flag = HashFlag::Alpha;

        self.nodes += 1;

        let in_check = self.pos.is_in_check(self.pos.active_player);

        let n_depth = if in_check { depth + 1 } else { depth };

        let mut temp_alpha = alpha;

        let mut best_move = NULL_MOVE;

        let mut legal_moves = 0;

        //Null move pruning
        if n_depth >= 3 && !in_check && self.ply > 0 {
            let old_pos = self.pos;

            //Switch side + update hash
            self.pos.active_player = opposite_color(self.pos.active_player);
            self.pos.zobrist_hash ^= SIDE_KEY;

            //Reset enpassant + update hash
            if self.pos.enpassant_square != Square::None {
                self.pos.zobrist_hash ^= ENPASSANT_KEYS[self.pos.enpassant_square as usize];
            };
            self.pos.enpassant_square = Square::None;

            //..., Depth - 1 - R (with R = 2), ...

            self.ply += 1;

            score = -self.negamax(n_depth - 1 - 2, -beta, -beta + 1);

            //Return to previous state
            self.pos = old_pos;

            self.ply -= 1;

            if self.stopping { return 0 }

            //Cut-off
            if score >= beta {
                return beta
            }
        }

        let mut moves = MoveGenerator::new_sorted(&self.pos, MoveTypes::All, self);
        moves.add_pv_move(self.tt);
        let mut moves = moves.peekable();

        let mut moves_searched = 0;

        while moves.peek().is_some() {
            //Always OK due to check just before
            let m = unsafe { moves.next().unwrap_unchecked() };
            
            let old_pos = self.pos;

            self.ply += 1;

            if !self.pos.make_move(&m, &mut self.repetition_table) { 
                self.ply -= 1;

                self.pos = old_pos;
                
                continue;
            }

            legal_moves += 1;

            self.repetition_table.move_back();

            if moves_searched == 0 {
                best_move = m;
                //Full PV Search
                score = -self.negamax(n_depth - 1, -beta, -temp_alpha);
            } else {
                //Regular search with LMR

                score = if  moves_searched >= FULL_DEPTH_MOVES && 
                            depth >= REDUCTION_LIMIT &&
                            !in_check &&
                            !m.is_capture() &&
                            m.promotion() == Piece::None as u8 {
                    //Reduced search
                    -self.negamax(n_depth - 2, -temp_alpha - 1, -temp_alpha)

                } else {
                    //Ensure a full search
                    temp_alpha + 1
                };

                //PVS
                if score > temp_alpha {
                    score = -self.negamax(n_depth - 1, -temp_alpha - 1, -temp_alpha);

                    //Check bounds
                    if score > temp_alpha && score < beta {
                        //Full search on failure
                        score = -self.negamax(n_depth - 1, -beta, -temp_alpha);
                    }
                }
            }

            self.ply -= 1;

            moves_searched += 1;

            //Reset internal board to state from before
            self.pos = old_pos;

            if self.stopping { return 0 }

            if score > temp_alpha {
                best_move = m;

                //Beta cut-off
                if score >= beta {
                    //Update killer moves
                    if !m.is_capture() {
                        self.killer_moves[1][self.ply as usize] = self.killer_moves[0][self.ply as usize];
                        self.killer_moves[0][self.ply as usize] = m;
                    }
        
                    //Record TT entry
                    self.tt.record(self.pos.zobrist_hash, beta, depth, HashFlag::Beta, self.ply, best_move);
        
                    return beta;
                }

                //Signal to record TT entry
                hash_flag = HashFlag::Exact;

                //Update history move
                if !m.is_capture() {
                    self.history_moves[m.piece() as usize][m.to_square() as usize] += depth as u8
                }

                temp_alpha = score;
            }
        }

        //Mate & Draw
        if legal_moves == 0 {
            if in_check {
                temp_alpha = -MATE_VALUE + self.ply as i32;
            }
            else {
                temp_alpha = 0;
            }
        }
        
        //Record TT entry
        self.tt.record(self.pos.zobrist_hash, temp_alpha, depth, hash_flag, self.ply, best_move);

        temp_alpha
    }

    #[inline]
    fn quiescence(&mut self, alpha: i32, beta: i32) -> i32 {
        if self.nodes & INPUT_POLL_INTERVAL == 0 {
            self.poll_input()
        }

        self.nodes += 1;

        let eval = self.pos.evaluate();

        //Dont't go on if reached max ply
        if self.ply > MAX_PLY as u8 - 1 || self.pos.half_moves == 100 {
            return eval;
        }

        let mut temp_alpha = alpha;

        if eval > temp_alpha {
            temp_alpha = eval;

            if eval >= beta {
                return beta
            }
        }

        let mut moves = MoveGenerator::new_sorted(&self.pos, MoveTypes::All, self);
        moves.add_pv_move(self.tt);
        let mut moves = moves.peekable();

        while moves.peek().is_some() {
            //Always OK due to check just before
            let m = unsafe { moves.next().unwrap_unchecked() };

            let mut old_pos = self.pos;

            if !self.pos.make_move(&m, self.repetition_table) {
                self.pos = old_pos;
                continue;
            }
            
            self.ply += 1;

            let score = -self.quiescence(-beta, -temp_alpha);

            self.ply -= 1;

            //Recover old position
            self.pos = old_pos;

            self.repetition_table.move_back();

            if score >= beta {
                return beta;
            }

            if score > temp_alpha {
                temp_alpha = score;
            }
        }

        temp_alpha
    }
}