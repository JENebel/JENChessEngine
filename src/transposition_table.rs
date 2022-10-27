use super::*;

const TT_SIZE: usize = 0x16_00000 / 18; // (byte size of TT) / (Size of TT entry)
const NULL_TT_ENTRY: TranspositionTableEntry = TranspositionTableEntry{key: 0, depth: 0, flag: HashFlag::Alpha, score: 0, best: NULL_MOVE};

#[derive(Copy, Clone)]
pub enum HashFlag {
    Alpha,
    Beta,
    Exact
}

#[derive(Copy, Clone)]
pub struct TranspositionTableEntry {
    key: u64,
    depth: u8,
    flag: HashFlag,
    score: i32,
    best: Move
}

pub struct TranspositionTable {
    table: Vec<TranspositionTableEntry>
}

impl TranspositionTableEntry {
    pub fn new(key: u64, depth: u8, flag: HashFlag, score: i32, best: Move) -> Self {
        Self { key: key, depth: depth, flag: flag, score: score, best: best }
    }
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self{ table: vec![NULL_TT_ENTRY; TT_SIZE] }
    }

    pub fn record_position(&mut self) {

    }
}