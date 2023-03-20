mod game;
mod bitboard;
mod attack_tables;
mod cmove;
mod move_list;
mod utilities;
mod search;
mod move_generator;
mod make_move;
mod perft;
mod evaluation;
mod transposition_table;
mod repetition_table;

use core::panic;
use std::{io::{self}, process, time::SystemTime};

use game::*;

use cmove::*;
use bitboard::*;
use attack_tables::*;
use move_list::*;
use utilities::*;
use search::*;
use move_generator::*;
use make_move::*;
use perft::*;
use evaluation::*;
use transposition_table::*;
use repetition_table::*;

fn main() {
    let io_receiver = IoWrapper::init();

    let mut game = Game::new_from_start_pos();

    let mut tt = TranspositionTable::new();

    let mut repetition_table = RepetitionTable::new();
    
    loop {
        let input = io_receiver.read_line();
        if input != "" {
            let mut split = input.split(" ").peekable();
            match split.next().unwrap().to_ascii_lowercase().as_str() {
                "exit" | "x" | "quit" => { println!(" Exited!"); process::exit(0) },
                "help" => print_help(),
                "d" => { game.pretty_print(); }
                "position" => {
                    if !split.peek().is_some() { continue; }
                    repetition_table.clear();
                    let p = parse_position(input.split_at(9).1.to_string(), &mut repetition_table);

                    if p.is_none() {
                        panic!(" Illegal fen string");
                    } else {
                        game = p.unwrap();
                    }
                },
                "perft" => {
                    if !split.peek().is_some() { continue; }
                    let mut split2 = split.next().unwrap().splitn(2, " ").peekable();
                    if !split2.peek().is_some() { continue; }
                    let pos = split2.next().unwrap().to_string();
                    let depth = (if pos == "simple" { if !split2.peek().is_some() { println!(" Please provide depth"); continue; } split2.next().unwrap() } else { pos.as_str() }).parse::<u8>().unwrap();
                    go_perft(depth, game, pos != "simple");
                },
                "perft!" => {
                    let depth = split.next().unwrap().parse::<u8>().unwrap();
                    for i in 1..depth + 1 {
                        go_perft(i, game, false)
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
                    print!("id author Joachim Enggaard Nebel\n");
                    print!("uciok\n");
                },
                "ucinewgame" | "cleartt" => {
                    repetition_table.clear();
                    tt.clear();
                },
                "isready" => print!("readyok\n"),
                "go" => {
                    if split.peek().is_none() { continue; }
                    parse_go(input.split_at(2).1.to_string(), &mut game, &io_receiver, &mut tt, &mut repetition_table)
                },
                "eval" => {
                    let result = evaluate(&game);
                    println!(" {}", result);
                },
                "sbench" => {
                    sbench(&io_receiver)
                },
                "move" => {
                    while !split.peek().is_none() {
                        let mov = split.next().unwrap();
                        let parsed = game.parse_move(mov.to_string());
                        if parsed.is_none() {
                            panic!("Illegal move");
                        }
                        else {
                            make_search_move(&mut game, &parsed.unwrap(), &mut repetition_table);
                        }
                    }
                },

                _ => println!(" {}", " Unknown command")
            }
        }
    }
}

fn parse_position(args: String, rep_table: &mut RepetitionTable) -> Option<Game> {
    let pos = args.split(" ").next().unwrap().to_string();
    let rest: String;
    let mut game;
    if pos == "startpos" {
        game = Game::new_from_start_pos();

        rest = args.chars().skip(9).collect();
    }
    else if pos == "fen" {
        if args.len() < 5 {
            return None;
        }
        let fen: String = args.chars().skip(4).take_while(|c| *c != 'm').collect();

        rest = args.chars().skip(4 + fen.len()).collect();

        let result = Game::new_from_fen(fen.as_str());
        match result {
            Some(g) => game = g,
            None => return None
        }
    }
    else { return None; }

    let mut split = rest.split(" ").peekable();

    if *split.peek().unwrap() == "moves" {
        split.next();
        while !split.peek().is_none() {
            let mov = split.next().unwrap();
            let parsed = game.parse_move(mov.to_string());
            if parsed.is_none() {
                panic!("Illegal move");
            }
            else {
                make_search_move(&mut game, &parsed.unwrap(), rep_table);
            }
        }
    }

    Some(game)
}

fn parse_go(args: String, game: &mut Game, io_receiver: &IoWrapper, tt: &mut TranspositionTable, rep_table: &mut RepetitionTable){
    let mut split = args.split(" ").peekable();

    //Load arguments
    let mut inc = 0;
    let mut time = -1;
    let mut moves_to_go = 30;
    let mut move_time = -1;
    let mut depth = -1;

    while split.peek().is_some() {
        let arg = split.next().unwrap();
        if arg == "" {
            continue;
        }
        match arg {
            "binc" => if game.active_player == Color::Black {
                let t = split.next().unwrap();
                inc = t.parse::<i64>().unwrap(); 
                } else {
                    split.next();
            },
            "winc" => if game.active_player == Color::White {
                let t = split.next().unwrap();
                inc = t.parse::<i64>().unwrap(); 
                } else {
                    split.next();
            },
            "btime" => if game.active_player == Color::Black {
                let t = split.next().unwrap();
                time = t.parse::<i64>().unwrap();
                } else {
                    split.next();
            },
            "wtime" => if game.active_player == Color::White {
                let t = split.next().unwrap();
                time = t.parse::<i64>().unwrap(); 
                } else {
                    split.next();
            },
            "movestogo" => {
                let t = split.next().unwrap();
                moves_to_go = t.parse::<i64>().unwrap(); 
            },
            "movetime" => {
                let t = split.next().unwrap();
                move_time = t.parse::<i64>().unwrap(); 
            },
            //Fixed depth
            "depth" => {
                if split.peek().is_none() { return; }
                let depth_str = split.next().unwrap();
                let d = depth_str.parse::<i8>();
                if d.is_err() { return; }
                depth = d.unwrap()
            },
            "infinite" => {},
            //Random mover
            "random" => {
                search_random(game);
                return;
            },
            
            _ => {
                println!("Illegal 'go' command: '{}'", arg);
            }
        }
    }

    //Decide time
    if move_time != -1 {
        time = move_time
    } else if time != -1 {
        if time > 2000 {
            time /= moves_to_go;
            time += inc;
            time -= 100;
        }
        else if inc != 0 {
            time = inc - 500;
        }
        else {
            time /= moves_to_go;
        }
    }

    //Run search
    search(game, depth, time, &io_receiver, tt, rep_table);
}

pub fn read_line() -> String {
    let stdin = io::stdin();
    let mut input: String = String::new(); 
    stdin.read_line(&mut input).expect("Could not read line");
    input.trim().to_string()
}

pub fn sbench(io_receiver: &IoWrapper) {
    let poss = [
        Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap(),    //Tricky position
        Game::new_from_fen("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1").unwrap(),     //killer position
        Game::new_from_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9").unwrap(),    //CMK position
        Game::new_from_fen("6k1/3q1pp1/pp5p/1r5n/8/1P3PP1/PQ4BP/2R3K1 w - - 0 1").unwrap(),
        Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 10").unwrap(),
        Game::new_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap(),
        Game::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap(),
        Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap(),
        Game::new_from_start_pos()
    ];
    let start = SystemTime::now();
    let depth = 10;
    let mut tt_hits = 0;
    let mut nodes = 0;
    for mut p in poss {
        //p.pretty_print();
        let result = search(&mut p, depth, -1, &io_receiver, &mut TranspositionTable::new(), &mut RepetitionTable::new());
        nodes += result.nodes_visited;
        tt_hits += result.tt_hits;
        if !result.reached_max_ply {
            println!("Cancelled!");
            return;
        }
    }
    let duration = start.elapsed().unwrap();
    println!(" RESULT: Depth: {}\t Nodes: {}\t TT hits: {}\tTime: {}ms", depth, nodes, tt_hits, duration.as_millis()); 
}

fn go_perft(depth: u8, mut game: Game, detail: bool) {
    let start = SystemTime::now();
    let result = perft(&mut game, depth, detail);
    let duration = start.elapsed().unwrap();
    println!(" Found {} moves for depth {} in {}ms", result, depth, duration.as_millis());
}

fn psuite() {
    println!(" Performance test running...");
    let mut game = Game::new_from_start_pos();

    //startpos
    let start = SystemTime::now();
    let r1 = perft(&mut game, 5, false);
    let duration1 = start.elapsed().unwrap();
    if r1 != 4865609 {println!(" ERROR! Found {} moves for depth 5 on start position, and expected 4,865,609", r1); }
    println!(" Perft on starting position at depth 5 found in {}ms", duration1.as_millis());

    //Kiwipete
    let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
    let start = SystemTime::now();
    let r2 = perft(&mut game, 4, false);
    let duration2 = start.elapsed().unwrap();
    if r2 != 4085603 {println!(" ERROR! Found {} moves for depth 4 on Kiwipete, and expected 4,085,603", r2);}
    println!(" Perft on Kiwipete at depth 4 found in {}ms", duration2.as_millis());

    //Position 3
    let mut game = Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r3 = perft(&mut game, 6, false);
    let duration3 = start.elapsed().unwrap();
    if r3 != 11030083 {println!(" ERROR! Found {} moves for depth 6 on Position 3, and expected 11,030,083", r3);}
    println!(" Perft on Position 3 at depth 6 found in {}ms", duration3.as_millis());

    //Position 4
    let mut game = Game::new_from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1").unwrap();
    let start = SystemTime::now();
    let r4 = perft(&mut game, 5, false);
    let duration4 = start.elapsed().unwrap();
    if r4 != 15833292 {println!(" ERROR! Found {} moves for depth 5 on Position 4, and expected 15,833,292", r4);}
    println!(" Perft on Position 4 at depth 5 found in {}ms", duration4.as_millis());

    //Position 5
    let mut game = Game::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let start = SystemTime::now();
    let r5 = perft(&mut game, 4, false);
    let duration5 = start.elapsed().unwrap();
    if r5 != 2103487 {println!(" ERROR! Found {} moves for depth 4 on Position 5, and expected 2,103,487", r5);}
    println!(" Perft on Position 5 at depth 4 found in {}ms", duration5.as_millis());

    //Position 6
    let mut game = Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r6 = perft(&mut game, 4, false);
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
    let r1 = perft(&mut game, 6, false);
    let duration1 = start.elapsed().unwrap();
    if r1 != 119060324 {println!(" ERROR! Found {} moves for depth 6 on start position, and expected 119,060,324", r1); return }
    println!(" Perft on starting position at depth 6 found in {}ms", duration1.as_millis());

    //Kiwipete
    let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
    let start = SystemTime::now();
    let r2 = perft(&mut game, 5, false);
    let duration2 = start.elapsed().unwrap();
    if r2 != 193690690 {println!(" ERROR! Found {} moves for depth 5 on Kiwipete, and expected 193,690,690", r2);}
    println!(" Perft on Kiwipete at depth 5 found in {}ms", duration2.as_millis());

    //Position 3
    let mut game = Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r3 = perft(&mut game, 7, false);
    let duration3 = start.elapsed().unwrap();
    if r3 != 178633661 {println!(" ERROR! Found {} moves for depth 7 on Position 3, and expected 178,633,661", r3);}
    println!(" Perft on Position 3 at depth 7 found in {}ms", duration3.as_millis());

    //Position 4
    let mut game = Game::new_from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1").unwrap();
    let start = SystemTime::now();
    let r4 = perft(&mut game, 6, false);
    let duration4 = start.elapsed().unwrap();
    if r4 != 706045033 {println!(" ERROR! Found {} moves for depth 6 on Position 4, and expected 706,045,033", r4);}
    println!(" Perft on Position 4 at depth 6 found in {}ms", duration4.as_millis());

    //Position 5
    let mut game = Game::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let start = SystemTime::now();
    let r5 = perft(&mut game, 5, false);
    let duration5 = start.elapsed().unwrap();
    if r5 != 89941194 {println!(" ERROR! Found {} moves for depth 5 on Position 5, and expected 89,941,194", r5);}
    println!(" Perft on Position 5 at depth 5 found in {}ms", duration5.as_millis());

    //Position 6
    let mut game = Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap();
    let start = SystemTime::now();
    let r6 = perft(&mut game, 5, false);
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
    println!(" x/exit/quit             All 3 variants exit the program
 d                       Displays the current board
 position startpos       Loads the starting position. Moves can be provided after: \"position startpos moves a2a4 a6a5\"
 position fen [fen]      Loads a position from a given fen string. Moves can be provided like for startpos
 perft [depth]           Finds number of legal moves at some depth
 perft [depth] simple    Same as perft but does not print result pr. move
 perft! [depth]          Performs a seperate simple perft for each depth <= [depth]
 unmake/undo             Unmakes the last made move if one exists
 make/move [move]        Make a move on the board. On the standard UCI form: \"a2a4\" and one of \"kbrq\" appended for promotions
 eval                    Prints the heuristic evaluation of the current position
 psuite                  Runs a suite of perft tests to validate movegenerator and to test performance
 psuite long             Same as psuite but runs to a deeper depth
 sbench                  Runs a benchmark of the searching algorithm
 help                    Writes out all legal commands. Note that the list provided from this command is out of date")
}