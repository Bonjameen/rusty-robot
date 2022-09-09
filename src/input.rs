use core::fmt;
use std::{collections::VecDeque, str::FromStr};

use crate::{
    input_utils::{Instruction, InstructionType},
    robot_utils::Direction,
};

pub fn parse_input(mut user_input: VecDeque<u8>) -> Result<Option<Instruction>, String> {
    let mut instruction = Instruction::new();
    match parse_instruction_type(&mut user_input) {
        Ok(instruction_type) => instruction.instruction_type = instruction_type,
        Err(error) => return Err(error),
    }
    println!("{instruction:?}");
    Ok(Some(instruction))
}

fn parse_instruction_type(
    user_input: &mut VecDeque<u8>,
) -> Result<Option<InstructionType>, String> {
    let mut instruction_result = Ok(None);
    let mut word = String::new();
    while instruction_result == Ok(None) {
        match user_input.pop_front() {
            Some(32) => instruction_result = match_instruction_type(&word[..]),
            Some(byte) => word.push(byte as char),
            None => instruction_result = match_instruction_type(&word[..]),
        }
    }
    return instruction_result;
}

fn match_instruction_type(word: &str) -> Result<Option<InstructionType>, String> {
    let mut instruction_type = None;
    match InstructionType::from_str(word) {
        Ok(instr_type) => {
            instruction_type = Some(instr_type);
        }
        Err(error) => {
            return Err("Invalid command entered, the only valid commands are:
          PLACE X,Y,F\tMOVE\tLEFT\tRIGHT\tREPORT"
                .to_string())
        }
    }
    Ok(instruction_type)
}
