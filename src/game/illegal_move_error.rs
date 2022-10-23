use crate::game::Player;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IllegalMoveError {
    MoveOutOfBounds,
    TileOccupied(Player),
}

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MoveOutOfBounds => write!(
                f,
                "Move out of bounds, row and col must both be one of [1,2,3]"
            ),
            Self::TileOccupied(p) => write!(f, "Tile is already occupied by the {p:?} player"),
        }
    }
}
