// This module is the code for the Monte Carlo tree search algorithm.

pub mod node;
pub mod simulation;

use itertools::Itertools;

use self::{node::Node, simulation::State};
use crate::objects::{moves::Move, GameState};

/// The Monte Carlo tree search algorithm.
#[derive(Debug, Clone)]
pub struct MonteCarlo {
    /// The current game state simulation.
    state: State,

    /// The root node of the search tree.
    root: Node,
}

impl MonteCarlo {
    /// Creates a new Monte Carlo tree search object.
    pub fn new(game_state: GameState) -> Self {
        Self {
            state: State::new(game_state),
            root: Node::new(None),
        }
    }
}

impl MonteCarlo {
    /// Returns the best path from the root to a leaf node, using the UCB1 algorithm.
    fn select(&self) -> Vec<Move> {
        // The current node we are searching.
        let mut current = &self.root;
        // The path from the root to the current node.
        let mut path = vec![];

        while !current.is_leaf() {
            let result = current.best_child();
            let best_child = *result.0;
            current = result.1;

            path.push(best_child);
        }

        path
    }

    /// Expands the given node at the end of the path, and returns a mutable reference to the new child node.
    fn expand(&mut self) -> &mut Node {
        let path = self.select();
        let mut current = &mut self.root;

        for move_ in path {
            current = current
                .children
                .get_mut(&move_)
                .expect("The child node should exist!");
        }

        let chosen_move = *Move::all()
            .iter()
            .find(|m| !current.children.keys().contains(m))
            .expect("This node should not be fully expanded!");

        current
            .children
            .insert(chosen_move, Node::new(Some(chosen_move)));

        current
    }

    /// Simulates a game from the given node, and returns if we won or not.
    fn simulate(&mut self, node: &Node) -> bool {
        todo!()
    }

    /// Backpropagates the result of the simulation up the tree.
    fn backpropagate(&mut self, mut path: Vec<Move>, won: bool) {
        let mut current = &mut self.root;

        while let Some(move_) = path.pop() {
            // Update the current node.
            current.visits += 1;
            current.wins += u32::from(won);
            current.parent_visits += 1;

            // Move to the next node.
            current = current
                .children
                .get_mut(&move_)
                .expect("The child node should exist!");
        }
    }
}
