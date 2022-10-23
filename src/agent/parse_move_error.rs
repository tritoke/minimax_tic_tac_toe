use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseMoveError {
    RowOutOfBounds(u8),
    ColOutOfBounds(u8),
    InvalidRow(String),
    InvalidCol(String),
    FormatError,
}

impl fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseMoveError::RowOutOfBounds(row) => write!(
                f,
                "Invalid move row={row} is out of bounds, it should be either 1, 2 or 3"
            ),
            ParseMoveError::ColOutOfBounds(col) => write!(
                f,
                "Invalid move col={col} is out of bounds, it should be either 1, 2 or 3"
            ),
            ParseMoveError::InvalidRow(row) => write!(
                f,
                "Invalid move row={row:?} is not a valid value, it should be either 1, 2 or 3"
            ),
            ParseMoveError::InvalidCol(col) => write!(
                f,
                "Invalid move col={col:?} is not a valid value, it should be either 1, 2 or 3"
            ),
            ParseMoveError::FormatError => write!(
                f,
                "Failed to parse move, invalid format - the correct format is row,col"
            ),
        }
    }
}
