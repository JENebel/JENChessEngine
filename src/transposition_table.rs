use super::*;

const TT_SIZE: usize = 0x16_00000 / 18; // (byte size of TT) / (Size of TT entry)

#[derive(Copy, Clone)]
pub enum HashFlag {
    Alpha,
    Beta,
    Exact
}

#[derive(Copy, Clone)]
pub enum TranspositionTableEntry {
    Record {
        hash: u64,
        depth: u8,
        flag: HashFlag,
        score: i32,
        best: Move
    },
    Empty
}

pub struct TranspositionTable {
    table: Vec<TranspositionTableEntry>
}

impl TranspositionTableEntry {
    pub fn new(hash: u64, depth: u8, flag: HashFlag, score: i32, best: Move) -> Self {
        Self::Record { hash: hash, depth: depth, flag: flag, score: score, best: best }
    }
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self{table: vec![TranspositionTableEntry::Empty; TT_SIZE]}
    }

    pub fn record_position(&mut self, hash: u64, score: i32, depth: u8, flag: HashFlag, best: Move) {
        self.table[(hash % TT_SIZE as u64) as usize] = TranspositionTableEntry::new(hash, depth, flag, score, best)
    }

    pub fn probe(&mut self, zhash: u64, depth: u8, alpha: i32, beta: i32) -> &TranspositionTableEntry {
        let entry = &self.table[(zhash % TT_SIZE as u64) as usize];

        match entry {
            TranspositionTableEntry::Record { hash, depth, flag, score, best } => if zhash == *hash {
                    return entry;
                }
                else {
                    &TranspositionTableEntry::Empty
                },
            TranspositionTableEntry::Empty => &TranspositionTableEntry::Empty,
        }
    }
}