use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::{eof, map, opt},
    multi::{many_till, separated_list0},
    sequence::tuple,
    IResult,
};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Zero};
use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let (_, mut monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
    for _ in 0..20 {
        run_round(&mut monkeys, true);
    }
    let monkey_business = calc_monkey_business(&mut monkeys);
    format!("{}", monkey_business)
}

pub fn part2() -> String {
    let _contents = get_input_file_contents();
    let contents = get_input_file_contents();
    let (_, mut monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
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
    Number(BigUint),
    InputValue,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<BigUint>,
    operation: Operation,
    operand: Operand,
    modulo: BigUint,
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
            )| Self {
                items: items
                    .iter()
                    .map(|i| i.parse::<BigUint>().expect("Failed to parse item"))
                    .collect::<Vec<BigUint>>(),
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
                            operand.parse::<BigUint>().expect("Failed to parse operand"),
                        )
                    }
                },
                modulo: modulo.parse::<BigUint>().expect("Failed to parse modulo"),
                true_monkey: true_monkey
                    .parse::<usize>()
                    .expect("Failed to parse true monkey"),
                false_monkey: false_monkey
                    .parse::<usize>()
                    .expect("Failed to parse false monkey"),
                items_inspected_count: 0,
            },
        )(input)
    }

    fn take_turn(&mut self, reduce_worry: bool) -> HashMap<usize, Vec<BigUint>> {
        let mut thrown_items: HashMap<usize, Vec<BigUint>> = HashMap::new();
        for item in self.items.drain(0..) {
            self.items_inspected_count += 1;
            let operand_value = match &self.operand {
                Operand::Number(num) => num.clone(),
                Operand::InputValue => item.clone(),
            };
            let mut new_item_value = match self.operation {
                Operation::Addition => item + operand_value,
                Operation::Multiplication => item * operand_value,
            };
            if reduce_worry {
                new_item_value = new_item_value / BigUint::from_u64(3).unwrap();
            }
            if &new_item_value % &self.modulo == Zero::zero() {
                let entry = thrown_items.entry(self.true_monkey).or_default();
                entry.push(new_item_value);
            } else {
                let entry = thrown_items.entry(self.false_monkey).or_default();
                entry.push(new_item_value);
            }
        }
        thrown_items
    }
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    map(many_till(Monkey::parse, eof), |(monkey, _)| monkey)(input)
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
    let (rest, monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
    assert!(rest.is_empty());
    assert_eq!(monkeys.len(), 4);

    assert_eq!(
        monkeys[0].items,
        vec![
            BigUint::from_u64(79).unwrap(),
            BigUint::from_u64(98).unwrap()
        ]
    );
    assert_eq!(monkeys[0].operation, Operation::Multiplication);
    assert_eq!(
        monkeys[0].operand,
        Operand::Number(BigUint::from_u64(19).unwrap())
    );
    assert_eq!(monkeys[0].modulo, BigUint::from_u64(23).unwrap());
    assert_eq!(monkeys[0].true_monkey, 2);
    assert_eq!(monkeys[0].false_monkey, 3);
}

#[test]
fn test_take_turn() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let (_, mut monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
    let thrown_items = monkeys[0].take_turn(true);

    assert!(monkeys[0].items.is_empty());
    assert_eq!(thrown_items.len(), 1);
    assert!(thrown_items.contains_key(&3));
    assert_eq!(
        thrown_items[&3],
        vec![
            BigUint::from_u64(500).unwrap(),
            BigUint::from_u64(620).unwrap()
        ]
    );
}

#[test]
fn test_run_round() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let (_, mut monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
    run_round(&mut monkeys, true);

    assert_eq!(monkeys[0].items.len(), 4);
    assert_eq!(
        monkeys[0].items,
        vec![
            BigUint::from_u64(20).unwrap(),
            BigUint::from_u64(23).unwrap(),
            BigUint::from_u64(27).unwrap(),
            BigUint::from_u64(26).unwrap()
        ]
    );
    assert_eq!(monkeys[1].items.len(), 6);
    assert_eq!(
        monkeys[1].items,
        vec![
            BigUint::from_u64(2080).unwrap(),
            BigUint::from_u64(25).unwrap(),
            BigUint::from_u64(167).unwrap(),
            BigUint::from_u64(207).unwrap(),
            BigUint::from_u64(401).unwrap(),
            BigUint::from_u64(1046).unwrap()
        ]
    );
    assert!(monkeys[2].items.is_empty());
    assert!(monkeys[3].items.is_empty());
}

#[test]
fn test_items_inspected_count() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let (_, mut monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
    for _ in 0..20 {
        run_round(&mut monkeys, true);
    }
    assert_eq!(monkeys[0].items_inspected_count, 101);
    assert_eq!(monkeys[1].items_inspected_count, 95);
    assert_eq!(monkeys[2].items_inspected_count, 7);
    assert_eq!(monkeys[3].items_inspected_count, 105);
}

#[test]
fn test_monkey_business() {
    let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
    let (_, mut monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
    for _ in 0..20 {
        run_round(&mut monkeys, true);
    }
    let monkey_business = calc_monkey_business(&mut monkeys);
    assert_eq!(monkey_business, 10605);
}

// #[test]
// fn test_worrisome_monkey_business() {
//     let contents = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\n";
//     let (_, mut monkeys) = parse_monkeys(&contents).expect("Failed to parse monkeys");
//     for _ in 0..10000 {
//         run_round(&mut monkeys, false);
//     }
//     let monkey_business = calc_monkey_business(&mut monkeys);
//     assert_eq!(monkey_business, 2713310158);
// }
