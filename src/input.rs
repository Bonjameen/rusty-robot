struct Instruction {
    instruction_type: Option<String>,
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

union Arg {
    co_ord: i8,
    direction: Direction,
}
