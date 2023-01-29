// This module is responsible for the game simulation.
mod utils;

use crate::objects::{
    moves::Move,
    point::Point,
    settings::Ruleset,
    snake::{Snake, SnakeID},
    GameState,
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

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
    alive_snakes: HashMap<SnakeID, Rc<RefCell<Snake>>>,

    /// The set of all dead snakes on the board.
    dead_snakes: HashMap<SnakeID, Rc<RefCell<Snake>>>,

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
            alive_snakes: state
                .board
                .snakes
                .into_iter()
                .map(|(id, snake)| (id, Rc::new(RefCell::new(snake))))
                .collect(),
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
            // First, we apply the update.
            for (id, move_) in &update.moves {
                self.move_snake(*id, *move_);
            }

            // Then, we remove the snakes that have died.
            self.remove_dead_snakes();

            // Finally, we spawn food and hazards.
            self.spawn_food();
            self.spawn_hazards();

            // Increment the turn counter.
            self.turn += 1;
        }
    }
}

/// A custom `Clone` implementation for the simulation.
impl Clone for Simulation {
    /// A custom `Clone` implementation for the simulation. This is necessary because
    /// we don't want to clone the `Rc` and `RefCell` pointers, because that would
    /// clone the references to the snakes, which we don't want to do. We only want
    /// to clone the actual snakes. Therefore, when calling this, it is safe to do
    /// `let sim = sim.clone()`, because the `Rc` and `RefCell` pointers will not be
    /// cloned.
    fn clone(&self) -> Self {
        Self {
            turn: self.turn,
            height: self.height,
            width: self.width,
            alive_snakes: self
                .alive_snakes
                .iter()
                .map(|(id, snake)| (*id, Rc::new(RefCell::new(snake.borrow().clone()))))
                .collect(),
            dead_snakes: self
                .dead_snakes
                .iter()
                .map(|(id, snake)| (*id, Rc::new(RefCell::new(snake.borrow().clone()))))
                .collect(),
            food: self.food.clone(),
            hazards: self.hazards.clone(),
            ate_food: self.ate_food.clone(),
            rules: self.rules.clone(),
            snake_id: self.snake_id,
        }
    }
}
