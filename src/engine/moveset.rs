use crate::game::moves::Move;
use std::array::IntoIter;

/// The set of moves that a snake can make. These all have a risk factor associated with
/// them. The higher the risk, the less likely the snake will make that move. The order
/// of the moves is the same as the order of the moves in the `Move` enum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveSet {
    up: f32,
    down: f32,
    left: f32,
    right: f32,
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
    pub fn multiply(&mut self, move_: &Move, risk: f32) {
        match move_ {
            Move::Up => self.up *= risk,
            Move::Down => self.down *= risk,
            Move::Left => self.left *= risk,
            Move::Right => self.right *= risk,
        }
    }

    /// Adds a tuple or array of tuples to all the weights.
    pub fn add(&mut self, move_: Move, risk: f32) {
        match move_ {
            Move::Up => self.up += risk,
            Move::Down => self.down += risk,
            Move::Left => self.left += risk,
            Move::Right => self.right += risk,
        }
    }

    // Turns all other moves to infinity. Effectively makes them impossible to choose.
    pub fn others_to_infinity(&mut self, moves: &[Move]) {
        self.up += if moves.contains(&Move::Up) {
            0.
        } else {
            f32::INFINITY
        };
        self.down += if moves.contains(&Move::Down) {
            0.
        } else {
            f32::INFINITY
        };
        self.left += if moves.contains(&Move::Left) {
            0.
        } else {
            f32::INFINITY
        };
        self.right += if moves.contains(&Move::Right) {
            0.
        } else {
            f32::INFINITY
        };
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
