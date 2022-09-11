use core::fmt;
use std::{ops::AddAssign, str::FromStr};

use num_derive::FromPrimitive;

pub struct Orientation {
    pub position: PosVector,
    pub direction: Direction,
}

impl Orientation {
    pub fn new() -> Self {
        Orientation {
            position: PosVector::new(),
            direction: Direction::NORTH,
        }
    }
}

pub struct Movement {
    pub delta_pos: PosVector,
    pub delta_angle: i16,
}

impl Movement {
    pub fn new() -> Self {
        Movement {
            delta_pos: PosVector::new(),
            delta_angle: 0,
        }
    }
}

pub struct PosVector {
    pub x: i8,
    pub y: i8,
}

impl PosVector {
    pub fn new() -> Self {
        PosVector { x: 0, y: 0 }
    }
}

impl AddAssign for PosVector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Direction {
    NORTH = 0,
    EAST = 90,
    SOUTH = 180,
    WEST = 270,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NORTH" => return Ok(Direction::NORTH),
            "EAST" => return Ok(Direction::EAST),
            "SOUTH" => return Ok(Direction::SOUTH),
            "WEST" => return Ok(Direction::WEST),
            _ => Err("Invalid direction passed in".to_string()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::NORTH => write!(f, "NORTH"),
            Direction::EAST => write!(f, "EAST"),
            Direction::SOUTH => write!(f, "SOUTH"),
            Direction::WEST => write!(f, "WEST"),
        }
    }
}
