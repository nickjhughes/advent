use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::{eof, map, opt},
    multi::{many_till, separated_list0},
    sequence::tuple,
    IResult,
};
use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let mut monkeys = parse_monkeys(&contents);
    for _ in 0..20 {
        run_round(&mut monkeys, true);
    }
    let monkey_business = calc_monkey_business(&mut monkeys);
    format!("{}", monkey_business)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let mut monkeys = parse_monkeys(&contents);
    for _ in 0..10000 {
        run_round(&mut monkeys, false);
    }
    let monkey_business = calc_monkey_business(&mut monkeys);
    format!("{}", monkey_business)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input11").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Number(u32),
    InputValue,
}

#[derive(Debug)]
struct Modulo {
    u32_value: u32,
    u32_is_valid: bool,
    modulos_are_valid: bool,
    values: HashMap<u32, u32>,
}

impl Modulo {
    fn new(value: u32, modulos: &[u32]) -> Self {
        let mut values = HashMap::new();
        for modulo in modulos {
            values.insert(*modulo, value % modulo);
        }
        Self {
            u32_value: value,
            u32_is_valid: true,
            modulos_are_valid: true,
            values,
        }
    }

    fn add_modulo(&mut self, modulo: u32, value: u32) {
        self.values.insert(modulo, value);
    }

    fn divisible_by(&self, modulo: u32) -> bool {
        if !self.values.contains_key(&modulo) {
            panic!("Unsupported modulo");
        }
        if self.u32_is_valid {
            self.u32_value % modulo == 0
        } else if self.modulos_are_valid {
            self.values[&modulo] == 0
        } else {
            panic!("Both u32 and modulo approaches are invalid");
        }
    }

    fn add(&mut self, operand: u32) {
        for (modulo, value) in self.values.iter_mut() {
            let new_value = *value + operand;
            *value = new_value % modulo;
        }
        let result = self.u32_value.overflowing_add(operand);
        self.u32_value = result.0;
        if result.1 {
            self.u32_is_valid = false;
        }
    }

    fn multiply(&mut self, operand: u32) {
        for (modulo, value) in self.values.iter_mut() {
            let new_value = *value * operand;
            *value = new_value % modulo;
        }
        let result = self.u32_value.overflowing_mul(operand);
        self.u32_value = result.0;
        if result.1 {
            self.u32_is_valid = false;
        }
    }

    fn multiply_self(&mut self) {
        for (modulo, value) in self.values.iter_mut() {
            let new_value = *value * *value;
            *value = new_value % modulo;
        }
        let result = self.u32_value.overflowing_mul(self.u32_value);
        self.u32_value = result.0;
        if result.1 {
            self.u32_is_valid = false;
        }
    }

    fn divide(&mut self, operand: u32) {
        for (modulo, value) in self.values.iter_mut() {
            let new_value = *value / operand;
            *value = new_value % modulo;
        }
        self.modulos_are_valid = false;
        let result = self.u32_value.overflowing_div(operand);
        self.u32_value = result.0;
        if result.1 {
            self.u32_is_valid = false;
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Modulo>,
    operation: Operation,
    operand: Operand,
    modulo: u32,
    true_monkey: usize,
    false_monkey: usize,
    items_inspected_count: usize,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("Monkey "),
                digit1::<&str, _>,
                tag(":\n  Starting items: "),
                separated_list0(tag(", "), digit1),
                tag("\n  Operation: new = old "),
                alt((tag("+"), tag("*"))),
                tag(" "),
                alphanumeric1::<&str, _>,
                tag("\n  Test: divisible by "),
                digit1::<&str, _>,
                tag("\n    If true: throw to monkey "),
                digit1::<&str, _>,
                tag("\n    If false: throw to monkey "),
                digit1::<&str, _>,
                tag("\n"),
                opt(tag("\n")),
            )),
            |(
                _,
                _,
                _,
                items,
                _,
                operation,
                _,
                operand,
                _,
                modulo,
                _,
                true_monkey,
                _,
                false_monkey,
                _,
                _,
            )| {
                let modulo = modulo.parse::<u32>().expect("Failed to parse modulo");
                Self {
                    items: items
                        .iter()
                        .map(|i| i.parse::<u32>().expect("Failed to parse item"))
                        .map(|i| Modulo::new(i, &[modulo]))
                        .collect::<Vec<Modulo>>(),
                    operation: match operation {
                        "+" => Operation::Addition,
                        "*" => Operation::Multiplication,
                        _ => unreachable!(),
                    },
                    operand: {
                        if operand == "old" {
                            Operand::InputValue
                        } else {
                            Operand::Number(
                                operand.parse::<u32>().expect("Failed to parse operand"),
                            )
                        }
                    },
                    modulo,
                    true_monkey: true_monkey
                        .parse::<usize>()
                        .expect("Failed to parse true monkey"),
                    false_monkey: false_monkey
                        .parse::<usize>()
                        .expect("Failed to parse false monkey"),
                    items_inspected_count: 0,
                }
            },
        )(input)
    }

    fn take_turn(&mut self, reduce_worry: bool) -> HashMap<usize, Vec<Modulo>> {
        let mut thrown_items: HashMap<usize, Vec<Modulo>> = HashMap::new();
        for mut item in self.items.drain(0..) {
            self.items_inspected_count += 1;
            match self.operation {
                Operation::Addition => {
                    let operand_value = match self.operand {
                        Operand::Number(num) => num,
                        _ => panic!("Can't add value to itself"),
                    };
                    item.add(operand_value)
                }
                Operation::Multiplication => match self.operand {
                    Operand::Number(num) => item.multiply(num),
                    Operand::InputValue => item.multiply_self(),
                },
            }
            if reduce_worry {
                item.divide(3);
            }
            if item.divisible_by(self.modulo) {
                let entry = thrown_items.entry(self.true_monkey).or_default();
                entry.push(item);
            } else {
                let entry = thrown_items.entry(self.false_monkey).or_default();
                entry.push(item);
            }
        }
        thrown_items
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let (_, mut monkeys) = map(many_till(Monkey::parse, eof), |(monkey, _)| monkey)(input)
        .expect("Failed to parse monkeys");
    let modulos = monkeys.iter().map(|m| m.modulo).collect::<Vec<u32>>();
    for monkey in monkeys.iter_mut() {
        for item in monkey.items.iter_mut() {
            for modulo in &modulos {
                item.add_modulo(*modulo, item.u32_value % modulo);
            }
        }
    }
    monkeys
}

fn run_round(monkeys: &mut [Monkey], reduce_worry: bool) {
    let mut i = 0;
    while i < monkeys.len() {
        let monkey = &mut monkeys[i];
        let mut thrown_items = monkey.take_turn(reduce_worry);
        for (j, items) in thrown_items.drain() {
            monkeys[j].items.extend(items);
        }
        i += 1;
    }
}

fn calc_monkey_business(monkeys: &mut [Monkey]) -> usize {
    monkeys.sort_by_key(|m| m.items_inspected_count);
    monkeys[monkeys.len() - 1].items_inspected_count
        * monkeys[monkeys.len() - 2].items_inspected_count
}

#[test]
fn test_parse_monkeys() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let monkeys = parse_monkeys(&contents);
    assert_eq!(monkeys.len(), 4);

    assert_eq!(
        monkeys[0]
            .items
            .iter()
            .filter(|i| i.u32_is_valid)
            .map(|i| i.u32_value)
            .collect::<Vec<u32>>(),
        vec![79, 98]
    );
    assert_eq!(monkeys[0].operation, Operation::Multiplication);
    assert_eq!(monkeys[0].operand, Operand::Number(19));
    assert_eq!(monkeys[0].modulo, 23);
    assert_eq!(monkeys[0].true_monkey, 2);
    assert_eq!(monkeys[0].false_monkey, 3);
}

#[test]
fn test_take_turn() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let mut monkeys = parse_monkeys(&contents);
    let thrown_items = monkeys[0].take_turn(true);

    assert!(monkeys[0].items.is_empty());
    assert_eq!(thrown_items.len(), 1);
    assert!(thrown_items.contains_key(&3));
    assert_eq!(thrown_items[&3].len(), 2);
    assert_eq!(
        thrown_items[&3]
            .iter()
            .filter(|i| i.u32_is_valid)
            .map(|i| i.u32_value)
            .collect::<Vec<u32>>(),
        vec![500, 620]
    );
}

#[test]
fn test_run_round() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let mut monkeys = parse_monkeys(&contents);
    run_round(&mut monkeys, true);

    assert_eq!(monkeys[0].items.len(), 4);
    assert_eq!(
        monkeys[0]
            .items
            .iter()
            .filter(|i| i.u32_is_valid)
            .map(|i| i.u32_value)
            .collect::<Vec<u32>>(),
        vec![20, 23, 27, 26]
    );
    assert_eq!(monkeys[1].items.len(), 6);
    assert_eq!(
        monkeys[1]
            .items
            .iter()
            .filter(|i| i.u32_is_valid)
            .map(|i| i.u32_value)
            .collect::<Vec<u32>>(),
        vec![2080, 25, 167, 207, 401, 1046]
    );
    assert!(monkeys[2].items.is_empty());
    assert!(monkeys[3].items.is_empty());
}

#[test]
fn test_items_inspected_count() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let mut monkeys = parse_monkeys(&contents);
    for _ in 0..20 {
        run_round(&mut monkeys, true);
    }
    assert_eq!(monkeys[0].items_inspected_count, 101);
    assert_eq!(monkeys[1].items_inspected_count, 95);
    assert_eq!(monkeys[2].items_inspected_count, 7);
    assert_eq!(monkeys[3].items_inspected_count, 105);
}

#[test]
fn test_items_inspected_count_stages() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let mut monkeys = parse_monkeys(&contents);

    run_round(&mut monkeys, false);
    assert_eq!(monkeys[0].items_inspected_count, 2);
    assert_eq!(monkeys[1].items_inspected_count, 4);
    assert_eq!(monkeys[2].items_inspected_count, 3);
    assert_eq!(monkeys[3].items_inspected_count, 6);

    for _ in 0..19 {
        run_round(&mut monkeys, false);
    }
    assert_eq!(monkeys[0].items_inspected_count, 99);
    assert_eq!(monkeys[1].items_inspected_count, 97);
    assert_eq!(monkeys[2].items_inspected_count, 8);
    assert_eq!(monkeys[3].items_inspected_count, 103);
}

#[test]
fn test_monkey_business() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let mut monkeys = parse_monkeys(&contents);
    for _ in 0..20 {
        run_round(&mut monkeys, true);
    }
    let monkey_business = calc_monkey_business(&mut monkeys);
    assert_eq!(monkey_business, 10605);
}

#[test]
fn test_modulo() {
    let modulos = vec![3, 7, 11];
    let mut a = Modulo::new(9, &modulos);

    assert!(a.divisible_by(3));
    assert!(!a.divisible_by(7));
    assert!(!a.divisible_by(11));

    a.add(2);
    assert!(!a.divisible_by(3));
    assert!(!a.divisible_by(7));
    assert!(a.divisible_by(11));

    a.multiply(7);
    assert!(!a.divisible_by(3));
    assert!(a.divisible_by(7));
    assert!(a.divisible_by(11));
}

#[test]
fn test_worrisome_monkey_business() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let mut monkeys = parse_monkeys(&contents);
    for _ in 0..10000 {
        run_round(&mut monkeys, false);
    }
    let monkey_business = calc_monkey_business(&mut monkeys);
    assert_eq!(monkey_business, 2713310158);
}
