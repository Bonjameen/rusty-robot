use std::io::stdin;

use input::parse_input;
use robot::ToyRobot;
use utils::byte_slice_to_vecdeque;

mod input;
mod input_utils;
mod robot;
mod robot_utils;
mod utils;

fn main() {
    let mut robot = ToyRobot::new();
    loop {
        let mut user_input = String::new();
        let mut instruction_result = Err("Instruction result not set".to_string());
        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                let mut input_vecd =
                    byte_slice_to_vecdeque(user_input.to_uppercase()[..].as_bytes());
                input_vecd.pop_back();
                instruction_result = parse_input(input_vecd);
            }
            Err(error) => eprintln!("error: {error}"),
        }
        match instruction_result {
            Ok(instruction) => match robot.take_instruction(instruction) {
                Ok(_) => {}
                Err(error) => eprintln!("error: {error}"),
            },
            Err(error) => eprintln!("error: {error}"),
        }
    }
}
