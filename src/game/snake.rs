use std::fmt::Display;

use super::point::Point;
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize};

/// A battlesnake.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Snake {
    pub id: SnakeID,
    pub name: String,
    pub health: u32,
    pub body: Vec<Point>,
    pub head: Point,
    pub length: u32,
    pub latency: String,
    pub shout: Option<String>,
}

impl Snake {
    /// Gets the snake's tail.
    pub fn tail(&self) -> Point {
        *self.body.last().expect("All snakes should have a tail.")
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
