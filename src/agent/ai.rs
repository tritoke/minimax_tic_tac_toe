use crate::agent::{Agent, Move};
use crate::game::{Board, Player};

pub struct Ai();

impl Agent for Ai {
    fn make_move(&self, board: &Board, player: Player) -> Option<Move> {
        minimax(board, player, player).1
    }

    fn is_ai(&self) -> bool {
        true
    }
}

fn minimax(board: &Board, player: Player, curr_player: Player) -> (i32, Option<Move>) {
    if board.is_full() {
        return (match board.winner() {
            Some(p) => if p == player { 1 } else { -1 },
            None => 0,
        }, None);
    }

    let mut accum = if player == curr_player {
        // MAX node
        -1
    } else {
        // MIN node
        1
    };
    let mut best_move = None;
    for row in 0..3 {
        for col in 0..3 {
            if board.get_tile(row, col).map(|tile| tile.is_empty()) != Some(true) { continue };
            let mut b = board.clone();
            b.attempt_move(row, col, curr_player).expect("we already checked the move was valid");
            let v = minimax(&b, player, !curr_player).0;
            if player == curr_player {
                // MAX node
                if v > accum {
                    best_move = Some(Move{ row, col });
                }
                accum = accum.max(v);
            } else {
                // MIN node
                if v < accum {
                    best_move = Some(Move{ row, col });
                }
                accum = accum.min(v);
            }
        }
    }

    (accum, best_move)
}
