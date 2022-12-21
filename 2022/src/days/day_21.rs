use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, newline},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let monkeys = parse_monkeys(&contents);
    let root_number = calc_root_number(&monkeys, None, false);
    format!("{}", root_number)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let monkeys = parse_monkeys(&contents);
    let humn_input = bisect_humn_input(&monkeys);
    format!("{}", humn_input)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input21").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(anychar, |ch| match ch {
            '+' => Self::Add,
            '-' => Self::Subtract,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            _ => panic!("invalid operator: {}", ch),
        })(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Operation {
    operator: Operator,
    operands: (String, String),
}

#[derive(Debug, PartialEq)]
enum MonkeyOutput {
    Operation(Operation),
    Number(f64),
}

impl MonkeyOutput {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(digit1, |d: &str| {
                Self::Number(d.parse::<f64>().expect("failed to parse number"))
            }),
            map(
                tuple((alpha1, tag(" "), Operator::parse, tag(" "), alpha1)),
                |(operand1, _, operator, _, operand2)| {
                    Self::Operation(Operation {
                        operator,
                        operands: (operand1.into(), operand2.into()),
                    })
                },
            ),
        ))(input)
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    name: String,
    output: MonkeyOutput,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((alpha1, tag(": "), MonkeyOutput::parse)),
            |(name, _, output)| Self {
                name: name.to_string(),
                output,
            },
        )(input)
    }
}

fn parse_monkeys(contents: &str) -> Vec<Monkey> {
    let (rest, monkeys) =
        terminated(separated_list1(newline, Monkey::parse), opt(newline))(contents)
            .expect("Failed to parse monkeys");
    assert!(rest.is_empty());
    monkeys
}

fn calc_root_number(monkeys: &[Monkey], humn_override: Option<f64>, root_override: bool) -> f64 {
    let mut monkey_numbers: HashMap<String, f64> = HashMap::new();
    loop {
        let mut monkeys_updated = 0;
        for monkey in monkeys {
            if monkey_numbers.contains_key(&monkey.name) {
                continue;
            }
            match &monkey.output {
                MonkeyOutput::Number(num) => {
                    if humn_override.is_some() && monkey.name == "humn" {
                        monkey_numbers.insert(monkey.name.clone(), humn_override.unwrap());
                    } else {
                        monkey_numbers.insert(monkey.name.clone(), *num);
                    }
                    monkeys_updated += 1;
                }
                MonkeyOutput::Operation(operation) => {
                    let (operand1, operand2) = &operation.operands;
                    if monkey_numbers.contains_key(operand1)
                        && monkey_numbers.contains_key(operand2)
                    {
                        let input1 = monkey_numbers[operand1];
                        let input2 = monkey_numbers[operand2];
                        let result = if root_override && monkey.name == "root" {
                            input1 - input2
                        } else {
                            match operation.operator {
                                Operator::Add => input1 + input2,
                                Operator::Subtract => input1 - input2,
                                Operator::Multiply => input1 * input2,
                                Operator::Divide => input1 / input2,
                            }
                        };
                        monkey_numbers.insert(monkey.name.clone(), result);
                        monkeys_updated += 1;
                    }
                }
            }
        }
        if monkeys_updated == 0 {
            break;
        }
    }
    monkey_numbers["root"]
}

fn bisect_humn_input(monkeys: &[Monkey]) -> f64 {
    let mut a = 0.0;
    let mut b = f64::MAX;
    let mut c: f64 = (a + b) / 2.0;
    loop {
        let c_value = calc_root_number(monkeys, Some(c), true);
        if (c - a) < 0.01 {
            break;
        }

        let a_value = calc_root_number(monkeys, Some(a), true);
        let b_value = calc_root_number(monkeys, Some(b), true);
        if a_value < 0.0 && b_value > 0.0 {
            if c_value > 0.0 {
                b = c;
            } else {
                a = c;
            }
        } else {
            if c_value > 0.0 {
                a = c;
            } else {
                b = c;
            }
        }
        c = (a + b) / 2.0;
    }
    c.round()
}

#[test]
fn test_parse_monkey() {
    {
        let input = "root: pppw + sjmn";
        let result = Monkey::parse(input);
        assert!(result.is_ok());
        let (rest, monkey) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            monkey,
            Monkey {
                name: "root".into(),
                output: MonkeyOutput::Operation(Operation {
                    operator: Operator::Add,
                    operands: ("pppw".into(), "sjmn".into())
                })
            }
        );
    }

    {
        let input = "ptdq: humn - dvpt";
        let result = Monkey::parse(input);
        assert!(result.is_ok());
        let (rest, monkey) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            monkey,
            Monkey {
                name: "ptdq".into(),
                output: MonkeyOutput::Operation(Operation {
                    operator: Operator::Subtract,
                    operands: ("humn".into(), "dvpt".into())
                })
            }
        );
    }

    {
        let input = "sjmn: drzm * dbpl";
        let result = Monkey::parse(input);
        assert!(result.is_ok());
        let (rest, monkey) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            monkey,
            Monkey {
                name: "sjmn".into(),
                output: MonkeyOutput::Operation(Operation {
                    operator: Operator::Multiply,
                    operands: ("drzm".into(), "dbpl".into())
                })
            }
        );
    }

    {
        let input = "pppw: cczh / lfqf";
        let result = Monkey::parse(input);
        assert!(result.is_ok());
        let (rest, monkey) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            monkey,
            Monkey {
                name: "pppw".into(),
                output: MonkeyOutput::Operation(Operation {
                    operator: Operator::Divide,
                    operands: ("cczh".into(), "lfqf".into())
                })
            }
        );
    }

    {
        let input = "dbpl: 5";
        let result = Monkey::parse(input);
        assert!(result.is_ok());
        let (rest, monkey) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            monkey,
            Monkey {
                name: "dbpl".into(),
                output: MonkeyOutput::Number(5.0)
            }
        );
    }
}

#[test]
fn test_parse_monkeys() {
    let contents = "root: pppw + sjmn\ndbpl: 5\n";
    let monkeys = parse_monkeys(contents);
    assert_eq!(monkeys.len(), 2);

    assert_eq!(
        monkeys[0],
        Monkey {
            name: "root".into(),
            output: MonkeyOutput::Operation(Operation {
                operator: Operator::Add,
                operands: ("pppw".into(), "sjmn".into())
            })
        }
    );

    assert_eq!(
        monkeys[1],
        Monkey {
            name: "dbpl".into(),
            output: MonkeyOutput::Number(5.0)
        }
    );
}

#[test]
fn test_calc_root_number() {
    let contents = "root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32\n";
    let monkeys = parse_monkeys(contents);
    let root_number = calc_root_number(&monkeys, None, false);
    assert_eq!(root_number, 152.0);
}

#[test]
fn test_bisect_humn_input() {
    let contents = "root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32\n";
    let monkeys = parse_monkeys(contents);
    let humn_input = bisect_humn_input(&monkeys);
    assert_eq!(humn_input, 301.0);
}
