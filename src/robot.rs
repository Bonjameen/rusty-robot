use crate::robot_utils::Orientation;

pub struct ToyRobot {
    orientation: Orientation,
}

impl ToyRobot {
    pub fn new() -> Self {
        ToyRobot {
            orientation: Orientation::new(),
        }
    }
}
