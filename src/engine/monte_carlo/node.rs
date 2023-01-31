use super::game::Simulation;
use crate::{configuration::mcts::MCTSConfig, objects::moves::Move};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// The different states a node can be in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeState {
    /// The node has not yet been visited/expanded. We need to create its children
    /// and then explore them.
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

    /// The first step in MCTS. Recursively descend the tree until we reach a leaf node,
    /// and then continue on with the other steps (expansion, simulation, backtracking).
    /// Returns if we won or not.
    /// TODO: Limit max depth?
    pub fn select(&mut self, config: &MCTSConfig, mut simulation: Simulation, root: bool) -> u32 {
        // Update the visits here for succinctness
        self.visits += config.games_per_search;

        // If this is not the root node, then we need to apply the move to the simulation. We
        // can't apply the root node move because the root node is really just a placeholder
        // for the current state, and the move is not actually a move.
        if !root {
            // Apply the move to the simulation
            simulation.apply_move(&self.move_);
        }

        let wins = match self.state {
            // If this node has been fully expanded, then we search down it to find the best
            // child to expand.
            NodeState::Expanded => {
                // Get the best child
                let best_child = self
                    .best_child(self.visits)
                    .expect("This node should have children!");

                best_child.select(config, simulation, false)
            }

            // If we have not visited this node yet, then we initialize it and
            // expand it.
            NodeState::Leaf => {
                // Set the state to visited
                self.state = NodeState::Visited;

                // Create all the children
                self.children = simulation.snake_moves().map(Node::new).collect();

                self.expand(config, simulation)
            }

            // If we have already visited, then expand this node more.
            NodeState::Visited => self.expand(config, simulation),
        };

        // Backpropigate the result
        self.wins += wins;

        wins
    }

    /// Second step in MCTS. Choose a node to expand, and expand it - i.e. run a simulation
    /// from this node and backpropigate the result. Returns if we won or not.
    pub fn expand(&mut self, config: &MCTSConfig, simulation: Simulation) -> u32 {
        // Choose the first child which is a leaf to expand.
        let child = self
            .children
            .iter_mut()
            .find(|child| child.state == NodeState::Leaf)
            .expect("This node should have children!");

        let wins = child.simulate(config, simulation);

        child.visits += config.games_per_search;
        child.wins += wins;

        // If all the children have been visited once, then we can set the state to expanded.
        if self.children.iter().all(|child| child.visits > 0) {
            self.state = NodeState::Expanded;
        }

        wins
    }

    /// Third step of MCTS. Apply the child move, run a simulation from this node and backpropigate the result.
    /// Returns how many times we won.
    pub fn simulate(&mut self, config: &MCTSConfig, mut simulation: Simulation) -> u32 {
        // Apply the move to the simulation
        simulation.apply_move(&self.move_);

        // Run the simulation
        let wins: u32 = (0..config.games_per_search)
            .into_par_iter()
            // Run the simulation
            .map(|_| simulation.clone().run_random_game())
            .map(u32::from)
            .sum();

        // Backpropigate the result
        self.visits += config.games_per_search;
        self.wins += wins;

        wins
    }
}
