use std::fs;

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let instructions = parse_instructions(&contents);
    let mut cpu = Cpu::new(instructions);
    cpu.run_program();
    let signal_strengths_sum = cpu.signal_strengths.iter().sum::<i64>();
    format!("{}", signal_strengths_sum)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let instructions = parse_instructions(&contents);
    let mut cpu = Cpu::new(instructions);
    cpu.run_program();
    format!("\n{}", cpu.draw_screen())
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input10").expect("Failed to open input file")
}

#[derive(Debug)]
struct Cpu {
    x: i64,
    x_history: Vec<i64>,
    signal_strengths: Vec<i64>,
    cycle: u64,
    instructions: Vec<Instruction>,
    instruction_counter: usize,
    executing_addx: bool,
    pixels: Vec<bool>,
}

impl Cpu {
    const ROWS: usize = 6;
    const COLS: usize = 40;

    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            x: 1,
            x_history: Vec::new(),
            signal_strengths: Vec::new(),
            cycle: 1,
            instructions,
            instruction_counter: 0,
            executing_addx: false,
            pixels: vec![false; Self::ROWS * Self::COLS],
        }
    }

    fn draw_screen(&self) -> String {
        let mut screen = String::with_capacity(Self::ROWS * Self::COLS);
        for row in 0..Self::ROWS {
            for col in 0..Self::COLS {
                let i = row * Self::COLS + col;
                screen.push(if self.pixels[i] { '#' } else { '.' });
            }
            screen.push('\n');
        }
        screen
    }

    fn run_program(&mut self) {
        while !self.finished_executing() {
            self.tick();
        }
    }

    fn finished_executing(&self) -> bool {
        self.instruction_counter == self.instructions.len()
    }

    fn record_signal_strength(&mut self) {
        if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
            self.signal_strengths.push(self.x * self.cycle as i64);
        }
    }

    fn draw_pixel(&mut self) {
        let col = (self.cycle - 1) % Self::COLS as u64;
        let row = (self.cycle - 1) / Self::COLS as u64;
        let i = (row * Self::COLS as u64 + col) as usize;
        self.pixels[i] =
            (col as i64 == self.x) || (col as i64 == self.x - 1) || (col as i64 == self.x + 1);
    }

    fn tick(&mut self) {
        if self.finished_executing() {
            return;
        }
        let instruction = self.instructions[self.instruction_counter];
        match instruction {
            Instruction::Addx(v) => {
                if !self.executing_addx {
                    // First cycle of addx instruction
                    self.executing_addx = true;
                    self.x_history.push(self.x);
                    self.record_signal_strength();
                    self.draw_pixel();
                } else {
                    // Second cycle of addx instruction
                    self.executing_addx = false;
                    self.x_history.push(self.x);
                    self.record_signal_strength();
                    self.draw_pixel();
                    self.x += v;
                    self.instruction_counter += 1;
                }
            }
            Instruction::Noop => {
                // Do nothing
                self.x_history.push(self.x);
                self.record_signal_strength();
                self.draw_pixel();
                self.instruction_counter += 1;
            }
        }
        self.cycle += 1;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    fn parse(input: &str) -> Option<Self> {
        let parts = input.split(' ').collect::<Vec<&str>>();
        if parts.len() == 1 && parts[0] == "noop" {
            Some(Self::Noop)
        } else if parts.len() == 2 && parts[0] == "addx" {
            let v = parts[1]
                .parse::<i64>()
                .expect("Failed to parse addx instruction value");
            Some(Self::Addx(v))
        } else {
            None
        }
    }
}

fn parse_instructions(contents: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        instructions.push(Instruction::parse(line).expect("Failed to parse instruction"));
    }
    instructions
}

#[test]
fn test_parse() {
    let contents = "noop\naddx 3\naddx -5\n";
    let instructions = parse_instructions(&contents);
    assert_eq!(instructions.len(), 3);
    assert_eq!(instructions[0], Instruction::Noop);
    assert_eq!(instructions[1], Instruction::Addx(3));
    assert_eq!(instructions[2], Instruction::Addx(-5));
}

#[test]
fn test_cpu() {
    let contents = "noop\naddx 3\naddx -5\n";
    let instructions = parse_instructions(&contents);
    let mut cpu = Cpu::new(instructions);
    cpu.run_program();
    assert_eq!(cpu.x_history.len(), 5);
    assert_eq!(cpu.x_history, vec![1, 1, 1, 4, 4]);
    assert_eq!(cpu.x, -1);
}

#[test]
fn test_signal_strength() {
    let contents = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n";
    let instructions = parse_instructions(&contents);
    let mut cpu = Cpu::new(instructions);
    cpu.run_program();
    assert_eq!(cpu.signal_strengths.len(), 6);
    assert_eq!(
        cpu.signal_strengths,
        vec![420, 1140, 1800, 2940, 2880, 3960]
    );
    assert_eq!(cpu.signal_strengths.iter().sum::<i64>(), 13140);
}

#[test]
fn test_screen_output() {
    let contents = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n";
    let instructions = parse_instructions(&contents);
    let mut cpu = Cpu::new(instructions);
    cpu.run_program();
    let screen = cpu.draw_screen();
    assert_eq!(screen, "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n");
}
