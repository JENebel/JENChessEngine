use super::*;

pub struct SearchEnv<'a> {
    pub nodes: u64,
    pub ply: u8,
    pub killer_moves: [[Move; MAX_PLY]; 2],
    pub history_moves: [[u8; 64]; 12],
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
            killer_moves: [[NULL_MOVE; MAX_PLY]; 2],
            history_moves: [[0; 64]; 12],
            stopping: false,
            io_receiver: io_receiver,
            start_time: SystemTime::now(),
            max_time: max_time,
            tt_hits: 0,
            repetition_table: rep_table
        }
    }

    pub fn poll_input(&mut self) {
        if (self.max_time != -1 && self.start_time.elapsed().unwrap().as_millis() as i64 >= self.max_time) || self.io_receiver.try_read_line().is_some() {
            self.stopping = true;
            return;
        }
    }
}