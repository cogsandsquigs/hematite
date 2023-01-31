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
use std::collections::{HashMap, HashSet};

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

    /// Applies a game move we make to the simulation. All other snakes are moved
    /// randomly.
    pub fn apply_move(&mut self, move_: &Move) {
        // If the game is over, we don't want to apply any updates.
        if !self.is_over() {
            let alive_snakes = self.alive_snakes.keys().copied().collect_vec();

            // Move all the snakes.
            for snake_id in alive_snakes {
                // If this snake is us, we apply the move we made.
                if snake_id == self.snake_id {
                    self.move_snake(snake_id, *move_);
                }
                // Otherwise, we move the snake using a good move.
                else {
                    self.move_snake(snake_id, self.random_good_move(&snake_id));
                }
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

    /// Gets all the good moves we can make.
    pub fn allowed_moves(&self) -> impl Iterator<Item = Move> {
        let snake = self.alive_snakes.get(&self.snake_id);

        // If we are alive, we return all the good moves we can make.
        if let Some(snake) = snake {
            self.good_moves(snake, true).into_iter()
        }
        // Otherwise, we return an empty iterator. We can't make any moves :(.
        else {
            vec![].into_iter()
        }
    }

    /// Runs a random game with the current state. Returns if we won or not.
    pub fn run_random_game(&self) -> bool {
        let mut simulation = self.clone();

        while !simulation.is_over() {
            let move_ = simulation.random_good_move(&simulation.snake_id);
            simulation.apply_move(&move_);
        }

        simulation.did_win()
    }
}
