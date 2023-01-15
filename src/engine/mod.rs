use rand::seq::SliceRandom;

use crate::{
    board::{
        Battlesnake, Board,
        Move::{self, *},
    },
    game::GameState,
};
use std::collections::HashMap;

/// The engine for Ferrite.
#[derive(Debug, Clone)]
pub struct Engine {
    /// The ID of the game.
    game_id: String,

    /// The board where the game is played.
    board: Board,

    /// The Battlesnake that this engine is controlling.
    you: Battlesnake,
}

impl Engine {
    /// Create a new engine.
    pub fn new(game_id: String, initial_state: GameState) -> Self {
        Self {
            game_id,
            board: initial_state.board,
            you: initial_state.you,
        }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, state: GameState) {
        self.board = state.board;
        self.you = state.you;
    }

    /// Get the next move for the snake.
    pub fn get_move(&self) -> Move {
        let mut is_move_safe: HashMap<_, _> =
            vec![(Up, true), (Down, true), (Left, true), (Right, true)]
                .into_iter()
                .collect();

        // We've included code to prevent your Battlesnake from moving backwards
        let my_head = &self.you.body[0]; // Coordinates of your head
        let my_neck = &self.you.body[1]; // Coordinates of your "neck"

        if my_neck.x < my_head.x {
            // Neck is left of head, don't move left
            is_move_safe.insert(Left, false);
        } else if my_neck.x > my_head.x {
            // Neck is right of head, don't move right
            is_move_safe.insert(Right, false);
        } else if my_neck.y < my_head.y {
            // Neck is below head, don't move down
            is_move_safe.insert(Down, false);
        } else if my_neck.y > my_head.y {
            // Neck is above head, don't move up
            is_move_safe.insert(Up, false);
        }

        // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
        // let board_width = &board.width;
        // let board_height = &board.height;

        // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
        // let my_body = &you.body;

        // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
        // let opponents = &board.snakes;

        // Are there any safe moves left?
        let safe_moves = is_move_safe
            .into_iter()
            .filter(|&(_, v)| v)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        // Choose a random move from the safe ones
        let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

        // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
        // let food = &board.food;

        *chosen
    }
}
