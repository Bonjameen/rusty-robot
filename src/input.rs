use std::{collections::VecDeque, str::FromStr};

use crate::{
    input_utils::{Arg, ArgValue, Instruction, InstructionType},
    robot_utils::Direction,
};

pub fn parse_input(mut user_input: VecDeque<u8>) -> Result<Instruction, String> {
    let mut instruction = Instruction::new();
    match parse_instruction_type(&mut user_input) {
        Ok(instruction_type) => instruction.instruction_type = instruction_type,
        Err(error) => return Err(error),
    }
    if instruction.instruction_type == Some(InstructionType::PLACE) {
        match parse_args(&mut user_input) {
            Ok(args) => instruction.args = args,
            Err(error) => return Err(error),
        }
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
    instruction_result
}

fn parse_args(user_input: &mut VecDeque<u8>) -> Result<[Option<Arg>; 3], String> {
    let mut args = [None, None, None];
    let mut arg = String::new();
    while args.iter().any(|arg| *arg == None) {
        match user_input.pop_front() {
            Some(32) => match parse_arg(&mut args, &arg) {
                Ok(_) => arg = String::new(),
                Err(error) => return Err(error),
            },
            Some(byte) => arg.push(byte as char),
            None => match parse_arg(&mut args, &arg) {
                Ok(_) => arg = String::new(),
                Err(error) => return Err(error),
            },
        }
    }
    Ok(args)
}

fn parse_arg(args: &mut [Option<Arg>; 3], arg: &str) -> Result<(), String> {
    match args {
        [_, Some(_), None] => match Direction::from_str(arg) {
            Ok(direction) => args[2] = Some(Arg::Direction(ArgValue { direction })),
            Err(_) => return Err("Invalid direction, direction input must be one of the following (not case-sensitive):
            NORTH\tEAST\tSOUTH\tWEST".to_string()),
        },
        [Some(_), None, _] => {
            match arg.parse::<i8>() {
                Ok(co_ord) => args[1] = Some(Arg::Int(ArgValue { co_ord })),
                Err(_) => return Err(
                    "Invalid y-coordinate, coordinates must be a whole number between 0 and 255"
                        .to_string(),
                ),
            }
        }
        _ => {
            match arg.parse::<i8>() {
                Ok(co_ord) => args[0] = Some(Arg::Int(ArgValue { co_ord })),
                Err(_) => return Err(
                    "Invalid x-coordinate, coordinates must be a whole number between 0 and 255"
                        .to_string(),
                ),
            }
        }
    }
    Ok(())
}

fn match_instruction_type(word: &str) -> Result<Option<InstructionType>, String> {
    let mut instruction_type = None;
    match InstructionType::from_str(word) {
        Ok(instr_type) => {
            instruction_type = Some(instr_type);
        }
        Err(_) => {
            return Err("Invalid command entered, the only valid commands are:
          PLACE X,Y,F\tMOVE\tLEFT\tRIGHT\tREPORT"
                .to_string())
        }
    }
    Ok(instruction_type)
}
