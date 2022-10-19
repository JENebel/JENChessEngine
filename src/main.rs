mod game;
mod bitboard;
mod attack_tables;
mod cmove;
mod move_list;
mod utilities;

use core::panic;
use std::{io::{self, BufRead}, process, time::SystemTime};

use game::*;

use cmove::*;
use bitboard::*;
use attack_tables::*;
use move_list::*;
use utilities::*;

fn main() {
    let mut game = Game::new_from_start_pos();
    loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let input = line.unwrap().trim().to_string();
            if input != "" {
                let mut split = input.split(" ").peekable();
                match split.next().unwrap().to_ascii_lowercase().as_str() {
                    "exit" | "x" | "quit" => { println!(" Exited!"); process::exit(0) },
                    "help" => print_help(),
                    "d" => { game.pretty_print(); }
                    "position" => {
                        if !split.peek().is_some() { break }
                        let pos = split.next().unwrap().to_string();
                        if pos == "startpos" {
                            game = Game::new_from_start_pos();
                        }
                        else if pos == "fen" {
                            if !split.peek().is_some() { break }
                            let pos: String = input.chars().skip(8).skip_while(|c| c.is_whitespace()).skip(3).skip_while(|c| c.is_whitespace()).collect();
                            let result = Game::new_from_fen(pos.as_str());
                            match result {
                                Some(g) => game = g,
                                None => println!(" Illegal fen string")
                            }
                        }
                        else { break }
                        if split.peek().is_some() && split.next().unwrap() == "moves" {
                            while split.peek().is_some() {
                                let mov = split.next().unwrap();
                                let parsed = game.parse_move(mov.to_string());
                                if parsed.is_none() {
                                    panic!("Illegal move");
                                }
                                else {
                                    game.make_move(&parsed.unwrap())
                                }
                            }
                        }
                    },
                    "perft" => {
                        if !split.peek().is_some() { break }
                        let mut split2 = split.next().unwrap().splitn(2, " ").peekable();
                        if !split2.peek().is_some() { break }
                        let pos = split2.next().unwrap().to_string();
                        let depth = (if pos == "simple" { if !split2.peek().is_some() { println!(" Please provide depth"); break } split2.next().unwrap() } else { pos.as_str() }).parse::<u8>().unwrap();
                        perft(depth, game, pos != "simple");
                    },
                    "perft!" => {
                        let depth = split.next().unwrap().parse::<u8>().unwrap();
                        for i in 1..depth + 1 {
                            perft(i, game, false)
                        }
                        println!(" Done with perft!")
                    },
                    "psuite" => {
                        if split.peek().is_some() {
                            let pos = split.next().unwrap().to_string();
                            if pos == "long" {
                                psuite_long()
                            }
                        }
                        else {
                            psuite();
                        }
                    },
                    "uci" => {
                        print!("id name JENCE\n");
                        print!("id author Joachim Enggård Nebel\n");
                        print!("uciok\n");
                    },
                    "ucinewgame" => { },
                    "isready" => print!("readyok\n"),
                    "go" => {
                        if split.peek().is_none() { break; }
                        let com = split.next().unwrap();
                        let result: SearchResult;
                        match com {
                            "depth" => {
                                if split.peek().is_none() { break; }
                                let depth_str = split.next().unwrap();
                                let depth = depth_str.parse::<u16>();
                                if depth.is_err() { break; }
                                result = game.search(depth.unwrap() as u8);
                            },
                            "random" => {
                                result = game.search_random();
                            },
                            _ => result = game.search(5)
                        }
                        print!("info score cp {} depth {} nodes {}\n", result.score, result.depth, result.nodes_visited);
                        print!("bestmove {}\n", result.best_move.to_uci());
                    },
                    "eval" => {
                        let result = game.evaluate();
                        println!(" {}", result);
                    },
                    "sbench" => {
                        sbench()
                    },

                    _ => println!(" {}", " Unknown command")
                }
            }
        }
        println!(" {}", " Unknown command")
    }
}

fn sbench() {
    //let mut game = Game::new_from_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ").unwrap();
    //let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let mut game = Game::new_from_fen("6k1/3q1pp1/pp5p/1r5n/8/1P3PP1/PQ4BP/2R3K1 w - - 0 1").unwrap();

    let start = SystemTime::now();
    let depth = 6;
    let result = game.search(depth);
    let duration = start.elapsed().unwrap();
    println!(" Found best move: {} for depth {}. Visited: {} nodes in {}ms", result.best_move.to_uci(), depth, result.nodes_visited, duration.as_millis());
    //Found best move: a7a5 for depth 4. Visited: 48707832 nodes in 21837ms
    //Found best move: b7b5 for depth 4. Visited: 19566125 nodes in 8958ms
    //Found best move: b7b5 for depth 4. Visited: 190479 nodes in 110ms


    //Found best move: f3f4 for depth 6. Visited: 6507312 nodes in 5542ms
}

fn perft(depth: u8, mut game: Game, detail: bool) {
    let start = SystemTime::now();
    let result = game.perft(depth, detail);
    let duration = start.elapsed().unwrap();
    println!(" Found {} moves for depth {} in {}ms", result, depth, duration.as_millis());
}

fn psuite() {
    println!(" Performance test running...");
    let mut game = Game::new_from_start_pos();

    //startpos
    let start = SystemTime::now();
    let r1 = game.perft(5, false);
    let duration1 = start.elapsed().unwrap();
    if r1 != 4865609 {println!(" ERROR! Found {} moves for depth 5 on start position, and expected 4,865,609", r1); return }
    println!(" Perft on starting position at depth 5 found in {}ms", duration1.as_millis());

    //Kiwipete
    let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
    let start = SystemTime::now();
    let r2 = game.perft(4, false);
    let duration2 = start.elapsed().unwrap();
    if r2 != 4085603 {println!(" ERROR! Found {} moves for depth 4 on Kiwipete, and expected 4,085,603", r2);}
    println!(" Perft on Kiwipete at depth 4 found in {}ms", duration2.as_millis());

    //Position 3
    let mut game = Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r3 = game.perft(6, false);
    let duration3 = start.elapsed().unwrap();
    if r3 != 11030083 {println!(" ERROR! Found {} moves for depth 6 on Position 3, and expected 11,030,083", r3);}
    println!(" Perft on Position 3 at depth 6 found in {}ms", duration3.as_millis());

    //Position 4
    let mut game = Game::new_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
    let start = SystemTime::now();
    let r4 = game.perft(5, false);
    let duration4 = start.elapsed().unwrap();
    if r4 != 15833292 {println!(" ERROR! Found {} moves for depth 5 on Position 4, and expected 15,833,292", r4);}
    println!(" Perft on Position 4 at depth 5 found in {}ms", duration4.as_millis());

    //Position 5
    let mut game = Game::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let start = SystemTime::now();
    let r5 = game.perft(4, false);
    let duration5 = start.elapsed().unwrap();
    if r5 != 2103487 {println!(" ERROR! Found {} moves for depth 4 on Position 5, and expected 2,103,487", r5);}
    println!(" Perft on Position 5 at depth 4 found in {}ms", duration5.as_millis());

    //Position 6
    let mut game = Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r6 = game.perft(4, false);
    let duration6 = start.elapsed().unwrap();
    if r6 != 3894594 {println!(" ERROR! Found {} moves for depth 4 on Position 6, and expected 3,894,594", r6);}
    println!(" Perft on Position 6 at depth 4 found in {}ms", duration6.as_millis());

    //Result
    println!("\n Performance test done!\n Results are as follows:");
    println!(" 1: {}ms\n 2: {}ms\n 3: {}ms\n 4: {}ms\n 5: {}ms\n 6: {}ms", duration1.as_millis(), duration2.as_millis(),duration3.as_millis(),duration4.as_millis(),duration5.as_millis(),duration6.as_millis());
    let time = duration1.as_millis()+duration2.as_millis()+duration3.as_millis()+duration4.as_millis()+duration5.as_millis()+duration6.as_millis();
    let total_result = r1+r2+r3+r4+r5+r6;
    println!(" total: {}ms", time);
    println!(" speed: {}/s", (total_result as f64 / (time as f64 / 1000 as f64)) as u64);
}

fn psuite_long() {
    println!(" Long performance test running...");
    let mut game = Game::new_from_start_pos();

    //startpos
    let start = SystemTime::now();
    let r1 = game.perft(6, false);
    let duration1 = start.elapsed().unwrap();
    if r1 != 119060324 {println!(" ERROR! Found {} moves for depth 6 on start position, and expected 119,060,324", r1); return }
    println!(" Perft on starting position at depth 6 found in {}ms", duration1.as_millis());

    //Kiwipete
    let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
    let start = SystemTime::now();
    let r2 = game.perft(5, false);
    let duration2 = start.elapsed().unwrap();
    if r2 != 193690690 {println!(" ERROR! Found {} moves for depth 5 on Kiwipete, and expected 193,690,690", r2);}
    println!(" Perft on Kiwipete at depth 5 found in {}ms", duration2.as_millis());

    //Position 3
    let mut game = Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r3 = game.perft(7, false);
    let duration3 = start.elapsed().unwrap();
    if r3 != 178633661 {println!(" ERROR! Found {} moves for depth 7 on Position 3, and expected 178,633,661", r3);}
    println!(" Perft on Position 3 at depth 7 found in {}ms", duration3.as_millis());

    //Position 4
    let mut game = Game::new_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
    let start = SystemTime::now();
    let r4 = game.perft(6, false);
    let duration4 = start.elapsed().unwrap();
    if r4 != 706045033 {println!(" ERROR! Found {} moves for depth 6 on Position 4, and expected 706,045,033", r4);}
    println!(" Perft on Position 4 at depth 6 found in {}ms", duration4.as_millis());

    //Position 5
    let mut game = Game::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let start = SystemTime::now();
    let r5 = game.perft(5, false);
    let duration5 = start.elapsed().unwrap();
    if r5 != 89941194 {println!(" ERROR! Found {} moves for depth 5 on Position 5, and expected 89,941,194", r5);}
    println!(" Perft on Position 5 at depth 5 found in {}ms", duration5.as_millis());

    //Position 6
    let mut game = Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r6 = game.perft(5, false);
    let duration6 = start.elapsed().unwrap();
    if r6 != 164075551 {println!(" ERROR! Found {} moves for depth 5 on Position 6, and expected 164,075,551", r6);}
    println!(" Perft on Position 6 at depth 5 found in {}ms", duration6.as_millis());

    //Result
    println!("\n Performance test done!\n Results are as follows:");
    println!(" 1: {}ms\n 2: {}ms\n 3: {}ms\n 4: {}ms\n 5: {}ms\n 6: {}ms", duration1.as_millis(), duration2.as_millis(),duration3.as_millis(),duration4.as_millis(),duration5.as_millis(),duration6.as_millis());
    let time = duration1.as_millis()+duration2.as_millis()+duration3.as_millis()+duration4.as_millis()+duration5.as_millis()+duration6.as_millis();
    let total_result = r1+r2+r3+r4+r5+r6;
    println!(" total: {}ms", time);
    println!(" speed: {}/s", total_result as i128 / (time as i128 / 1000 as i128));
}

fn print_help() {
    println!(" Commands:");
    println!("  {}", "help                                  - Displays all legal commands");
    println!("  {}", "exit/x/quit                           - Closes application");
    println!("  {}", "d                                     - Displays the current board");
    println!("  {}", "position [startpos/fen(fen string)]]  - Sets the game to the given FEN, or to the initial state with \"start\"");
    println!("  {}", "fen                                   - Prints the FEN string for the current position");
    println!("  {}", "perft (opt) [depth]                   - Counts the number of legal moves at the given depth. Add the simple as \"opt\" to do barebones");
    println!("  {}", "perft! [depth]                        - Does a simple perft for every PLY up to n");
    println!("  {}", "unmake/undo                           - Unmakes last move if possible");
    println!("  {}", "make/move [move]                      - Make move with active player. move example: \"h3h4\" in case of promotion, add a Q, R, B or N, so fx. \"a6a7Q\"");
    println!("  {}", "psuite (opt)                          - Performs an extensive performance test with perft on several positions. \"opt\" can be \"long\" for longer test");
    println!("  {}", "eval                                  - Evaluates the current position, and shows the result");
}