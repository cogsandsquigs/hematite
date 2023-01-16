use rocket::serde::{Deserialize, Serialize};

/// A coordinate on the board.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    /// Gets all the orthogonal neighbors of the coordinate.
    pub fn ortho_neighbors(&self) -> Vec<Coord> {
        vec![
            (self.x, self.y + 1).into(),
            (self.x, self.y - 1).into(),
            (self.x - 1, self.y).into(),
            (self.x + 1, self.y).into(),
        ]
    }

    /// Gets the manhattan distance between two coordinates.
    pub fn manhattan_distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Coord> for (i32, i32) {
    fn from(val: Coord) -> Self {
        (val.x, val.y)
    }
}
