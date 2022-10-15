use super::*;
pub struct MoveList {
    moves: [Option<Move>; 256],
    count: usize
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [None; 256],
            count: 0
        }
    }
    
    pub fn add_move(&mut self, cmove: Move) {
        self.moves[self.count] = Some(cmove);
        self.count += 1;
    }

    pub fn print(&self) {
        println!("Move count: {}", self.count);
        for i in 0..self.count {
            self.moves[i].unwrap().print();
        }
        println!();
    }
}