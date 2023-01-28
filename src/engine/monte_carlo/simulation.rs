use crate::objects::{moves::Move, point::Point, snake::SnakeID, GameState};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

/// A simulation of a game of Battlesnake.
pub struct Simulation {
    /// The random number generator.
    rng: rand::rngs::SmallRng,

    /// The state of the current game.
    state: GameState,

    /// A set of all snakes that ate food last turn.
    /// TODO: maybe make this more efficient?
    did_eat_food: HashSet<SnakeID>,

    /// Checks if we died last turn.
    did_die: bool,
}

/// Public API for the simulation - simulate the actual game!
impl Simulation {
    /// Create a new simulation.
    pub fn new(state: GameState) -> Self {
        Self {
            rng: rand::rngs::SmallRng::from_entropy(),
            state,
            did_eat_food: HashSet::new(),
            did_die: false,
        }
    }
}

/// Private API for the simulation - helper functions.
impl Simulation {
    /// Put food on the board randomly, according to game rules.
    fn put_food(&mut self) {
        if self.state.board.food.len() as u32 >= self.state.board.width * self.state.board.height {
        }
        // If there is not enough food. or if we randomly generate a number less than
        // the food spawn chance, put more food on the board.
        else if self.state.board.food.len() as u32
            <= self.state.game.ruleset.settings.minimum_food
            || self
                .rng
                .gen_bool(self.state.game.ruleset.settings.food_spawn_chance)
        {
            let point = self.random_point();
            self.state.board.food.insert(point);
        }
    }

    /// A random point on the board.
    fn random_point(&mut self) -> Point {
        let x = self.rng.gen_range(0..self.state.board.width as i32);
        let y = self.rng.gen_range(0..self.state.board.height as i32);
        (x, y).into()
    }

    /// Simulate the application of moves. Panics if the map from snake ID to move does
    /// not contain a move for all snakes in the game.
    fn apply_moves(&mut self, moves: HashMap<SnakeID, Move>) {
        // All the snakes on the board.
        // TODO: maybe make this more efficient?
        let snakes = self.state.board.snakes.clone();
        // All the snakes we remove from the board.
        let mut removed_snakes = HashSet::new();

        // First, update the turn counter.
        self.state.turn += 1;

        // Then, we apply all the moves to the snake bodies.
        for id in snakes.keys() {
            let move_ = moves
                .get(id)
                .unwrap_or_else(|| panic!("Missing move for snake '{}'", id));

            self.apply_move(id, move_);

            // If the snake is violating the rules, remove it from the board.
            if self.is_violating_rules(id) {
                removed_snakes.insert(id);
            }
        }

        // We remove all snakes at once because sometimes two snakes can collide and both
        // get removed.
        for id in removed_snakes {
            if id == &self.state.you.id {
                self.did_die = true;
            }

            self.state.board.snakes.remove(id);
            self.did_eat_food.remove(id);
        }
    }

    /// Apply a move to a snake - used for the Monte-Carlo simulation. Panics if the snake
    /// does not exist.
    fn apply_move(&mut self, id: &SnakeID, move_: &Move) {
        // Get the snake that we are moving.
        let snake = self
            .state
            .board
            .snakes
            .get_mut(id)
            .unwrap_or_else(|| panic!("Could not find snake '{}'", id));

        // Update the snake's head.
        snake.head = move_.to_point(&snake.head);

        // Update the snake's body.
        snake.body.insert(0, snake.head);

        // If the snake did not eat food during the previous move, remove the tail.
        if !self.did_eat_food.contains(&snake.id) {
            snake.body.pop();
        }

        // If the snake did not eat food during this turn, remove the id from the set.
        if !self.state.board.food.contains(&snake.head) {
            self.did_eat_food.remove(&snake.id);
            self.state.board.food.remove(&snake.head);
        }
    }

    /// Checks if a snake is violating any of the game rules. Panics if the snake
    /// does not exist.
    pub fn is_violating_rules(&self, id: &SnakeID) -> bool {
        // Get the snake that we are checking.
        let snake = self
            .state
            .board
            .snakes
            .get(id)
            .unwrap_or_else(|| panic!("Could not find snake '{}'", id));

        // Check if the snake is out of bounds.
        if !self.state.board.is_on_board(&snake.head) {
            return true;
        }

        // Check if the snake is colliding with any other snake's body, NOT including
        // any heads. Those are checked separately.
        if self
            .state
            .board
            .snakes
            .values()
            .map(|snake| &snake.body)
            .flat_map(|body| {
                body.iter()
                    .skip(1) // Skip checking the head.
                    .collect_vec()
            })
            .any(|point| point == &snake.head)
        {
            return true;
        }

        // If the snake is colliding with another snake's head which is not its own,
        // AND the other snake is equal or longer to it, then the snake is violating
        // the rules.
        if self
            .state
            .board
            .other_snakes(snake)
            .any(|other| other.head == snake.head && other.body.len() >= snake.body.len())
        {
            return true;
        }

        false
    }
}
