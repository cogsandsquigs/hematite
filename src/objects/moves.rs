use super::point::Point;
use rand::seq::SliceRandom;
use rocket::serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A move that a snake can make.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Move {
    #[serde(rename = "up")]
    Up,

    #[serde(rename = "down")]
    Down,

    #[serde(rename = "left")]
    Left,

    #[serde(rename = "right")]
    Right,
}

impl Move {
    /// Returns the set of all possible moves.
    pub fn all() -> [Move; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    /// Returns a random move.
    pub fn random<R: rand::Rng>(rng: &mut R) -> Self {
        *Self::all()
            .choose(rng)
            .expect("There should always be at least one move.")
    }

    /// Turns two coordinates into a move, based on the difference between them.
    pub fn from_points(start: &Point, end: &Point) -> Option<Self> {
        let (x_diff, y_diff) = (end.x - start.x, end.y - start.y);

        match (x_diff, y_diff) {
            (0, 1) => Some(Self::Up),
            (0, -1) => Some(Self::Down),
            (-1, 0) => Some(Self::Left),
            (1, 0) => Some(Self::Right),
            _ => None,
        }
    }

    /// Turns the move into an ending coordinate point given a starting coordinate.
    pub fn to_point(&self, start: &Point) -> Point {
        match self {
            Move::Up => (start.x, start.y + 1).into(),
            Move::Down => (start.x, start.y - 1).into(),
            Move::Left => (start.x - 1, start.y).into(),
            Move::Right => (start.x + 1, start.y).into(),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Up => write!(f, "up"),
            Move::Down => write!(f, "down"),
            Move::Left => write!(f, "left"),
            Move::Right => write!(f, "right"),
        }
    }
}
