use super::game::Simulation;
use crate::objects::moves::Move;
use rayon::prelude::*;

/// The different states a node can be in.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum NodeState {
    /// The node has not yet been visited/expanded. We need to create its children
    /// and then explore them.
    #[default]
    Leaf,

    /// The node has been visited, but not fully expanded. We still need to explore
    /// some of its children.
    Visited,

    /// The node has been fully expanded. We can now select the best child to explore.
    Expanded,
}

/// A node in the Monte Carlo search tree. It contains the moves that all the snakes have
/// made at this node, and contains its children.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    /// The move that lead to this node.
    pub move_: Move,

    /// The number of times this node has been visited.
    pub visits: u32,

    /// The total amount of wins for this node.
    pub wins: u32,

    /// The state of the node: What we can/should do with it.
    pub state: NodeState,

    /// The children of this node.
    pub children: Vec<Node>,
}

impl Node {
    /// Create a new node.
    pub fn new(move_: Move) -> Self {
        Self {
            visits: 0,
            wins: 0,
            state: NodeState::Leaf,
            move_,
            children: Vec::new(),
        }
    }

    /// Create an empty node. By default, an empty node is a leaf node.
    pub fn empty() -> Self {
        Self {
            visits: 0,
            wins: 0,
            state: NodeState::Leaf,
            move_: Move::Up,
            children: Vec::new(),
        }
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

    /// Runs a simulation of the game at this node `n` times and returns the number of wins.
    /// Assumes that all previous moves have already been applied to `simulation` before
    /// passing it on to this function.
    ///
    /// Note that this function is parallelized using rayon, to speed up the simulation runs.
    pub fn simulate_n_times(&mut self, mut simulation: Simulation, n: u32) -> u32 {
        simulation.apply_move(&self.move_);

        (0..n)
            .into_par_iter()
            .map(|_| simulation.clone().run_random_game())
            .map(u32::from)
            .sum()
    }
}
