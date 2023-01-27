use std::collections::HashMap;

use super::{
    board::Board,
    moves::Move,
    snake::{Snake, SnakeID},
    Game,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}

impl GameState {
    /// Applies a set of moves to the entire game state. The move set is a map from a snake's ID to
    /// the move that snake will make.
    pub fn apply_moves(&mut self, moves: HashMap<SnakeID, Move>) {
        todo!()
    }
}
