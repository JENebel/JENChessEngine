use super::*;

pub struct SearchEnv<'a> {
    pub nodes: u64,
    pub ply: u8,
    pub killer_moves: [[Option<Move>; MAX_PLY]; 2],
    pub history_moves: [[i32; 64]; 12],
    pub pv_lengths: [usize; MAX_PLY],
    pub pv_table: [[Move; MAX_PLY]; MAX_PLY],
    pub follow_pv: bool,
    pub score_pv: bool,
    pub stopping: bool,
    io_receiver: &'a IoWrapper,
    pub start_time: SystemTime,
    max_time: i64,
    pub tt_hits: u32,
    pub repetition_table: &'a mut RepetitionTable,
}

impl <'a>SearchEnv<'a> {
    pub fn new(max_time: i64, io_receiver: &'a IoWrapper, rep_table: &'a mut RepetitionTable) -> Self {
        Self{
            nodes: 0,
            ply: 0,
            killer_moves: [[None; MAX_PLY]; 2],
            history_moves: [[0 as i32; 64]; 12],
            pv_lengths: [0; MAX_PLY],
            pv_table: [[NULL_MOVE; MAX_PLY]; MAX_PLY],
            follow_pv: false,
            score_pv: false,
            stopping: false,
            io_receiver: io_receiver,
            start_time: SystemTime::now(),
            max_time: max_time,
            tt_hits: 0,
            repetition_table: rep_table
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

    pub fn poll_input(&mut self) {
        if (self.max_time != -1 && self.start_time.elapsed().unwrap().as_millis() as i64 >= self.max_time) || self.io_receiver.try_read_line().is_some() {
            self.stopping = true;
            return;
        }
    }
}