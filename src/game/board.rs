use std::fmt;

use crate::game::{IllegalMoveError, Player, Tile};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Board {
    tiles: [Tile; 9],
}

impl Board {
    pub fn winner(&self) -> Option<Player> {
        macro_rules! all_equal {
            ($idx1:expr, $idx2:expr, $idx3:expr) => {
                self.tiles[$idx1] == self.tiles[$idx2] && self.tiles[$idx2] == self.tiles[$idx3]
            };
        }

        // check first row, col and diag
        let mut winner = None;
        if all_equal!(0, 1, 2) || all_equal!(0, 3, 6) || all_equal!(0, 4, 8) {
            winner = winner.or(Player::from_tile(self.tiles[0]));
        }

        // check second row, col and opposite diag
        if all_equal!(3, 4, 5) || all_equal!(1, 4, 7) || all_equal!(2, 4, 6) {
            winner = winner.or(Player::from_tile(self.tiles[4]));
        }

        // check last row and col
        if all_equal!(6, 7, 8) || all_equal!(2, 5, 8) {
            winner = winner.or(Player::from_tile(self.tiles[8]));
        }

        winner
    }

    pub fn is_full(&self) -> bool {
        self.tiles.iter().find(|tile| tile.is_empty()).is_none()
    }

    pub fn attempt_move(
        &mut self,
        row: u8,
        col: u8,
        player: Player,
    ) -> Result<(), IllegalMoveError> {
        let tile = self
            .get_tile_mut(row, col)
            .ok_or(IllegalMoveError::MoveOutOfBounds)?;
        match tile {
            // empty tile is okay to make a move
            Tile::Empty => {
                *tile = player.into();
                Ok(())
            }

            // Nought / Cross make this an illegal move
            Tile::Nought => Err(IllegalMoveError::TileOccupied(Player::Noughts)),
            Tile::Cross => Err(IllegalMoveError::TileOccupied(Player::Crosses)),
        }
    }

    pub fn get_tile(&self, row: u8, col: u8) -> Option<&Tile> {
        if row >= 3 || col >= 3 {
            None
        } else {
            let idx = row * 3 + col;
            self.tiles.get(idx as usize)
        }
    }

    fn get_tile_mut(&mut self, row: u8, col: u8) -> Option<&mut Tile> {
        if row >= 3 || col >= 3 {
            None
        } else {
            let idx = row * 3 + col;
            self.tiles.get_mut(idx as usize)
        }
    }
}

impl fmt::Display for Board {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f,
             "     1   2   3  \n   \
                ╔═══╤═══╤═══╗\n \
              1 ║ {} │ {} │ {} ║\n   \
                ╟───┼───┼───╢\n \
              2 ║ {} │ {} │ {} ║\n   \
                ╟───┼───┼───╢\n \
              3 ║ {} │ {} │ {} ║\n   \
                ╚═══╧═══╧═══╝",
             self.tiles[0], self.tiles[1], self.tiles[2],
             self.tiles[3], self.tiles[4], self.tiles[5],
             self.tiles[6], self.tiles[7], self.tiles[8]
         )
    }
}
