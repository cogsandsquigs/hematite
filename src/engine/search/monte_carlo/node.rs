use crate::objects::moves::Move;
use std::collections::HashMap;

/// A node in the Monte Carlo search tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// The move that led to this node.
    move_: Option<Move>,

    /// The number of visits the parent node has made.
    pub parent_visits: u32,

    /// The number of visits to this node.
    pub visits: u32,

    /// The number of wins for this node.
    pub wins: u32,

    /// The children of this node.
    pub children: HashMap<Move, Node>,
}

/// Public API for the node.
impl Node {
    /// Create a new node.
    pub fn new(move_: Option<Move>) -> Self {
        Self {
            parent_visits: 0,
            visits: 0,
            wins: 0,
            move_,
            children: HashMap::new(),
        }
    }

    /// Returns a mutable reference to the child node with the given move path.
    pub fn get_child(&mut self, path: &[Move]) -> &mut Node {
        let mut current = self;

        for move_ in path {
            current = current
                .children
                .get_mut(move_)
                .expect("The child node should exist!");
        }

        current
    }

    /// Checks if this node is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty() && !self.is_fully_expanded()
    }

    /// Checks if this node is fully expanded.
    pub fn is_fully_expanded(&self) -> bool {
        self.children.len() == 4
    }

    /// Gets the best child of this node, using the UCB1 algorithm. Panics
    /// if this node has no children.
    pub fn best_child(&self) -> (&Move, &Node) {
        self.children
            .iter()
            .max_by(|(_, a), (_, b)| a.ucb1().total_cmp(&b.ucb1()))
            .expect("This node should have children!")
    }

    /// Get the UCB1 value for this node. This tells us how good this node is, and
    /// how much we want to explore it. The higher the value, the more we should explore
    /// it.
    pub fn ucb1(&self) -> f64 {
        // The exploitation term is the ratio of wins to visits. This is the
        // (rough) probability of winning from this node.
        let exploitation = self.wins as f64 / self.visits as f64;

        // The exploration term is a measure of how much we want to explore this
        // node. As we visit this node more and more, we want to explore less and
        // less, and vice versa.
        // TODO: replace the 2.0 with a configurable parameter.
        let exploration = (2.0 * (self.parent_visits as f64).ln() / self.visits as f64).sqrt();

        exploitation + exploration
    }
}
