use num_traits::FromPrimitive;

use crate::{
    input_utils::{Arg, Instruction, InstructionType},
    robot_utils::{Direction, Movement, Orientation},
};

pub struct ToyRobot {
    orientation: Orientation,
}

impl ToyRobot {
    pub fn new() -> Self {
        ToyRobot {
            orientation: Orientation::new(),
        }
    }

    pub fn take_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction.instruction_type {
            Some(InstructionType::REPORT) => self.print_report(),
            Some(InstructionType::PLACE) => return self.execute_placement(instruction.args),
            Some(_) => {
                let delta = instruction.to_delta(&self.orientation);
                self.execute_movement(delta);
            }
            None => {}
        }
        Ok(())
    }

    fn print_report(&self) {
        let position = &self.orientation.position;
        let direction = &self.orientation.direction;
        println!("Output: {} {} {}", position.x, position.y, direction)
    }

    fn execute_placement(&mut self, args: [Option<Arg>; 3]) -> Result<(), String> {
        for i in 0..3 {
            unsafe {
                match &args[i] {
                    Some(Arg::Int(arg)) => {
                        if i == 0 {
                            self.orientation.position.x = arg.co_ord;
                            continue;
                        }
                        self.orientation.position.y = arg.co_ord;
                    }
                    Some(Arg::Direction(arg)) => {
                        self.orientation.direction = arg.direction;
                    }
                    None => return Err("ERROR: not all arguments are populated".to_string()),
                }
            }
        }
        Ok(())
    }

    fn execute_movement(&mut self, delta: Movement) {
        self.orientation.position += delta.delta_pos;
        match FromPrimitive::from_i16(
            ((self.orientation.direction as i16) + delta.delta_angle) % 360,
        ) {
            Some(
                direction @ (Direction::NORTH
                | Direction::EAST
                | Direction::SOUTH
                | Direction::WEST),
            ) => self.orientation.direction = direction,
            None => {}
        }
    }
}
