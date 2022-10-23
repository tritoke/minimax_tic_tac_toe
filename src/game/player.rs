use crate::game::Tile;
use std::ops::Not;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    #[default]
    Noughts,
    Crosses,
}

impl Player {
    pub fn from_tile(tile: Tile) -> Option<Self> {
        match tile {
            Tile::Empty => None,
            Tile::Nought => Some(Player::Noughts),
            Tile::Cross => Some(Player::Crosses),
        }
    }

    pub fn is_noughts(&self) -> bool {
        *self == Player::Noughts
    }

    pub fn is_crosses(&self) -> bool {
        *self == Player::Crosses
    }
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Noughts => Player::Crosses,
            Player::Crosses => Player::Noughts,
        }
    }
}
