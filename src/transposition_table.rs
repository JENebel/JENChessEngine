use super::*;

pub const TT_SIZE: usize =  (32 * 1_048_576) / std::mem::size_of::<TranspositionTableEntry>();
pub const UNKNOWN_SCORE: i32 = i32::MIN;
pub const EMPTY_TT_ENTRY: TranspositionTableEntry = TranspositionTableEntry { hash: 0, depth: 0, flag: HashFlag::Alpha, score: UNKNOWN_SCORE, best_move: NULL_MOVE};

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum HashFlag {
    Alpha,
    Beta,
    Exact
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct TranspositionTableEntry {
    hash: u64,
    depth: u8,
    flag: HashFlag,
    score: i32,
    best_move: Move
}

pub struct TranspositionTable {
    table: Box<[TranspositionTableEntry]>
}

impl TranspositionTableEntry {
    pub fn new(hash: u64, depth: u8, flag: HashFlag, score: i32, best_move: Move) -> Self {
        Self {
            hash,
            depth,
            flag,
            score,
            best_move
        }
    }
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self{table: vec![EMPTY_TT_ENTRY; TT_SIZE].into_boxed_slice()}
    }

    pub fn record(&mut self, hash: u64, score: i32, depth: u8, flag: HashFlag, ply: u8, best_move: Move) {

        let index = (hash % TT_SIZE as u64) as usize;

        let existing = self.table[index];

        //Only replace if deeper AND existing is not exact
        if existing.depth <= depth && existing.flag != HashFlag::Exact {
            //Adjust mating scores before insertion
            let mut adjusted_score: i32 = score;
            if score < -MATE_BOUND {
                adjusted_score -= ply as i32;
            } else if score > MATE_BOUND {
                adjusted_score += ply as i32;
            }

            self.table[index] = TranspositionTableEntry::new(hash, depth, flag, adjusted_score, best_move)
        }
    }

    pub fn probe_score(&mut self, hash: u64, depth: u8, alpha: i32, beta: i32, ply: u8) -> i32 {

        let entry = &self.table[(hash % TT_SIZE as u64) as usize];

        //Empty entry
        if entry.score == UNKNOWN_SCORE {
            return UNKNOWN_SCORE;
        }

        if hash == entry.hash {
            if entry.depth >= depth {
                //Adjust mating scores before extraction
                let mut adjusted_score: i32 = entry.score;
                if adjusted_score < -MATE_BOUND {
                    adjusted_score += ply as i32;
                } else if adjusted_score > MATE_BOUND {
                    adjusted_score -= ply as i32;
                }

                //Return appropriate score
                if entry.flag == HashFlag::Exact {
                    return adjusted_score
                }
                else if entry.flag == HashFlag::Alpha && adjusted_score <= alpha {
                    return alpha
                }
                else if entry.flag == HashFlag::Beta && adjusted_score >= beta {
                    return beta
                }
            }
        }

        return UNKNOWN_SCORE;
    }

    pub fn probe_best_move(&self, hash: u64) -> Move {
        self.table[(hash % TT_SIZE as u64) as usize].best_move
    }

    pub fn clear(&mut self) {
        for i in 0..self.table.len() {
            self.table[i] = EMPTY_TT_ENTRY;
        }
    }
}