use rocket::serde::{Deserialize, Serialize};

/// A coordinate on the board.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Gets all the orthogonal neighbors of the coordinate.
    pub fn ortho_neighbors(&self) -> Vec<Point> {
        vec![
            (self.x, self.y + 1).into(),
            (self.x, self.y - 1).into(),
            (self.x - 1, self.y).into(),
            (self.x + 1, self.y).into(),
        ]
    }

    /// Gets the manhattan distance between two coordinates.
    pub fn manhattan_distance(&self, other: &Point) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (i32, i32) {
    fn from(val: Point) -> Self {
        (val.x, val.y)
    }
}