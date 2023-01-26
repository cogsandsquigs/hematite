use rocket::serde::{Deserialize, Serialize};

/// A coordinate on the board.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Gets all the orthogonal neighbors of the coordinate.
    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            (self.x, self.y + 1).into(),
            (self.x, self.y - 1).into(),
            (self.x - 1, self.y).into(),
            (self.x + 1, self.y).into(),
        ]
    }

    /// Gets the manhattan distance between two coordinates.
    pub fn distance(&self, other: &Point) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    /// Gets the closest distance of a point to a list of points.
    pub fn closest_distance(&self, points: &[Point]) -> Option<u32> {
        points.iter().map(|point| self.distance(point)).min()
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
