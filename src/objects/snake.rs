use std::fmt::Display;

use super::{moves::Move, point::Point};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize};

/// A battlesnake. I tried to make this as efficient to copy/manipulate as possible, but
/// there are still some problems.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Snake {
    /// The snake's ID.
    pub id: SnakeID,

    /// How much health the snake has. When health reaches 0, the snake dies.
    pub health: i32,

    /// The snake's body. The array is ordered from head to tail.
    pub body: Vec<Point>,

    /// The snake's head.
    pub head: Point,

    /// The length of the snake.
    pub length: u32,

    /// The squad the snake is in. Only applies during squad battles.
    pub squad: Option<String>,
    // These are fields that are not used in the game, but are included in the API response.
    // /// The snake's name.
    // pub name: String,
    //
    // /// The snake's latency.
    // pub latency: String,
    //
    // /// The snake's shout - what is this?
    // pub shout: Option<String>,
}

impl Snake {
    /// Gets the snake's tail.
    pub fn tail(&self) -> Point {
        *self.body.last().expect("All snakes should have a tail.")
    }

    /// Gets the previous move of the snake.
    pub fn previous_move(&self) -> Move {
        Move::from_points(&self.body[1], &self.head)
            .expect("Snake should have a valid previous move.")
    }
}

/// An ID for a snake. This is not a `String` like in the original implementation, but a custom
/// type that implies `Copy` for efficiency.
///
/// An example of an ID for a snake is `gs_Mgf9YfdBvqk4JMmCYtSWFxx9`. These IDs are 27 bytes long, but have a 3-byte
/// prefix contributes no information. Therefore, the `SnakeID` type is 24 bytes long.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SnakeID {
    id: [u8; 24],
}

impl Serialize for SnakeID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(27))?;
        seq.serialize_element(&b"gs_")?;
        seq.serialize_element(&self.id)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SnakeID {
    fn deserialize<D>(deserializer: D) -> Result<SnakeID, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Remove first three leading charactes, which are "gs_". They contribute no information to
        // the snake ID.
        let id = s
            .strip_prefix("gs_")
            .ok_or_else(|| serde::de::Error::custom("Snake ID must start with gs_"))?;

        let id = id.as_bytes();

        if id.len() != 24 {
            return Err(serde::de::Error::custom("Snake ID must be 27 bytes long"));
        }

        let id = id.try_into().unwrap();

        Ok(Self { id })
    }
}

impl Display for SnakeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gs_{}", std::str::from_utf8(&self.id).unwrap())
    }
}
