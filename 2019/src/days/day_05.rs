use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut program = Program::parse(&input);
    program.run(1);
    assert!(program.output.iter().rev().skip(1).all(|v| *v == 0));
    program.output.last().unwrap().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut program = Program::parse(&input);
    program.run(5);
    program.output.first().unwrap().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input05").expect("Failed to open input file")
}

#[derive(Debug)]
struct Program {
    memory: Vec<i32>,
    initial_memory: Vec<i32>,
    instruction_pointer: usize,
    halted: bool,
    output: Vec<i32>,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add {
        a: Parameter,
        b: Parameter,
        c: Parameter,
    },
    Multiplty {
        a: Parameter,
        b: Parameter,
        c: Parameter,
    },
    Input {
        a: Addr,
    },
    Output {
        a: Parameter,
    },
    JumpIfTrue {
        a: Parameter,
        b: Parameter,
    },
    JumpIfFalse {
        a: Parameter,
        b: Parameter,
    },
    LessThan {
        a: Parameter,
        b: Parameter,
        c: Parameter,
    },
    Equals {
        a: Parameter,
        b: Parameter,
        c: Parameter,
    },
    Halt,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
enum Parameter {
    Position(Addr),
    Immediate(i32),
}

#[derive(Debug, PartialEq)]
struct Addr(usize);

impl Program {
    fn parse(input: &str) -> Self {
        let memory: Vec<i32> = input
            .trim_end()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let initial_memory = memory.clone();
        Program {
            memory,
            initial_memory,
            instruction_pointer: 0,
            halted: false,
            output: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.memory = self.initial_memory.clone();
        self.instruction_pointer = 0;
        self.halted = false;
        self.output.clear();
    }

    fn run(&mut self, input: i32) {
        while !self.halted {
            let (instruction, instruction_len) =
                Instruction::parse(&self.memory[self.instruction_pointer..]);
            self.instruction_pointer += instruction_len;

            match instruction {
                Instruction::Add { a, b, c } => {
                    let a = match a {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let b = match b {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let result = a + b;
                    match c {
                        Parameter::Position(addr) => self.memory[addr.0] = result,
                        Parameter::Immediate(value) => self.memory[value as usize] = result,
                    }
                }
                Instruction::Multiplty { a, b, c } => {
                    let a = match a {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let b = match b {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let result = a * b;
                    match c {
                        Parameter::Position(addr) => self.memory[addr.0] = result,
                        Parameter::Immediate(value) => self.memory[value as usize] = result,
                    }
                }
                Instruction::Input { a: addr } => {
                    self.memory[addr.0] = input;
                }
                Instruction::Output { a } => {
                    let value = match a {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    self.output.push(value);
                }
                Instruction::JumpIfTrue { a, b } => {
                    let value = match a {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    if value != 0 {
                        self.instruction_pointer = match b {
                            Parameter::Position(addr) => self.memory[addr.0],
                            Parameter::Immediate(value) => value,
                        } as usize;
                    }
                }
                Instruction::JumpIfFalse { a, b } => {
                    let value = match a {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    if value == 0 {
                        self.instruction_pointer = match b {
                            Parameter::Position(addr) => self.memory[addr.0],
                            Parameter::Immediate(value) => value,
                        } as usize;
                    }
                }
                Instruction::LessThan { a, b, c } => {
                    let a = match a {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let b = match b {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let result = if a < b { 1 } else { 0 };
                    match c {
                        Parameter::Position(addr) => self.memory[addr.0] = result,
                        Parameter::Immediate(value) => self.memory[value as usize] = result,
                    }
                }
                Instruction::Equals { a, b, c } => {
                    let a = match a {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let b = match b {
                        Parameter::Position(addr) => self.memory[addr.0],
                        Parameter::Immediate(value) => value,
                    };
                    let result = if a == b { 1 } else { 0 };
                    match c {
                        Parameter::Position(addr) => self.memory[addr.0] = result,
                        Parameter::Immediate(value) => self.memory[value as usize] = result,
                    }
                }
                Instruction::Halt => self.halted = true,
            }
        }
    }
}

impl Instruction {
    fn parse(memory: &[i32]) -> (Self, usize) {
        match memory[0] % 100 {
            1 => {
                let param_modes = Self::parse_parameter_modes(memory[0]);
                (
                    Instruction::Add {
                        a: Parameter::from_mode(memory[1], param_modes[0]),
                        b: Parameter::from_mode(memory[2], param_modes[1]),
                        c: Parameter::from_mode(memory[3], param_modes[2]),
                    },
                    4,
                )
            }
            2 => {
                let param_modes = Self::parse_parameter_modes(memory[0]);
                (
                    Instruction::Multiplty {
                        a: Parameter::from_mode(memory[1], param_modes[0]),
                        b: Parameter::from_mode(memory[2], param_modes[1]),
                        c: Parameter::from_mode(memory[3], param_modes[2]),
                    },
                    4,
                )
            }
            3 => (
                Instruction::Input {
                    a: Addr(memory[1] as usize),
                },
                2,
            ),
            4 => {
                let param_modes = Self::parse_parameter_modes(memory[0]);
                (
                    Instruction::Output {
                        a: Parameter::from_mode(memory[1], param_modes[0]),
                    },
                    2,
                )
            }
            5 => {
                let param_modes = Self::parse_parameter_modes(memory[0]);
                (
                    Instruction::JumpIfTrue {
                        a: Parameter::from_mode(memory[1], param_modes[0]),
                        b: Parameter::from_mode(memory[2], param_modes[1]),
                    },
                    3,
                )
            }
            6 => {
                let param_modes = Self::parse_parameter_modes(memory[0]);
                (
                    Instruction::JumpIfFalse {
                        a: Parameter::from_mode(memory[1], param_modes[0]),
                        b: Parameter::from_mode(memory[2], param_modes[1]),
                    },
                    3,
                )
            }
            7 => {
                let param_modes = Self::parse_parameter_modes(memory[0]);
                (
                    Instruction::LessThan {
                        a: Parameter::from_mode(memory[1], param_modes[0]),
                        b: Parameter::from_mode(memory[2], param_modes[1]),
                        c: Parameter::from_mode(memory[3], param_modes[2]),
                    },
                    4,
                )
            }
            8 => {
                let param_modes = Self::parse_parameter_modes(memory[0]);
                (
                    Instruction::Equals {
                        a: Parameter::from_mode(memory[1], param_modes[0]),
                        b: Parameter::from_mode(memory[2], param_modes[1]),
                        c: Parameter::from_mode(memory[3], param_modes[2]),
                    },
                    4,
                )
            }
            99 => (Instruction::Halt, 1),
            opcode => panic!("invalid opcode: {opcode}"),
        }
    }

    fn parse_parameter_modes(opcode: i32) -> [ParameterMode; 3] {
        [
            ParameterMode::parse((opcode / 100) % 10),
            ParameterMode::parse((opcode / 1000) % 10),
            ParameterMode::parse((opcode / 10000) % 10),
        ]
    }
}

impl Parameter {
    fn from_mode(value: i32, mode: ParameterMode) -> Self {
        match mode {
            ParameterMode::Position => Parameter::Position(Addr(value as usize)),
            ParameterMode::Immediate => Parameter::Immediate(value),
        }
    }
}

impl ParameterMode {
    fn parse(value: i32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("invalid paramter mode {value}"),
        }
    }
}

#[test]
fn test_parse() {
    let input = "1,9,10,3,2,3,11,0,99,30,40,50\n";
    let program = Program::parse(input);
    assert_eq!(
        program.memory,
        vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
fn test_run_program() {
    {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50\n";
        let mut program = Program::parse(input);
        program.run(0);
        assert_eq!(
            program.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    {
        let input = "1,0,0,0,99";
        let mut program = Program::parse(input);
        program.run(0);
        assert_eq!(program.memory, vec![2, 0, 0, 0, 99]);
    }

    {
        let input = "2,3,0,3,99";
        let mut program = Program::parse(input);
        program.run(0);
        assert_eq!(program.memory, vec![2, 3, 0, 6, 99]);
    }

    {
        let input = "2,4,4,5,99,0";
        let mut program = Program::parse(input);
        program.run(0);
        assert_eq!(program.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    {
        let input = "1,1,1,4,99,5,6,0,99";
        let mut program = Program::parse(input);
        program.run(0);
        assert_eq!(program.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    {
        let input = "1002,4,3,4,33";
        let mut program = Program::parse(input);
        program.run(0);
        assert_eq!(program.memory, vec![1002, 4, 3, 4, 99]);
    }
}

#[test]
fn test_new_opcodes() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let mut program = Program::parse(input);

    program.run(7);
    assert_eq!(program.output.first(), Some(&999));
    program.reset();

    program.run(8);
    assert_eq!(program.output.first(), Some(&1000));
    program.reset();

    program.run(9);
    assert_eq!(program.output.first(), Some(&1001));
    program.reset();
}
