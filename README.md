Overview
--------
The classic game of Tic-Tac-Toe. The game is played in the terminal and can be played against a human or against an AI. It is implemented in Rust and uses the bitboard representation of the game state. The game can also be used to run matches between AI algorithms. The AI can be one of the following:
- Monte Carlo tree search (MCTS)
- Random
- Minimax
- Simple

Building
--------
```
cargo build --release
```

Types of players
------------
- MCTS (0)
- Random (1)
- Minimax (2)
- Simple (3)
- User (4)

Options
-------
```
-t <type>           Type of player. Default is MCTS.
```
```
-o                  User plays as O. Default is X.
```
```
-m <type1> <type2>  AI vs. AI match (101 games).
```

Usage
-------
Play against MCTS:
```
cargo run --release --
```
Play as O against MCTS:
```
cargo run --release -- -o
```
Play against Random:
```
cargo run --release -- -t 1
```
Run a match between MCTS and Random:
```
cargo run --release -- -m 0 1
```
Play against another User:
```
cargo run --release -- -t 4
```
