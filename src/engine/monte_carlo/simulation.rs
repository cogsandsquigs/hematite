use crate::objects::{
    board::Board, moves::Move, point::Point, settings::Ruleset, snake::SnakeID, GameState,
};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

/// A simulation of a game of Battlesnake.
#[derive(Debug, Clone)]
pub struct Simulation {
    /// The random number generator.
    rng: rand::rngs::SmallRng,

    /// The rules of the game.
    rules: Ruleset,

    /// The board state.
    board: Board,

    /// The turn number.
    turn: u32,

    /// A set of all snakes that ate food last turn.
    /// TODO: maybe make this more efficient?
    did_eat_food: HashSet<SnakeID>,
}

/// Public API for the simulation - simulate the actual game!
impl Simulation {
    /// Create a new simulation.
    pub fn new(state: GameState) -> Self {
        Self {
            rng: rand::rngs::SmallRng::from_entropy(),
            rules: state.game.ruleset,
            board: state.board,
            turn: state.turn,
            did_eat_food: HashSet::new(),
        }
    }

    /// Run a step of the simulation of the game.
    pub fn step(&mut self, moves: &HashMap<SnakeID, Move>) {
        // If the game is not over, simulate a turn.
        if !self.is_game_over() {
            self.apply_moves(moves);
            self.put_food();
        }
    }

    /// Run a random step of the simulation of the game.
    pub fn random_step(&mut self) {
        let moves = self.random_moves();
        self.step(&moves);
    }
}

/// Private API for the simulation - helper functions.
impl Simulation {
    /// Checks if the game is over.
    fn is_game_over(&self) -> bool {
        // If there is only 1 (or 0) snakes left, the game is over.
        if self.board.snakes.len() < 2 {
            return true;
        }

        false
    }

    /// A random point on the board.
    fn random_point(&mut self) -> Point {
        let x = self.rng.gen_range(0..self.board.width as i32);
        let y = self.rng.gen_range(0..self.board.height as i32);
        (x, y).into()
    }

    /// A random set of moves for all snakes.
    fn random_moves(&mut self) -> HashMap<SnakeID, Move> {
        self.board
            .snakes
            .keys()
            .map(|id| (*id, Move::random(&mut self.rng)))
            .collect()
    }

    /// Put food on the board randomly, according to game rules.
    fn put_food(&mut self) {
        if self.board.food.len() as u32 >= self.board.width * self.board.height {
        }
        // If there is not enough food. or if we randomly generate a number less than
        // the food spawn chance, put more food on the board.
        else if self.board.food.len() as u32 <= self.rules.settings.minimum_food
            || self.rng.gen_bool(self.rules.settings.food_spawn_chance)
        {
            let point = self.random_point();
            self.board.food.insert(point);
        }
    }

    /// Simulate the application of moves. Panics if the map from snake ID to move does
    /// not contain a move for all snakes in the game.
    fn apply_moves(&mut self, moves: &HashMap<SnakeID, Move>) {
        // All the snakes on the board.
        // TODO: maybe make this more efficient?
        let snakes = self.board.snakes.clone();
        // All the snakes we remove from the board.
        let mut removed_snakes = HashSet::new();

        // First, update the turn counter.
        self.turn += 1;

        // Then, we apply all the moves to the snake bodies.
        for id in snakes.keys() {
            let move_ = moves
                .get(id)
                .unwrap_or_else(|| panic!("Could not find move for snake '{}'", id));

            self.apply_move(id, move_);

            // If the snake is violating the rules, remove it from the board.
            if self.is_violating_rules(id) {
                removed_snakes.insert(id);
            }
        }

        // We remove all snakes at once because sometimes two snakes can collide and both
        // get removed.
        for id in removed_snakes {
            self.board.snakes.remove(id);
            self.did_eat_food.remove(id);
        }
    }

    /// Apply a move to a snake - used for the Monte-Carlo simulation. Panics if the snake
    /// does not exist.
    fn apply_move(&mut self, id: &SnakeID, move_: &Move) {
        // Get the snake that we are moving.
        let snake = self
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

        // If the snake did not eat food during this turn, remove the id from the set and decrement
        // the health.
        if !self.board.food.contains(&snake.head) {
            // Remove it here so that we don't remove it twice.
            self.did_eat_food.remove(&snake.id);
            self.board.food.remove(&snake.head);

            snake.health -= if self.board.hazards.contains(&snake.head) {
                self.rules.settings.hazard_damage_per_turn
            } else {
                1
            };
        }
        // If the snake did eat food, add the id to the set and reset health.
        else {
            self.did_eat_food.insert(snake.id);
            snake.health = 100;
            snake.length += 1;
        }
    }

    /// Checks if a snake is violating any of the game rules. Panics if the snake
    /// does not exist.
    pub fn is_violating_rules(&self, id: &SnakeID) -> bool {
        // Get the snake that we are checking.
        let snake = self
            .board
            .snakes
            .get(id)
            .unwrap_or_else(|| panic!("Could not find snake '{}'", id));

        // Check if the snake is out of bounds.
        if !self.board.is_on_board(&snake.head) {
            return true;
        }

        // Check if the snake is colliding with any other snake's body, NOT including
        // any heads. Those are checked separately.
        if self
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
            .board
            .other_snakes(snake)
            .any(|other| other.head == snake.head && other.body.len() >= snake.body.len())
        {
            return true;
        }

        false
    }
}
