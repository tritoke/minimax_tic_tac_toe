# MiniMax Tic Tac Toe

An implementation of the minimax algorithm for playing tic-tac-toe, written in rust, massively overengineered.

It should gracefully handle all cases for invalid user input and weird bot bugs (none left???).

## Playing the game
To play the game do:
```sh
cargo run -- <player 1 type> <player 2 type>
```

e.g. for an AI vs human game where the AI goes first:
```
cargo run -- ai human
```

## An example game
```
minimax_tic_tac_toe on  main [!?] is 📦 v0.1.0 via 🦀 v1.66.0
➜ cargo run -- ai human
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minimax_tic_tac_toe ai human`
Human the board looks like this:
     1   2   3
   ╔═══╤═══╤═══╗
 1 ║ O │   │   ║
   ╟───┼───┼───╢
 2 ║   │   │   ║
   ╟───┼───┼───╢
 3 ║   │   │   ║
   ╚═══╧═══╧═══╝

You play Crosses, make your move (row, col): 2,2
Human the board looks like this:
     1   2   3
   ╔═══╤═══╤═══╗
 1 ║ O │ O │   ║
   ╟───┼───┼───╢
 2 ║   │ X │   ║
   ╟───┼───┼───╢
 3 ║   │   │   ║
   ╚═══╧═══╧═══╝

You play Crosses, make your move (row, col): 1,3
Human the board looks like this:
     1   2   3
   ╔═══╤═══╤═══╗
 1 ║ O │ O │ X ║
   ╟───┼───┼───╢
 2 ║   │ X │   ║
   ╟───┼───┼───╢
 3 ║ O │   │   ║
   ╚═══╧═══╧═══╝

You play Crosses, make your move (row, col): 2,1
Human the board looks like this:
     1   2   3
   ╔═══╤═══╤═══╗
 1 ║ O │ O │ X ║
   ╟───┼───┼───╢
 2 ║ X │ X │ O ║
   ╟───┼───┼───╢
 3 ║ O │   │   ║
   ╚═══╧═══╧═══╝

You play Crosses, make your move (row, col): 3,2
The game is a draw.
board:
     1   2   3
   ╔═══╤═══╤═══╗
 1 ║ O │ O │ X ║
   ╟───┼───┼───╢
 2 ║ X │ X │ O ║
   ╟───┼───┼───╢
 3 ║ O │ X │ O ║
   ╚═══╧═══╧═══╝
```
