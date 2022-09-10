use std::io::stdin;

use input::parse_input;
use utils::byte_slice_to_vecdeque;

mod input;
mod input_utils;
mod robot;
mod robot_utils;
mod utils;

fn main() {
    let mut user_input = String::new();
    let mut instruction_result = Err("Instruction result not set".to_string());
    match stdin().read_line(&mut user_input) {
        Ok(_) => {
            let mut input_vecd = byte_slice_to_vecdeque(user_input.to_uppercase()[..].as_bytes());
            input_vecd.pop_back();
            instruction_result = parse_input(input_vecd);
        }
        Err(error) => eprintln!("error: {error}"),
    }
}
