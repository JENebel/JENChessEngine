use std::{thread, io::stdin, sync::mpsc::{self, Receiver}};

pub struct IoWrapper {
    receiver: Receiver<String>
}

impl IoWrapper {
    pub fn init() -> Self {
        Self { receiver: init_input_thread( )}
    }

    pub fn try_read_line(&self) -> Option<String> {
        match self.receiver.try_recv() {
            Ok(line) => Some(line.trim().to_string()),
            Err(_) => None,
        }
    }

    pub fn read_line(&self) -> String {
        match self.receiver.recv() {
            Ok(line) => line.trim().to_string(),
            Err(_) => unreachable!(),
        }
    }
}

fn init_input_thread() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap_or_default();
    });
    rx
}