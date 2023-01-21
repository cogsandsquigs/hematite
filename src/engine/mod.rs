mod hazards;
mod modes;
mod moveset;
mod safety;
mod utils;

use self::{modes::Mode, moveset::MoveSet};
use crate::game::{
    board::{Battlesnake, Board},
    moves::Move,
    GameState,
};
use log::debug;
use rand::{distributions::WeightedError, rngs::SmallRng, seq::SliceRandom, SeedableRng};

/// The engine for Hematite.
#[derive(Debug, Clone)]
pub struct Engine {
    /// The board where the game is played.
    board: Board,

    /// The Battlesnake that this engine is controlling.
    you: Battlesnake,

    /// The mode the engine is in
    mode: Mode,

    /// The random number generator for the engine.
    rng: SmallRng,

    /// The set of moves that the snake can make.
    moves: MoveSet,
}

impl Engine {
    /// Create a new engine.
    pub fn new(initial_state: GameState) -> Self {
        Self {
            board: initial_state.board,
            you: initial_state.you,
            mode: Mode::Hungry,
            rng: SmallRng::from_entropy(),
            moves: MoveSet::new(),
        }
    }

    /// Update the engine with a new game state.
    pub fn update(&mut self, state: GameState) {
        self.board = state.board;
        self.you = state.you;

        // Reset the movesets.
        self.moves = MoveSet::new();
    }

    /// Get the next move for the snake.
    pub fn get_move(&mut self) -> Move {
        // Update the engine mode.
        self.update_engine_mode();

        // Update the moveset with probabilities for safe moves.
        self.safe_moves();

        // Temporary safe move set to keep if we need to fall back to it.
        let safe_moves = self.moves;

        debug!("Safe moveset: {:?}", self.moves);

        // Update the moveset with probabilities for moves that will make the snake
        // live longer.
        self.hazard_moves();

        debug!("Hazard moveset: {:?}", self.moves);

        match self.mode {
            Mode::Hungry => self.hungry(),
            Mode::Scared => self.scared(),
        }

        debug!("Moveset: {:?}", self.moves);

        // Get a random move weighted by the moveset.
        match self
            .moves
            .as_vec()
            .choose_weighted(&mut self.rng, |(_, weight)| 1.0 / (1.0 + *weight))
        {
            // If everything is Ok, return the move.
            Ok((move_, _)) => *move_,

            // If all the moves have zero weight, return a random *safe* move.
            Err(WeightedError::AllWeightsZero) => {
                debug!("All moves have zero weight - choosing a random safe move");

                // Get a random move weighted by the moveset.
                match safe_moves
                    .as_vec()
                    .choose_weighted(&mut self.rng, |(_, weight)| 1.0 / (1.0 + *weight))
                {
                    // If everything is Ok, return the move.
                    Ok((move_, _)) => *move_,

                    Err(error) => {
                        debug!("No move available: {error}");

                        *Move::all()
                            .choose(&mut self.rng)
                            .expect("There should always be a move")
                    }
                }
            }

            Err(error) => {
                debug!("No move available: {error}");

                *Move::all()
                    .choose(&mut self.rng)
                    .expect("There should always be a move")
            }
        }
    }
}
