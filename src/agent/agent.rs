use crate::agent::Move;
use crate::game::{Board, Player};

pub trait Agent {
    fn make_move(&self, board: &Board, player: Player) -> Option<Move>;
    fn is_ai(&self) -> bool;
}
