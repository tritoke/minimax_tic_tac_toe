use crate::agent::{Agent, Move};
use crate::game::{Board, Player};
use std::io::{self, Write};

pub struct Human();

impl Agent for Human {
    fn make_move(&self, board: &Board, player: Player) -> Option<Move> {
        let mut line = String::new();
        loop {
            print!("Human the board looks like this:\n{board}\n\nYou play {player:?}, make your move (row, col): ");
            line.clear();
            io::stdout().flush().ok()?;
            io::stdin().read_line(&mut line).ok()?;

            match line.parse() {
                Ok(mv) => break Some(mv),
                Err(e) => println!("{e}"),
            }
        }
    }

    fn is_ai(&self) -> bool {
        false
    }
}
