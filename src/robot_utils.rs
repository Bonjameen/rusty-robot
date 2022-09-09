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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    NORTH = 0,
    EAST = 90,
    SOUTH = 180,
    WEST = 270,
}
