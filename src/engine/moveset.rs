use crate::game::moves::Move;
use std::array::IntoIter;

/// The set of moves that a snake can make.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveSet {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl MoveSet {
    /// Creates a new moveset
    pub fn new() -> Self {
        Self {
            up: true,
            down: true,
            left: true,
            right: true,
        }
    }

    /// Sets a move to `false`, i.e. it is not a valid move.
    pub fn invalidate(&mut self, move_: &Move) {
        match move_ {
            Move::Up => self.up = false,
            Move::Down => self.down = false,
            Move::Left => self.left = false,
            Move::Right => self.right = false,
        }
    }

    /// Sets all other moves which are not in `moves` to `false`.
    pub fn invalidate_others_many(&mut self, moves: &[Move]) {
        if !moves.contains(&Move::Up) {
            self.up = false;
        }

        if !moves.contains(&Move::Down) {
            self.down = false;
        }

        if !moves.contains(&Move::Left) {
            self.left = false;
        }

        if !moves.contains(&Move::Right) {
            self.right = false;
        }
    }
}

impl IntoIterator for MoveSet {
    type Item = (Move, bool);

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
