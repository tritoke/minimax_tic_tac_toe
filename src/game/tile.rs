use crate::game::Player;
use std::fmt::{self, Write};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    #[default]
    Empty,
    Nought,
    Cross,
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        *self == Tile::Empty
    }

    pub fn is_nought(&self) -> bool {
        *self == Tile::Nought
    }

    pub fn is_cross(&self) -> bool {
        *self == Tile::Cross
    }
}

impl From<Player> for Tile {
    fn from(player: Player) -> Tile {
        match player {
            Player::Noughts => Tile::Nought,
            Player::Crosses => Tile::Cross,
        }
    }
}

impl fmt::Display for Tile {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Empty  => ' ',
            Tile::Nought => 'O',
            Tile::Cross  => 'X',
        };

        f.write_char(c)
    }
}
