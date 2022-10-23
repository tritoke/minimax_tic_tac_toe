use minimax_tic_tac_toe::{
    agent::{Agent, Move},
    game::{Board, Player},
    PlayerType,
};

fn main() {
    fn get_player_type(n: usize) -> PlayerType {
        std::env::args()
            .nth(n)
            .and_then(|s| match s.as_str() {
                "human" => Some(PlayerType::Human),
                "ai" => Some(PlayerType::Ai),
                _ => None,
            })
            .unwrap_or_default()
    }

    // get the player type from the program arguments
    let player1 = get_player_type(1);
    let player2 = get_player_type(2);

    // setup the agents
    let agent_noughts: Box<dyn Agent> = player1.into_agent();
    let agent_crosses: Box<dyn Agent> = player2.into_agent();

    // setup the player and board
    let mut board: Board = Default::default();

    match play_game(
        &mut board,
        Player::default(),
        &agent_noughts,
        &agent_crosses,
    ) {
        GameResult::Winner(player) => {
            println!("The winner is: {player:?}!");
        }
        GameResult::Forfeit(player) => {
            // if an agent cannot make a move they forfeit
            println!("{player:?}, was unable to make a move, and thus forfeits the game.");
        }
        GameResult::Draw => {
            println!("The game is a draw.");
        }
    }

    println!("board:\n{board}");
}

fn play_game(
    board: &mut Board,
    starting_player: Player,
    agent_noughts: &Box<dyn Agent>,
    agent_crosses: &Box<dyn Agent>,
) -> GameResult {
    let mut curr_player = starting_player;
    loop {
        if agent_noughts.is_ai() && agent_crosses.is_ai() {
            println!("board:\n{board}\n");
        }

        // play until some player has a win
        if let Some(player) = board.winner() {
            return GameResult::Winner(player);
        }

        // if the board is full, its a draw
        if board.is_full() {
            return GameResult::Draw;
        }

        // loop until the agent has performed a valid move
        loop {
            // pick between agents and ask them to make a move
            let agent = match curr_player {
                Player::Noughts => agent_noughts,
                Player::Crosses => agent_crosses,
            };

            // get the agent's move, forfeit if they can't make one
            let Move { row, col } = if let Some(mv) = agent.make_move(&board, curr_player) {
                mv
            } else {
                return GameResult::Forfeit(curr_player);
            };

            // attempt to play the agent's move
            match board.attempt_move(row, col, curr_player) {
                // if the move is valid the player's move has been made so exit the loop
                Ok(()) => break,
                Err(e) => {
                    // if the AI made an illegal move, forfeit the game
                    if agent.is_ai() {
                        return GameResult::Forfeit(curr_player);
                    } else {
                        // if its the human tell them why it failed, and let them try again
                        println!("Invalid move: {e}");
                    }
                }
            }
        }

        // swap who is playing
        curr_player = !curr_player;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GameResult {
    Winner(Player),
    Forfeit(Player),
    Draw,
}
