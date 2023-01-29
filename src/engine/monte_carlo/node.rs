use crate::objects::{moves::Move, snake::SnakeID};
use std::collections::HashMap;

use super::game::Update;

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
}
