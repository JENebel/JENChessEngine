use super::*;


const TT_SIZE: usize = 0x16_00000 / std::mem::size_of::<TranspositionTable>(); // (byte size of TT) / (Size of TT entry)
pub const UNKNOWN_SCORE: i32 = i32::MIN;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum HashFlag {
    Alpha,
    Beta,
    Exact
}

#[derive(Copy, Clone)]
pub enum TranspositionTableEntry {
    Empty,
    Record {
        hash: u64,
        depth: u8,
        flag: HashFlag,
        score: i32,
        //best: Move
    }
}

pub struct TranspositionTable {
    table: Vec<TranspositionTableEntry>
}

impl TranspositionTableEntry {
    pub fn new(hash: u64, depth: u8, flag: HashFlag, score: i32) -> Self {
        Self::Record {
            hash: hash,
            depth: depth,
            flag: flag,
            score: score,
            //best: best 
        }
    }
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self{table: vec![TranspositionTableEntry::Empty; TT_SIZE]}
    }

    pub fn record(&mut self, hash: u64, score: i32, depth: u8, flag: HashFlag, ply: u8) {
        //Adjust mating scores before insertion
        let mut adjusted_score: i32 = score;
        if score < -MATE_BOUND {
            adjusted_score -= ply as i32;
        } else if score > MATE_BOUND {
            adjusted_score += ply as i32;
        }

        self.table[(hash % TT_SIZE as u64) as usize] = TranspositionTableEntry::new(hash, depth, flag, adjusted_score)
    }

    pub fn probe(&mut self, p_hash: u64, p_depth: u8, p_alpha: i32, p_beta: i32, ply: u8) -> i32 {

        let entry = &self.table[(p_hash % TT_SIZE as u64) as usize];

        match entry {
            TranspositionTableEntry::Record { hash, depth, flag, score } => {
                if p_hash == *hash {
                    if *depth >= p_depth {
                        //Adjust mating scores before extraction
                        let mut adjusted_score: i32 = *score;
                        if adjusted_score < -MATE_BOUND {
                            adjusted_score += ply as i32;
                        } else if adjusted_score > MATE_BOUND {
                            adjusted_score -= ply as i32;
                        }


                        if *flag == HashFlag::Exact {
                            return adjusted_score
                        }
                        else if *flag == HashFlag::Alpha && adjusted_score <= p_alpha {
                            return p_alpha
                        }
                        else if *flag == HashFlag::Beta && adjusted_score >= p_beta {
                            return p_beta
                        }
                    }
                }
            },
            TranspositionTableEntry::Empty => return UNKNOWN_SCORE,
        }

        return UNKNOWN_SCORE;
    }

    pub fn clear(&mut self) {
        for i in 0..self.table.len() {
            self.table[i] = TranspositionTableEntry::Empty;
        }
    }
}

#[cfg(test)]
mod tt_tests {
    use crate::*;

    #[test]
    pub fn tt () {
        let mut game = Game::new_from_fen("").unwrap();
        game.pretty_print();
        search_bare(&mut game, 4, -1, &IoWrapper::init());
    }
}