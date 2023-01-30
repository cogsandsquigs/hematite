// This module is responsible for the game simulation.
mod utils;

use crate::objects::{
    moves::Move,
    point::Point,
    settings::Ruleset,
    snake::{Snake, SnakeID},
    GameState,
};
use itertools::Itertools;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use std::collections::{HashMap, HashSet};

/// An update to the game state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    /// The set of all moves to be made.
    pub moves: HashMap<SnakeID, Move>,
}

impl Update {
    /// Create a new update.
    pub fn new(moves: HashMap<SnakeID, Move>) -> Self {
        Self { moves }
    }
}

/// The actual game state itself.
#[derive(Debug, Clone)]
pub struct Simulation {
    // The snake we're controlling.
    pub snake_id: SnakeID,

    /// The number of turns that have passed.
    turn: u32,

    /// The height of the board.
    height: u32,

    /// The width of the board.
    width: u32,

    /// The set of all living snakes on the board.
    alive_snakes: HashMap<SnakeID, Snake>,

    /// The set of all dead snakes on the board.
    dead_snakes: HashMap<SnakeID, Snake>,

    /// The set of all food on the board.
    food: HashSet<Point>,

    /// The set of all hazards on the board.
    hazards: HashSet<Point>,

    /// The set of all snakes who ate food in the last turn.
    ate_food: HashSet<SnakeID>,

    /// The rules of the game.
    rules: Ruleset,
}

/// Public API for the game simulation.
impl Simulation {
    /// Create a new game simulation.
    pub fn new(state: GameState) -> Self {
        Self {
            turn: state.turn,
            height: state.board.height,
            width: state.board.width,
            alive_snakes: state.board.snakes,
            dead_snakes: HashMap::new(),
            food: state.board.food,
            hazards: state.board.hazards,
            // TODO: This is wrong. We need to check if the snakes ate food in the
            // last turn. How do we do that?
            ate_food: HashSet::new(),
            rules: state.game.ruleset,
            snake_id: state.you.id,
        }
    }

    /// Applies a game update to the simulation.
    pub fn apply_update(&mut self, update: &Update) {
        // If the game is over, we don't want to apply any updates.
        if !self.is_over() {
            let alive_snakes = self.alive_snakes.keys().copied().collect_vec();

            // Move all the snakes.
            for snake_id in alive_snakes {
                let move_ = update.moves.get(&snake_id).unwrap_or(&Move::Up);
                self.move_snake(snake_id, *move_);
            }

            // Then, we remove the snakes that have died.
            self.remove_dead_snakes();

            // Finally, we spawn food.
            self.spawn_food();
            // TODO: Spawn hazards

            // Increment the turn counter.
            self.turn += 1;
        }
    }

    /// Runs a random game with good moves until the game is over. Returns if we won.
    pub fn run_random_game(&mut self) -> bool {
        let mut rng = SmallRng::from_entropy();

        while !self.is_over() {
            // Apply a random update. Set `use_state` to true because we can consider
            // the state of the board when making a move during a random rollout.
            let possible_updates = self.possible_updates(true);
            let update = possible_updates.choose(&mut rng).unwrap();

            self.apply_update(update);
        }

        self.did_win()
    }

    /// Gets all the possible updates that can be made, including those for dead
    /// snakes.
    pub fn possible_updates(&self, use_state: bool) -> Vec<Update> {
        self.snakes()
            .map(|snake| {
                let good_moves = self
                    .good_moves(snake, use_state)
                    .into_iter()
                    .map(|move_| (snake.id, move_))
                    .collect_vec();

                // If there are no good moves, we use the previous move.
                if good_moves.is_empty() {
                    vec![(snake.id, snake.previous_move())]
                }
                // Otherwise, we use the good moves.
                else {
                    good_moves
                }
            })
            .multi_cartesian_product()
            .map(|moves| Update::new(moves.into_iter().collect()))
            .collect_vec()
    }
}
