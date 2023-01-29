// This is the main code for the Monte Carlo search. It is responsible for the
// tree search and the tree update.

mod game;
mod node;
mod utils;

use self::{game::Simulation, node::Node};
use crate::objects::GameState;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// The Monte-Carlo search tree.
pub struct MonteCarlo {
    /// The game simulation.
    state: Simulation,

    /// The root node of the tree. using a `Rc` and `RefCell` allows us to have
    /// multiple references to the same node.
    root: Node,
}

/// Public API for the Monte Carlo search tree.
impl MonteCarlo {
    /// Create a new Monte Carlo search tree.
    pub fn new(state: GameState) -> Self {
        Self {
            state: Simulation::new(state),
            root: Node::new(HashMap::new()),
        }
    }

    /// Update the tree with a new game state. It first updates the simulation with
    /// the new game state, then prunes the tree by placing the node that matches
    /// the new game state as the root node, and removing all other top-level nodes.
    /// If such a node does not exist, then the tree is reset.
    pub fn update(&mut self, state: GameState) {
        self.state = Simulation::new(state);

        todo!()
    }

    // /// Gets the evaluation of all the moves.
    // pub fn move_scores(&self) -> HashMap<Move, f64> {
    //     Move::all()
    //         .into_iter()
    //         .map(|move_| (move_, self.move_score(&move_)))
    //         .collect()
    // }

    // /// Gets the evaluation of any move. Panics if the root node has no children.
    // pub fn move_score(&self, move_: &Move) -> f64 {
    //     // Gets all the child nodes that have the same move as the one we're evaluating.
    //     let children = self
    //         .children(&self.root.borrow())
    //         .expect("The root should have children!")
    //         .iter()
    //         .filter(|child| {
    //             child
    //                 .borrow()
    //                 .update
    //                 .moves
    //                 .iter()
    //                 .any(|(id, m)| id == &self.state.snake_id && m == move_)
    //         })
    //         .collect_vec();

    //     // If there are no children with the same move, then we haven't visited this
    //     // move yet. Return 0.0.
    //     if children.is_empty() {
    //         0.0
    //     }
    //     // Otherwise, calculate the average win rate of all the children.
    //     else {
    //         let (wins, visits) = children.iter().fold((0, 0), |(wins, visits), child| {
    //             (wins + child.borrow().wins, visits + child.borrow().visits)
    //         });

    //         wins as f64 / visits as f64
    //     }
    // }
}

/// Private API for the Monte Carlo search tree.
impl MonteCarlo {
    /// Run one round of the Monte Carlo search.
    fn run_round(&mut self) {
        // Recursively descend the tree until we reach a leaf node.
        self.select(&self.root);
    }

    /// Recursively descend the tree until we reach a leaf node, and expand it.
    fn select(&mut self, node: &Node) {
        // If the node is a leaf node, then expand it.
        if self.is_leaf(node) {
            // self.expand(node);
            todo!()
        }
        // Otherwise, recursively select the best child node.
        else {
            let child = self
                .best_child(node)
                .expect("The node should have children!");
            self.select(&child);
        }
    }

    // /// The first stage of MCTS. Searches for a leaf node in the tree, and returns
    // /// the stack of nodes that we traversed to get to the leaf node, and a mutable
    // /// reference to the leaf node itself.
    // fn select(&mut self) -> (Vec<Rc<RefCell<Node>>>, Rc<RefCell<Node>>) {
    //     // The current path to the leaf node.
    //     let mut path = Vec::new();
    //     // The current node we're looking at.
    //     let mut current = self.root.clone();

    //     // While the current node is not a leaf node, keep traversing the tree.
    //     while self.is_leaf(&current.borrow()) {
    //         // Get the best child node.
    //         let child = self.best_child(&current.borrow()).expect("");

    //         // Add the current node to the path.
    //         path.push(current);

    //         // Set the current node to the child node.
    //         current = child;
    //     }

    //     // Return the path and the leaf node.
    //     (path, current)
    // }

    // /// The second stage of MCTS. Expands the leaf node by creating all the
    // /// possible children of the leaf node, or, if the leaf node already has
    // /// children, then it randomly selects one of the non-evaluated children.
    // /// Returns a mutable reference to the expanded node.
    // fn expand(
    //     &mut self,
    //     path: Vec<&Rc<RefCell<Node>>>,
    //     leaf: &Rc<RefCell<Node>>,
    // ) -> Rc<RefCell<Node>> {
    //     // If the node has no children, then create all the children.
    //     if self.children(&leaf.borrow()).is_some() {
    //         todo!()
    //     }

    //     // Select one of the non-evaluated children. We can do this by randomly
    //     // selecting a child, and then checking if it has been evaluated. However,
    //     // its more efficient to just select the first child that hasn't been
    //     // evaluated.
    //     let children = self
    //         .children(&leaf.borrow())
    //         .expect("The leaf should have children!");

    //     // Get the first child that hasn't been evaluated.
    //     let child = children
    //         .iter()
    //         .find(|child| !child.borrow().is_visited())
    //         .expect("For a leaf node to be a leaf, some children must not have been visited!");

    //     // Return the child.
    //     child.clone()
    // }
}
