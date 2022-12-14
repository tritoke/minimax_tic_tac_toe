# MiniMax Tic Tac Toe

An implementation of the minimax algorithm for playing tic-tac-toe, written in rust, massively overengineered.

It should gracefully handle all cases for invalid user input and weird bot bugs (none left???).

## MSRV
The minimum supported rust version is `1.62.1`.

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
minimax_tic_tac_toe on ๎  main [!?] is ๐ฆ v0.1.0 via ๐ฆ v1.66.0
โ cargo run -- ai human
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minimax_tic_tac_toe ai human`
Human the board looks like this:
     1   2   3
   โโโโโคโโโโคโโโโ
 1 โ O โ   โ   โ
   โโโโโผโโโโผโโโโข
 2 โ   โ   โ   โ
   โโโโโผโโโโผโโโโข
 3 โ   โ   โ   โ
   โโโโโงโโโโงโโโโ

You play Crosses, make your move (row, col): 2,2
Human the board looks like this:
     1   2   3
   โโโโโคโโโโคโโโโ
 1 โ O โ O โ   โ
   โโโโโผโโโโผโโโโข
 2 โ   โ X โ   โ
   โโโโโผโโโโผโโโโข
 3 โ   โ   โ   โ
   โโโโโงโโโโงโโโโ

You play Crosses, make your move (row, col): 1,3
Human the board looks like this:
     1   2   3
   โโโโโคโโโโคโโโโ
 1 โ O โ O โ X โ
   โโโโโผโโโโผโโโโข
 2 โ   โ X โ   โ
   โโโโโผโโโโผโโโโข
 3 โ O โ   โ   โ
   โโโโโงโโโโงโโโโ

You play Crosses, make your move (row, col): 2,1
Human the board looks like this:
     1   2   3
   โโโโโคโโโโคโโโโ
 1 โ O โ O โ X โ
   โโโโโผโโโโผโโโโข
 2 โ X โ X โ O โ
   โโโโโผโโโโผโโโโข
 3 โ O โ   โ   โ
   โโโโโงโโโโงโโโโ

You play Crosses, make your move (row, col): 3,2
The game is a draw.
board:
     1   2   3
   โโโโโคโโโโคโโโโ
 1 โ O โ O โ X โ
   โโโโโผโโโโผโโโโข
 2 โ X โ X โ O โ
   โโโโโผโโโโผโโโโข
 3 โ O โ X โ O โ
   โโโโโงโโโโงโโโโ
```
