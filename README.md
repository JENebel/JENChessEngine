# JENChessEngine
or JENCE

A chess engine written in rust.

Current ELO is around 2000.

If you are lucky it is online and accepting challenges on lichess: https://lichess.org/@/JENCE

It has the following features:
* Backbone
  * Bitboards
  * Copy-make
  * Pre-calculated attack tables using PEXT for sliding pieces, thus requiring a BMI2 enabled CPU
* Move ordering heuristics
  * PV first
  * MVV_LVA
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
  * Simple transposition table
  * Threefold repitition detection
* Evaluation
  * Material values
  * Piece-Square tables
  * Simple Pawn structure bonuses/penalties
  * Simple piece mobility
  * Simple king safety
