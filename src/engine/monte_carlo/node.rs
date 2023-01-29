use itertools::Itertools;

use super::game::{Simulation, Update};
use crate::objects::{moves::Move, snake::SnakeID};
use std::collections::{HashMap, HashSet};

/// The different states a node can be in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeState {
    /// The node has not yet been visited/expanded.
    Leaf,

    /// The node has been visited, but not fully expanded.
    Expandable,

    /// The node has been fully expanded.
    Expanded,
}

/// A node in the Monte Carlo search tree. It contains the moves that all the snakes have
/// made at this node, and contains its children.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    /// The game update that led to this node.
    pub update: Update,

    /// The number of times this node has been visited.
    pub visits: u32,

    /// The total amount of wins for this node.
    pub wins: u32,

    /// The state of the node: What we can/should do with it.
    pub state: NodeState,

    /// The maximum amount of different child game states allowed at this node.
    pub max_updates: usize,

    /// The children of this node.
    pub children: Vec<Node>,
}

impl Node {
    /// Create a new node.
    pub fn new(moves: HashMap<SnakeID, Move>) -> Self {
        Self {
            visits: 0,
            wins: 0,
            update: Update::new(moves.into_iter().collect()),
            children: Vec::new(),
            max_updates: 0,
            state: NodeState::Leaf,
        }
    }

    /// Check if this node has been visited.
    pub fn is_visited(&self) -> bool {
        self.visits > 0
    }

    /// Get the UCB1 value for this node.
    pub fn ucb1(&self, parent_visits: u32) -> f64 {
        // The ratio of wins to visits.
        let win_rate = self.wins as f64 / self.visits as f64;

        // The exploration term. This is a measure of how much we want to explore
        // this node in the tree. As we visit this node more, we want to explore it
        // less.
        // TODO: Maybe make the '2.0' term a configurable parameter?
        let exploration = (2.0 * (parent_visits as f64).ln() / self.visits as f64).sqrt();

        win_rate + exploration
    }

    /// Gets the best child based on the UCB1 score.
    pub fn best_child(&mut self, parent_visits: u32) -> Option<&mut Node> {
        self.children
            .iter_mut()
            .max_by(|a, b| a.ucb1(parent_visits).total_cmp(&b.ucb1(parent_visits)))
    }

    /// The first step in MCTS. Recursively descend the tree until we reach a leaf node,
    /// and then continue on with the other steps (expansion, simulation, backtracking).
    /// Returns if we won or not.
    /// TODO: Limit max depth?
    pub fn select(&mut self, mut simulation: Simulation, depth: u32) -> bool {
        // Update the visits here for succinctness
        self.visits += 1;
        // Apply the move to the simulation
        simulation.apply_update(&self.update);

        let did_win = match self.state {
            // If this node is not a leaf or expandable node, we keep searching.
            NodeState::Expanded => {
                // Get the best child
                let best_child = self
                    .best_child(self.visits)
                    .expect("This node should have children!");

                best_child.select(simulation, depth + 1)
            }

            // If we have not visited this node yet, then we initialize it and
            // expand it.
            NodeState::Leaf => {
                todo!();

                self.expand(simulation)
            }

            // If we have already started to expand this node, then expand this node.
            NodeState::Expandable => self.expand(simulation),
        };

        if did_win {
            self.wins += 1;
        }

        did_win
    }

    /// Second step in MCTS. Expand the node and then simulate an outcome. Returns if we
    /// won or not.
    pub fn expand(&mut self, simulation: Simulation) -> bool {
        todo!()
    }
}
