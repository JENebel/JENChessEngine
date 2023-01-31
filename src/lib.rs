mod bitboard;
mod position;
mod attack_tables;
mod uci;
mod evaluation;
mod constants;
mod move_generator;
mod cmove;

pub struct Settings {
    pub threads: u8,
    pub transposition_table_mb: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self { threads: 1, transposition_table_mb: 128 }
    }
}

pub struct SearchContext {
    // TranspositionTable
    // 
}