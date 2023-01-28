use super::{board::Board, snake::Snake, Game};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}
