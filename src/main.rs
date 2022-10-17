#![feature(const_eval_limit)]
#![const_eval_limit = "0"]

mod game;
mod bitboard;
mod attack_tables;
mod cmove;
mod move_list;

use std::{io::{self, BufRead}, process, time::SystemTime};

use game::*;
use cmove::*;
use bitboard::*;
use attack_tables::*;
use move_list::*;

fn main() {
    let mut game = Game::new_from_start_pos();
    println!("{}", "--Engine started--");
    print_help();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap().trim().to_string();
        if input != "" {
            let mut split = input.splitn(2, " ");
            match split.next().unwrap().to_ascii_lowercase().as_str() {
                "exit" | "x" => { println!(" {}", "Exited!"); process::exit(0) },
                "help" => print_help(),
                "d" => { game.pretty_print();
                        /*println!(" {}", game.export_fen());*/ },
                "pos" => {
                    let pos = split.next().unwrap().to_string();
                    if pos == "start" {
                        game = Game::new_from_start_pos();
                    }
                    else {
                        game = Game::new_from_fen(pos.as_str());
                    }
                },
                "perft" => {
                    let mut split2 = split.next().unwrap().splitn(2, " ");
                    let pos = split2.next().unwrap().to_string();
                    let depth = (if pos == "simple" { split2.next().unwrap() } else { pos.as_str() }).parse::<u8>().unwrap();
                    perft(depth, game, pos != "simple");
                },
                "perft!" => {
                    let depth = split.next().unwrap().parse::<u8>().unwrap();
                    for i in 1..depth + 1 {
                        perft(i, game, false)
                    }
                    println!(" Done with perft!")
                },
                /*"fen" => {
                    println!(" {}", game.export_fen())
                },
                "unmake" | "undo" => {
                    if game.stack_index > 0 {
                        game.unmake_last_move();
                        println!(" Move unmade succesfully")
                    }
                    else {
                        println!(" Can't unmake move, stack is empty")
                    }
                },
                "make" | "move" => {
                    let mov = split.next().unwrap().to_string();

                    let from = square_from_string(mov.chars().skip(0).take(2).collect::<String>().as_str());
                    let to = square_from_string(mov.chars().skip(2).take(2).collect::<String>().as_str());

                    let pos_movs: Vec<Move> = game.generate_legal_moves_from(from).into_iter().filter(|m| m.to == to).collect();

                    if pos_movs.len() == 0 { println!("Not a possible move!"); break }
                    if pos_movs.len() > 1 && mov.len() < 5 { println!("Please specify promotion by Q, R, B or N"); break }

                    if pos_movs.len() == 1 { game.make_move(pos_movs[0]) }
                    else {
                        let prom = mov.chars().skip(4).take(1).collect::<String>();
                        let mov = match prom.to_uppercase().as_str() {
                            "Q" => pos_movs.into_iter().find(|m| m.move_kind == MoveKind::QueenPromotion  || m.move_kind == MoveKind::QueenPromotion).unwrap(),
                            "R" => pos_movs.into_iter().find(|m| m.move_kind == MoveKind::RookPromotion   || m.move_kind == MoveKind::RookPromotion).unwrap(),
                            "B" => pos_movs.into_iter().find(|m| m.move_kind == MoveKind::BishopPromotion || m.move_kind == MoveKind::BishopPromotion).unwrap(),
                            "N" => pos_movs.into_iter().find(|m| m.move_kind == MoveKind::KnightPromotion || m.move_kind == MoveKind::KnightPromotion).unwrap(),
                            _ => { println!(" Not a valid move"); break; }
                        };
                        game.make_move(mov);
                    }
                },*/
                "psuite" => {
                    psuite()
                },
                /*"eval" => {
                    println!("{}", game.evaluate_position())
                },*/
                _ => println!(" {}", " Unknown command")
            }
        }
    }
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
    let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10");
    let start = SystemTime::now();
    let r2 = game.perft(4, false);
    let duration2 = start.elapsed().unwrap();
    if r2 != 4085603 {println!(" ERROR! Found {} moves for depth 4 on Kiwipete, and expected 4,085,603", r2);}
    println!(" Perft on Kiwipete at depth 4 found in {}ms", duration2.as_millis());

    //Position 3
    let mut game = Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 10");
    let start = SystemTime::now();
    let r3 = game.perft(6, false);
    let duration3 = start.elapsed().unwrap();
    if r3 != 11030083 {println!(" ERROR! Found {} moves for depth 6 on Position 3, and expected 11,030,083", r3);}
    println!(" Perft on Position 3 at depth 6 found in {}ms", duration3.as_millis());

    //Position 4
    let mut game = Game::new_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let start = SystemTime::now();
    let r4 = game.perft(5, false);
    let duration4 = start.elapsed().unwrap();
    if r4 != 15833292 {println!(" ERROR! Found {} moves for depth 5 on Position 4, and expected 15,833,292", r4);}
    println!(" Perft on Position 4 at depth 5 found in {}ms", duration4.as_millis());

    //Position 5
    let mut game = Game::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let start = SystemTime::now();
    let r5 = game.perft(4, false);
    let duration5 = start.elapsed().unwrap();
    if r5 != 2103487 {println!(" ERROR! Found {} moves for depth 4 on Position 5, and expected 2,103,487", r5);}
    println!(" Perft on Position 5 at depth 4 found in {}ms", duration5.as_millis());

    //Position 6
    let mut game = Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
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
    println!(" speed: {}/s", total_result as i128 / (time as i128 / 1000 as i128));
}

fn print_help() {
    println!(" Commands:");
    println!("  {}", "help                   - Displays all legal commands");
    println!("  {}", "exit/x                 - Closes application");
    println!("  {}", "d                      - Displays the current board");
    println!("  {}", "pos [position]         - Sets the game to the given FEN, or to the initial state with \"start\"");
    println!("  {}", "fen                    - Prints the FEN string for the current position");
    println!("  {}", "perft (opt) [depth]    - Counts the number of legal moves at the given depth. Add the simple as \"opt\" to do barebones");
    println!("  {}", "perft! [depth]         - Does a simple perft for every PLY up to n");
    println!("  {}", "unmake/undo            - Unmakes last move if possible");
    println!("  {}", "make/move [move]       - Make move with active player. move example: \"h3h4\" in case of promotion, add a Q, R, B or N, so fx. \"a6a7Q\"");
    println!("  {}", "psuite                 - Performs an extensive performance test with perft on several positions");
    println!("  {}", "eval                   - Evaluates the current position, and shows the result");
}