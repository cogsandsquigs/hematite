// This is the main code for the Monte Carlo search. It is responsible for the
// tree search and the tree update.

mod game;
mod node;

use std::collections::HashMap;

use itertools::Itertools;

use self::{game::Simulation, node::Node};
use crate::objects::{moves::Move, GameState};

/// The Monte-Carlo search tree.
#[derive(Debug, Clone)]
pub struct MonteCarlo {
    /// The game simulation.
    state: Simulation,

    /// The root node of the tree.
    root: Node,
}

/// Public API for the Monte Carlo search tree.
impl MonteCarlo {
    /// Create a new Monte Carlo search tree.
    pub fn new(state: GameState) -> Self {
        Self {
            state: Simulation::new(state),
            root: Node::empty(),
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

    /// Gets the evaluation of all the moves.
    pub fn move_scores(&self) -> HashMap<Move, f64> {
        Move::all()
            .into_iter()
            .map(|move_| (move_, self.move_score(&move_)))
            .collect()
    }

    /// Gets the evaluation of any move. Panics if the root node has no children.
    pub fn move_score(&self, move_: &Move) -> f64 {
        // Gets all the child nodes that have the same move as the one we're evaluating.
        let children = self
            .root
            .children
            .iter()
            .filter(|child| {
                child.update.moves.iter().any(|(id, m)| {
                    println!(
                        "{:?} == {:?} && {:?} == {:?}",
                        id, self.state.snake_id, m, move_
                    );
                    id == &self.state.snake_id && m == move_
                })
            })
            .collect_vec();

        // If there are no children with the same move, then we haven't visited this
        // move yet. Return 0.0.
        if children.is_empty() {
            0.0
        }
        // Otherwise, calculate the average win rate of all the children.
        else {
            (children.iter().map(|child| child.wins).sum::<u32>() as f64) / self.root.visits as f64
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
        self.root.select(simulation);
    }
}
