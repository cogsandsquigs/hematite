mod simulation;

use self::simulation::Simulation;
use crate::objects::{moves::Move, snake::SnakeID, GameState};

/// A node in the Monte-Carlo search tree.
struct MCTSNode {
    /// The simulation of the game.
    simulation: Simulation,

    /// The current stack of moves we are planning to make. This stack is
    /// ordered from first to last move.
    current_move: Vec<Move>,

    /// The snake we are playing as
    snake_id: SnakeID,
}

impl MCTSNode {
    /// Create a new MCTS node.
    fn new(state: GameState, current_move: Move, snake_id: SnakeID) -> Self {
        Self {
            simulation: Simulation::new(state),
            current_move: vec![current_move],
            snake_id,
        }
    }

    /// Create a child node of this node.
    fn child(&self, move_: Move) -> Self {
        let mut current_move = self.current_move.clone();
        current_move.push(move_);

        Self {
            simulation: self.simulation.clone(),
            current_move,
            snake_id: self.snake_id,
        }
    }

    /// Run a random simulation of the game, and return the score. The final state
    /// of the simulation is discarded, returning only the score. Note that because
    /// of this, this can be run in parallel.
    fn random_simulation(&self) -> i32 {
        // TODO: maybe make this more efficient?
        let mut simulation = self.simulation.clone();

        for move_ in &self.current_move {
            let mut moves = simulation.random_moves();
            moves.insert(self.snake_id, *move_);

            simulation.step(&moves);
        }

        // Now, run the random game.
        simulation.random_game(&self.snake_id)
    }
}
