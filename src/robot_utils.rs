use std::str::FromStr;

pub struct Orientation {
    position: PosVector,
    direction: Direction,
}

impl Orientation {
    pub fn new() -> Self {
        Orientation {
            position: PosVector::new(),
            direction: Direction::NORTH,
        }
    }
}

pub struct PosVector {
    x: i8,
    y: i8,
}

impl PosVector {
    pub fn new() -> Self {
        PosVector { x: 0, y: 0 }
    }
}

pub struct Movement {
    delta_pos: PosVector,
    delta_angle: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
