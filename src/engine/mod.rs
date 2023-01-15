use crate::game::GameState;

/// The engine for Ferrite.
#[derive(Debug, Clone)]
pub struct Engine {
    game_id: String,
}

impl Engine {
    /// Create a new engine.
    pub fn new(game_id: String, _state: GameState) -> Self {
        Self { game_id }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, _state: GameState) {
        todo!()
    }

    /// Get the next move for the snake.
    pub fn get_move(&self, _state: GameState) -> String {
        todo!()
    }
}
