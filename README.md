# JENChessEngine
or JENCE

A chess engine written in rust.

It is decently strong and performant but currently lacks an opening book, is incompetent in endgames and has an insufficient evaluation function.

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
* Evaluation
  * Material values
  * Piece-Square tables
