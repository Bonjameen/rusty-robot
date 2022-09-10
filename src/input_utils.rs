use std::{fmt, str::FromStr};

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

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: Option<InstructionType>,
    pub args: [Option<Arg>; 3],
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            instruction_type: None,
            args: [None, None, None],
        }
    }
}

pub union ArgValue {
    pub co_ord: i8,
    pub direction: Direction,
}

pub enum Arg {
    Int(ArgValue),
    Direction(ArgValue),
}

impl fmt::Debug for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            match self {
                Arg::Int(val) => {
                    let co_ord = val.co_ord;
                    write!(f, "{co_ord}")
                }
                Arg::Direction(val) => {
                    let direction = val.direction;
                    write!(f, "{direction:?}")
                }
            }
        }
    }
}

impl PartialEq for Arg {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            match (self, other) {
                (Self::Int(l0), Self::Int(r0)) => l0.co_ord == r0.co_ord,
                (Self::Direction(l0), Self::Direction(r0)) => l0.direction == r0.direction,
                (Self::Direction(_), Self::Int(_)) => return false,
                (Self::Int(_), Self::Direction(_)) => return false,
            }
        }
    }
}
