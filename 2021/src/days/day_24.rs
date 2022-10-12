use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    combinator::{map, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::fmt;
use std::fs;
use std::rc::Rc;

pub fn part1() -> String {
    let instructions = Rc::new(load_instructions());
    let mut alu = Alu::new(instructions, vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    alu.execute();
    println!("{}", alu.z);

    "".to_string()
}

pub fn part2() -> String {
    "".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            alt((tag("w"), tag("x"), tag("y"), tag("z"))),
            |register| match register {
                "w" => Register::W,
                "x" => Register::X,
                "y" => Register::Y,
                "z" => Register::Z,
                _ => unreachable!(),
            },
        )(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Register(Register),
    Number(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Input,
    Add(Value),
    Multiply(Value),
    Divide(Value),
    Modulo(Value),
    Equal(Value),
}

#[derive(Debug, Clone)]
struct Instruction {
    result_register: Register,
    operation: Operation,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                alt((
                    tag("inp"),
                    tag("add"),
                    tag("mul"),
                    tag("div"),
                    tag("mod"),
                    tag("eql"),
                )),
                multispace1,
                Register::parse,
                opt(tuple((
                    multispace1,
                    alt((
                        map(Register::parse, |register| Value::Register(register)),
                        map(
                            tuple((opt(tag::<_, &str, _>("-")), digit1)),
                            |(neg, num)| {
                                let mut parsed = num.parse::<i64>().unwrap();
                                if neg.is_some() {
                                    parsed *= -1
                                }
                                Value::Number(parsed)
                            },
                        ),
                    )),
                ))),
            )),
            |(op, _, result_register, value)| Instruction {
                result_register,
                operation: match op {
                    "inp" => Operation::Input,
                    "add" => Operation::Add(value.unwrap().1),
                    "mul" => Operation::Multiply(value.unwrap().1),
                    "div" => Operation::Divide(value.unwrap().1),
                    "mod" => Operation::Modulo(value.unwrap().1),
                    "eql" => Operation::Equal(value.unwrap().1),
                    _ => unreachable!(),
                },
            },
        )(input)
    }
}

struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    input_counter: usize,
    program_counter: usize,
    instructions: Rc<Vec<Instruction>>,
    inputs: Vec<i64>,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map(
        tuple((Instruction::parse, opt(line_ending))),
        |(instruction, _)| instruction,
    ))(&input)
}

fn load_instructions() -> Vec<Instruction> {
    let contents = fs::read_to_string("inputs/input24").expect("Failed to open input file");
    let (_, instructions) = parse_instructions(&contents).expect("Failed to parse input file");
    instructions
}

impl fmt::Debug for Alu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ALU")
            .field("w", &self.w)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("pc", &self.program_counter)
            .finish()
    }
}

impl Alu {
    fn new(instructions: Rc<Vec<Instruction>>, inputs: Vec<i64>) -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            program_counter: 0,
            input_counter: 0,
            instructions,
            inputs,
        }
    }

    fn execute(&mut self) {
        while self.program_counter < self.instructions.len() {
            self.execute_instruction();
            self.program_counter += 1;
        }
    }

    fn execute_instruction(&mut self) {
        let instruction = self.instructions[self.program_counter].clone();
        let operation_value = match &instruction.operation {
            Operation::Input => {
                let value = self.inputs[self.input_counter];
                self.input_counter += 1;
                value
            }
            Operation::Add(value) => self.get_value(value),
            Operation::Multiply(value) => self.get_value(value),
            Operation::Divide(value) => self.get_value(value),
            Operation::Modulo(value) => self.get_value(value),
            Operation::Equal(value) => self.get_value(value),
        };
        let register_value = self.get_register(&instruction.result_register);
        let result = match &instruction.operation {
            Operation::Input => operation_value,
            Operation::Add(..) => register_value + operation_value,
            Operation::Multiply(..) => register_value * operation_value,
            Operation::Divide(..) => register_value / operation_value,
            Operation::Modulo(..) => register_value % operation_value,
            Operation::Equal(..) => {
                if register_value == operation_value {
                    1
                } else {
                    0
                }
            }
        };
        self.set_register(&instruction.result_register, result);
    }

    fn get_value(&self, value: &Value) -> i64 {
        match value {
            Value::Number(number) => *number,
            Value::Register(register) => self.get_register(register),
        }
    }

    fn set_register(&mut self, register: &Register, value: i64) {
        match register {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
        }
    }

    fn get_register(&self, register: &Register) -> i64 {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }
}

#[test]
fn parse_test() {
    let input = "inp x";
    let (_, instruction) = Instruction::parse(input).unwrap();
    assert_eq!(instruction.result_register, Register::X);
    assert_eq!(instruction.operation, Operation::Input);

    let input = "mul x 1";
    let (_, instruction) = Instruction::parse(input).unwrap();
    assert_eq!(instruction.result_register, Register::X);
    assert_eq!(instruction.operation, Operation::Multiply(Value::Number(1)));

    let input = "add x w";
    let (_, instruction) = Instruction::parse(input).unwrap();
    assert_eq!(instruction.result_register, Register::X);
    assert_eq!(
        instruction.operation,
        Operation::Add(Value::Register(Register::W))
    );

    let input = "div z -1";
    let (_, instruction) = Instruction::parse(input).unwrap();
    assert_eq!(instruction.result_register, Register::Z);
    assert_eq!(instruction.operation, Operation::Divide(Value::Number(-1)));
}

#[test]
fn alu_test1() {
    let instructions = Rc::new(vec![
        Instruction {
            result_register: Register::X,
            operation: Operation::Input,
        },
        Instruction {
            result_register: Register::X,
            operation: Operation::Multiply(Value::Number(-1)),
        },
    ]);
    let mut alu = Alu::new(instructions.clone(), vec![5]);
    alu.execute();
    assert_eq!(alu.x, -5);
}

#[test]
fn alu_test2() {
    let instructions = Rc::new(vec![
        Instruction {
            result_register: Register::Z,
            operation: Operation::Input,
        },
        Instruction {
            result_register: Register::X,
            operation: Operation::Input,
        },
        Instruction {
            result_register: Register::Z,
            operation: Operation::Multiply(Value::Number(3)),
        },
        Instruction {
            result_register: Register::Z,
            operation: Operation::Equal(Value::Register(Register::X)),
        },
    ]);

    let mut alu = Alu::new(instructions.clone(), vec![5, 15]);
    alu.execute();
    assert_eq!(alu.z, 1);

    let mut alu = Alu::new(instructions.clone(), vec![4, 15]);
    alu.execute();
    assert_eq!(alu.z, 0);
}

#[test]
fn alu_test3() {
    let instructions = Rc::new(vec![
        Instruction {
            result_register: Register::W,
            operation: Operation::Input,
        },
        Instruction {
            result_register: Register::Z,
            operation: Operation::Add(Value::Register(Register::W)),
        },
        Instruction {
            result_register: Register::Z,
            operation: Operation::Modulo(Value::Number(2)),
        },
        Instruction {
            result_register: Register::W,
            operation: Operation::Divide(Value::Number(2)),
        },
        Instruction {
            result_register: Register::Y,
            operation: Operation::Add(Value::Register(Register::W)),
        },
        Instruction {
            result_register: Register::Y,
            operation: Operation::Modulo(Value::Number(2)),
        },
        Instruction {
            result_register: Register::W,
            operation: Operation::Divide(Value::Number(2)),
        },
        Instruction {
            result_register: Register::X,
            operation: Operation::Add(Value::Register(Register::W)),
        },
        Instruction {
            result_register: Register::X,
            operation: Operation::Modulo(Value::Number(2)),
        },
        Instruction {
            result_register: Register::W,
            operation: Operation::Divide(Value::Number(2)),
        },
        Instruction {
            result_register: Register::W,
            operation: Operation::Modulo(Value::Number(2)),
        },
    ]);

    let mut alu = Alu::new(instructions.clone(), vec![10]);
    alu.execute();
    assert_eq!(alu.z, 0);
    assert_eq!(alu.y, 1);
    assert_eq!(alu.x, 0);
    assert_eq!(alu.w, 1);
}
