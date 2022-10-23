use crate::agent::ParseMoveError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    pub row: u8,
    pub col: u8,
}

impl std::str::FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, col) = s.split_once(',').ok_or(ParseMoveError::FormatError)?;
        let row = row
            .trim()
            .parse::<u8>()
            .map_err(|_| ParseMoveError::InvalidRow(row.trim().to_owned()))?;
        let col = col
            .trim()
            .parse::<u8>()
            .map_err(|_| ParseMoveError::InvalidCol(col.trim().to_owned()))?;

        if !(1..=3).contains(&row) {
            return Err(ParseMoveError::RowOutOfBounds(row));
        }

        if !(1..=3).contains(&col) {
            return Err(ParseMoveError::ColOutOfBounds(col));
        }

        Ok(Move {
            row: row - 1,
            col: col - 1,
        })
    }
}
