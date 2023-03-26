# JENChessEngine

JENChessEngine, or just JENCE, is a chess engine written in rust.

ELO is around 2200.

If you are lucky it is online and accepting challenges on [Lichess](https://lichess.org/@/JENCE).

Development is well under way for the successor to this engine, [Cadabra](https://github.com/PQNebel/Cadabra).

## Using the engine

Note that a BMI2 enabled cpu is required. This should be supported by all newer x86 processors.

Binaries for linux and windows are available under the latest release, but the best performance is achieved by compiling the source. In testing the provided binary is 10-20% slower than a natively compiled one.

It is simply compiled with

    cargo build --release

The engine uses the [UCI](https://backscattering.de/chess/uci/) interface for automatic playing with these commands:

    x/exit/quit             // All 3 variants exit the program
    d                       // Displays the current board
    position startpos       // Loads the starting position. Moves can be provided after: "position startpos moves a2a4 a6a5"
    position fen [fen]      // Loads a position from a given fen string. Moves can be provided like for startpos
    perft [depth]           // Finds number of legal moves at some depth
    perft [depth] simple    // Same as perft but does not print result pr. move
    perft! [depth]          // Performs a seperate simple perft for each depth <= [depth]
    unmake/undo             // Unmakes the last made move if one exists
    make/move [move]        // Make a move on the board. On the standard UCI form: "a2a4" and one of "kbrq" appended for promotions
    eval                    // Prints the heuristic evaluation of the current position
    psuite                  // Runs a suite of perft tests to validate movegenerator and to test performance
    psuite long             // Same as psuite but runs to a deeper depth
    sbench                  // Runs a benchmark of the searching algorithm
    help                    // Writes out all legal commands. Note that the list provided from this command is out of date

## Technicalities

### Move generation

The engine uses bitboards, and pre-calculated attack tables with PEXT indexing for sliding piece move generation.\
This is why bmi2 is required.

### Search

* Move ordering heuristics
  * PV first
  * MVV_LVA table
  * 2 killer moves
  * History moves
* Search techniques
  * Negamax alpha/beta
  * Quiescence search
  * Check extension
  * Null move pruning
  * Late Move Reduction
  * PV search
  * Narrow aspiration window
  * Iterative deepening
  * Simple 64Mb transposition table
  * Threefold repitition detection
* Evaluation
  * Material values
  * Piece-Square tables
  * Simple Pawn structure bonuses/penalties
  * Simple piece mobility
  * Simple king safety
  
## Credits

* [CodeMonkeyKing](https://github.com/maksimKorzh)'s [BBC](https://github.com/maksimKorzh/bbc) has been a very important inspiration and help with his great [YouTube series](https://youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs)
* The [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page) is an awesome reasource.
* And of course all the open source chess engines out there!

## Limitations

* No options can be changed through the UCI interface.
* It is not very portable as it requires a BMI2 enabled CPU.
* The code is not very idiomatic (or pretty), as it was one of my first projects in Rust. See the successor [Cadabra](https://github.com/PQNebel/Cadabra).
