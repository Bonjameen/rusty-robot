use core::fmt;
use std::{collections::VecDeque, str::FromStr};

use crate::robot_utils::Direction;

#[derive(PartialEq, Eq, Debug)]
pub enum InstructionType {
    PLACE,
    MOVE,
    LEFT,
    RIGHT,
    REPORT,
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionType::PLACE => write!(f, "PLACE"),
            InstructionType::LEFT => write!(f, "LEFT"),
            InstructionType::RIGHT => write!(f, "RIGHT"),
            InstructionType::MOVE => write!(f, "MOVE"),
            InstructionType::REPORT => write!(f, "REPORT"),
        }
    }
}

impl FromStr for InstructionType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "PLACE" => Ok(InstructionType::PLACE),
            "LEFT" => Ok(InstructionType::LEFT),
            "RIGHT" => Ok(InstructionType::RIGHT),
            "MOVE" => Ok(InstructionType::MOVE),
            "REPORT" => Ok(InstructionType::REPORT),
            _ => Err("Invalid Instruction Type passed".to_string()),
        }
    }
}

pub struct Instruction {
    instruction_type: Option<InstructionType>,
    args: [Option<Arg>; 3],
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            instruction_type: None,
            args: [None, None, None],
        }
    }
}

pub union Arg {
    co_ord: i8,
    direction: Direction,
}

pub fn parse_input(mut user_input: VecDeque<u8>) -> Result<Instruction, String> {
    let mut instruction = Instruction::new();
    match parse_instruction_type(&mut user_input) {
        Ok(instruction_type) => instruction.instruction_type = instruction_type,
        Err(error) => return Err(error),
    }
    Ok(instruction)
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
