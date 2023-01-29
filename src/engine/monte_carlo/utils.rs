use super::{node::Node, MonteCarlo};
use std::{cell::RefCell, rc::Rc};

/// Utilities for the Monte Carlo engine.
impl MonteCarlo {
    // /// Checks if this node is a leaf node. A leaf node is a node that either has no
    // /// children, or not all of its children have been visited yet.
    // pub fn is_leaf(&self, node: &Node) -> bool {
    //     if let Some(children) = self.tree.get(&node.update) {
    //         !children.iter().all(|child| child.borrow().is_visited())
    //     } else {
    //         true
    //     }
    // }

    // /// Gets the children of a given node.
    // pub fn children(&self, node: &Node) -> Option<&Vec<Rc<RefCell<Node>>>> {
    //     self.tree.get(&node.update)
    // }

    // /// Gets the best child node of a given node.
    // pub fn best_child(&self, node: &Node) -> Option<Rc<RefCell<Node>>> {
    //     if let Some(children) = self.children(node) {
    //         children
    //             .iter()
    //             .max_by(|a, b| {
    //                 a.borrow()
    //                     .ucb1(node.visits)
    //                     .total_cmp(&b.borrow().ucb1(node.visits))
    //             })
    //             .cloned()
    //     } else {
    //         None
    //     }
    // }
}
