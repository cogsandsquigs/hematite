// This is the main code for the Monte Carlo search. It is responsible for the
// tree search and the tree update.

mod game;
mod node;

use self::{game::Simulation, node::Node};
use crate::{
    configuration::mcts::MCTSConfig,
    objects::{moves::Move, GameState},
};
use std::collections::HashMap;

/// The Monte-Carlo search tree.
#[derive(Debug, Clone)]
pub struct MonteCarlo {
    // The configuration for the Monte Carlo search tree.
    config: MCTSConfig,

    /// The game simulation.
    state: Simulation,

    /// The root node of the tree.
    root: Node,
}

/// Public API for the Monte Carlo search tree.
impl MonteCarlo {
    /// Create a new Monte Carlo search tree. Since the root node is not considered
    /// an actual move, it is represented by an empty node.
    pub fn new(state: GameState, config: MCTSConfig) -> Self {
        Self {
            state: Simulation::new(state),
            root: Node::empty(),
            config,
        }
    }

    /// Update the tree with a new game state. It first updates the simulation with
    /// the new game state, then prunes the tree by placing the node that matches
    /// the new game state as the root node, and removing all other top-level nodes.
    /// If such a node does not exist, then the tree is reset.
    pub fn update(&mut self, state: GameState) {
        self.state = Simulation::new(state);

        // TODO: prune tree
        self.root = Node::empty();
    }

    /// Runs the Monte Carlo search for a given amount of iterations.
    pub fn search(&mut self, iterations: u32) {
        for _ in 0..iterations {
            self.run_round();
        }
    }

    /// Gets the number of wins
    pub fn wins(&self) -> u32 {
        self.root.wins
    }

    /// Gets the number of visits
    pub fn visits(&self) -> u32 {
        self.root.visits
    }

    /// Gets the evaluation of all the moves.
    pub fn move_scores(&self) -> HashMap<Move, f64> {
        Move::all()
            .into_iter()
            .map(|move_| (move_, self.move_score(&move_)))
            .collect()
    }

    /// Gets the evaluation of any move.
    pub fn move_score(&self, move_: &Move) -> f64 {
        // Gets the child node that has the same move as the one we're evaluating.
        let child = self
            .root
            .children
            .iter()
            // Since there should only be one child with a given move, we can use
            // `find` instead of `filter`.
            .find(|child| &child.move_ == move_);

        // If such a child exists, return the evaluation of the move.
        if let Some(Node { wins, .. }) = child {
            *wins as f64 / self.root.visits as f64
        }
        // Otherwise, if such a child does not exist, return 0.
        else {
            0.0
        }
    }
}

/// Private API for the Monte Carlo search tree.
impl MonteCarlo {
    /// Run one round of the Monte Carlo search.
    fn run_round(&mut self) {
        // Create a copy of the simulation to play on.
        let simulation = self.state.clone();

        // Recursively descend the tree until we reach a leaf node.
        self.root.select(&self.config, simulation, true);
    }
}
