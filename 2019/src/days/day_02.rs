use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut program = Program::parse(&input);
    program.memory[1] = 12;
    program.memory[2] = 2;
    program.run();
    program.memory[0].to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut program = Program::parse(&input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            program.reset();
            program.memory[1] = noun;
            program.memory[2] = verb;
            program.run();
            if program.memory[0] == 19690720 {
                return (100 * noun + verb).to_string();
            }
        }
    }
    panic!("no solution found")
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input02").expect("Failed to open input file")
}

#[derive(Debug)]
struct Program {
    memory: Vec<u32>,
    initial_memory: Vec<u32>,
    instruction_pointer: usize,
    halted: bool,
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add = 1,
    Multiplty = 2,
    Halt = 99,
}

impl Program {
    fn parse(input: &str) -> Self {
        let memory: Vec<u32> = input
            .trim_end()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let initial_memory = memory.clone();
        Program {
            memory,
            initial_memory,
            instruction_pointer: 0,
            halted: false,
        }
    }

    fn reset(&mut self) {
        self.memory = self.initial_memory.clone();
        self.instruction_pointer = 0;
        self.halted = false;
    }

    fn run(&mut self) {
        while !self.halted {
            match OpCode::parse(self.memory[self.instruction_pointer]) {
                OpCode::Add => {
                    let a = self.memory[self.memory[self.instruction_pointer + 1] as usize];
                    let b = self.memory[self.memory[self.instruction_pointer + 2] as usize];
                    let result_addr = self.memory[self.instruction_pointer + 3] as usize;
                    self.memory[result_addr] = a + b;
                    self.instruction_pointer += 4;
                }
                OpCode::Multiplty => {
                    let a = self.memory[self.memory[self.instruction_pointer + 1] as usize];
                    let b = self.memory[self.memory[self.instruction_pointer + 2] as usize];
                    let result_addr = self.memory[self.instruction_pointer + 3] as usize;
                    self.memory[result_addr] = a * b;
                    self.instruction_pointer += 4;
                }
                OpCode::Halt => self.halted = true,
            }
        }
    }
}

impl OpCode {
    fn parse(integer: u32) -> Self {
        match integer {
            1 => OpCode::Add,
            2 => OpCode::Multiplty,
            99 => OpCode::Halt,
            _ => panic!("invalid opcode {integer}"),
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
        program.run();
        assert_eq!(
            program.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    {
        let input = "1,0,0,0,99";
        let mut program = Program::parse(input);
        program.run();
        assert_eq!(program.memory, vec![2, 0, 0, 0, 99]);
    }

    {
        let input = "2,3,0,3,99";
        let mut program = Program::parse(input);
        program.run();
        assert_eq!(program.memory, vec![2, 3, 0, 6, 99]);
    }

    {
        let input = "2,4,4,5,99,0";
        let mut program = Program::parse(input);
        program.run();
        assert_eq!(program.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    {
        let input = "1,1,1,4,99,5,6,0,99";
        let mut program = Program::parse(input);
        program.run();
        assert_eq!(program.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
