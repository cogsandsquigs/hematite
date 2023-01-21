use crate::game::moves::Move;
use std::array::IntoIter;

/// The set of moves that a snake can make. These are all weighted using floats.
/// The higher the weight, the more likely the snake is to make that move. The
/// order of the moves is the same as the order of the moves in the `Move` enum.
/// The sum of all the weights should be 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveSet {
    pub up: f32,
    pub down: f32,
    pub left: f32,
    pub right: f32,
}

impl MoveSet {
    /// Returns the set of all possible moves.
    pub fn new() -> Self {
        Self {
            up: 1.,
            down: 1.,
            left: 1.,
            right: 1.,
        }
    }

    /// Multiplies all the weights by a given factor.
    pub fn multiply(&mut self, move_: Move, factor: f32) {
        match move_ {
            Move::Up => self.up *= factor,
            Move::Down => self.down *= factor,
            Move::Left => self.left *= factor,
            Move::Right => self.right *= factor,
        }
    }

    /// Adds a tuple or array of tuples to all the nonzero weights. This is necessary
    /// because if a weight is zero, we have declared it to be an impossible move.
    pub fn add_nonzero(&mut self, weights: impl IntoIterator<Item = (Move, f32)>) {
        for (move_, weight) in weights {
            match move_ {
                Move::Up => {
                    if self.up != 0. {
                        self.up += weight;
                    }
                }
                Move::Down => {
                    if self.down != 0. {
                        self.down += weight;
                    }
                }
                Move::Left => {
                    if self.left != 0. {
                        self.left += weight;
                    }
                }
                Move::Right => {
                    if self.right != 0. {
                        self.right += weight;
                    }
                }
            }
        }
    }

    /// Sums all the weights.
    pub fn sum(&self) -> f32 {
        self.up + self.down + self.left + self.right
    }

    /// Turns the moveset into a vector of moves.
    pub fn as_vec(&self) -> Vec<(Move, f32)> {
        self.into_iter().collect()
    }
}

impl IntoIterator for MoveSet {
    type Item = (Move, f32);

    type IntoIter = IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            (Move::Up, self.up),
            (Move::Down, self.down),
            (Move::Left, self.left),
            (Move::Right, self.right),
        ]
        .into_iter()
    }
}

impl Default for MoveSet {
    fn default() -> Self {
        Self::new()
    }
}

impl From<[f32; 4]> for MoveSet {
    fn from(weights: [f32; 4]) -> Self {
        Self {
            up: weights[0],
            down: weights[1],
            left: weights[2],
            right: weights[3],
        }
    }
}

impl From<MoveSet> for [f32; 4] {
    fn from(moveset: MoveSet) -> Self {
        [moveset.up, moveset.down, moveset.left, moveset.right]
    }
}

impl From<MoveSet> for (f32, f32, f32, f32) {
    fn from(moveset: MoveSet) -> Self {
        (moveset.up, moveset.down, moveset.left, moveset.right)
    }
}
