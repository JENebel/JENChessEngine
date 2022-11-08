#[derive(Copy, Clone)]
pub struct RepetitionTable {
    pub table: [u64; 512],
    pub index: usize
}

impl RepetitionTable {
    pub fn new() -> Self {
        Self {table: [0; 512], index: 0 }
    }

    pub fn insert(&mut self, hash: u64) {
        self.table[self.index] = hash;
        self.index += 1;
    }

    pub fn move_back(&mut self) {
        self.index -= 1
    }

    pub fn is_now_in_threefold_repetition(&mut self) -> bool {
        let curr_hash = self.table[self.index];
        for h in (0..self.index).rev() {
            if self.table[h] == curr_hash {
                return true;
            }
        }
        return false
    }

    pub fn clear(&mut self) {
        self.index = 0;
    }
}