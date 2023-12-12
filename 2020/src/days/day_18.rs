use std::{collections::VecDeque, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let expressions = parse_expressions(&input, Precedence::LeftRight);
    expressions
        .iter()
        .map(|e| e.evaluate())
        .sum::<u64>()
        .to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let expressions = parse_expressions(&input, Precedence::AddMult);
    expressions
        .iter()
        .map(|e| e.evaluate())
        .sum::<u64>()
        .to_string()
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    BinOp(Box<BinOp>),
    Literal(u64),
}

#[derive(Debug, PartialEq, Clone)]
struct BinOp {
    op: Op,
    left: Expr,
    right: Expr,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Mult,
}

#[derive(Debug, Clone, Copy)]
enum Precedence {
    LeftRight,
    AddMult,
}

impl Expr {
    fn parse(input: &str, precedence: Precedence) -> (Self, usize) {
        let mut sub_expressions = VecDeque::new();
        let mut ops = VecDeque::new();

        let mut i = 0;
        while i < input.len() {
            let ch = &input[i..i + 1].chars().next().unwrap();
            match ch {
                '0'..='9' => {
                    sub_expressions.push_back(Expr::Literal((*ch as u8 - b'0') as u64));
                    i += 1;
                }
                '(' => {
                    // Recursively parse the whole sub-expression
                    i += 1;
                    let (sub_expr, chars_parsed) = Expr::parse(&input[i..], precedence);
                    sub_expressions.push_back(sub_expr);
                    i += chars_parsed;
                }
                ')' => {
                    // We're done parsing a sub-expression
                    i += 1;
                    break;
                }
                '+' => {
                    ops.push_back(Op::Add);
                    i += 1;
                }
                '*' => {
                    ops.push_back(Op::Mult);
                    i += 1;
                }
                ' ' => {
                    i += 1;
                }
                _ => {
                    panic!("invalid character in input");
                }
            }
        }
        // End of string, we're done parsing the expression
        if sub_expressions.len() == 1 && ops.is_empty() {
            // Literal
            (sub_expressions.pop_front().unwrap(), i)
        } else if sub_expressions.len() == ops.len() + 1 {
            // Binary op(s)
            match precedence {
                Precedence::LeftRight => {
                    let mut expr = Expr::BinOp(Box::new(BinOp {
                        op: ops.pop_front().unwrap(),
                        left: sub_expressions.pop_front().unwrap(),
                        right: sub_expressions.pop_front().unwrap(),
                    }));
                    while let Some(op) = ops.pop_front() {
                        expr = Expr::BinOp(Box::new(BinOp {
                            op,
                            left: expr,
                            right: sub_expressions.pop_front().unwrap(),
                        }));
                    }
                    (expr, i)
                }
                Precedence::AddMult => {
                    // Do all the additions first, left to right
                    let mut add_ops = 0;
                    for (i, op) in ops.iter().enumerate().filter(|(_, o)| **o == Op::Add) {
                        let expr = Expr::BinOp(Box::new(BinOp {
                            op: *op,
                            left: sub_expressions.remove(i - add_ops).unwrap(),
                            right: sub_expressions.remove(i - add_ops).unwrap(),
                        }));
                        sub_expressions.insert(i - add_ops, expr);
                        add_ops += 1;
                    }
                    ops.retain(|o| *o != Op::Add);
                    // Then the multiplications, left to right
                    let mut mul_ops = 0;
                    for (i, op) in ops.iter().enumerate().filter(|(_, o)| **o == Op::Mult) {
                        let expr = Expr::BinOp(Box::new(BinOp {
                            op: *op,
                            left: sub_expressions.remove(i - mul_ops).unwrap(),
                            right: sub_expressions.remove(i - mul_ops).unwrap(),
                        }));
                        sub_expressions.insert(i - mul_ops, expr);
                        mul_ops += 1;
                    }
                    assert_eq!(sub_expressions.len(), 1);
                    (sub_expressions.pop_back().unwrap(), i)
                }
            }
        } else {
            panic!("invalid expression")
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Expr::BinOp(bin_op) => bin_op.evaluate(),
            Expr::Literal(literal) => *literal,
        }
    }
}

impl BinOp {
    fn evaluate(&self) -> u64 {
        match self.op {
            Op::Add => self.left.evaluate() + self.right.evaluate(),
            Op::Mult => self.left.evaluate() * self.right.evaluate(),
        }
    }
}

fn parse_expressions(input: &str, precedence: Precedence) -> Vec<Expr> {
    input
        .lines()
        .map(|line| Expr::parse(line, precedence).0)
        .collect()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input18").expect("Failed to open input file")
}

#[test]
fn test_parse_expression_leftright() {
    {
        let input = "1";
        let expression = Expr::parse(input, Precedence::LeftRight).0;
        assert_eq!(expression, Expr::Literal(1));
    }

    {
        let input = "1 + 2";
        let expression = Expr::parse(input, Precedence::LeftRight).0;
        assert_eq!(
            expression,
            Expr::BinOp(Box::new(BinOp {
                op: Op::Add,
                left: Expr::Literal(1),
                right: Expr::Literal(2)
            }))
        );
    }

    {
        let input = "1 + (2 * 3)";
        let expression = Expr::parse(input, Precedence::LeftRight).0;
        assert_eq!(
            expression,
            Expr::BinOp(Box::new(BinOp {
                op: Op::Add,
                left: Expr::Literal(1),
                right: Expr::BinOp(Box::new(BinOp {
                    op: Op::Mult,
                    left: Expr::Literal(2),
                    right: Expr::Literal(3),
                }))
            }))
        );
    }

    {
        let input = "1 + 2 * 3";
        let expression = Expr::parse(input, Precedence::LeftRight).0;
        assert_eq!(
            expression,
            Expr::BinOp(Box::new(BinOp {
                op: Op::Mult,
                left: Expr::BinOp(Box::new(BinOp {
                    op: Op::Add,
                    left: Expr::Literal(1),
                    right: Expr::Literal(2),
                })),
                right: Expr::Literal(3),
            }))
        );
    }

    {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let expression = Expr::parse(input, Precedence::LeftRight).0;
        assert_eq!(
            expression,
            Expr::BinOp(Box::new(BinOp {
                op: Op::Add,
                left: Expr::BinOp(Box::new(BinOp {
                    op: Op::Add,
                    left: Expr::Literal(1),
                    right: Expr::BinOp(Box::new(BinOp {
                        op: Op::Mult,
                        left: Expr::Literal(2),
                        right: Expr::Literal(3)
                    }))
                })),
                right: Expr::BinOp(Box::new(BinOp {
                    op: Op::Mult,
                    left: Expr::Literal(4),
                    right: Expr::BinOp(Box::new(BinOp {
                        op: Op::Add,
                        left: Expr::Literal(5),
                        right: Expr::Literal(6)
                    }))
                }))
            }))
        );
    }
}

#[test]
fn test_parse_expression_addmult() {
    {
        let input = "1";
        let expression = Expr::parse(input, Precedence::AddMult).0;
        assert_eq!(expression, Expr::Literal(1));
    }

    {
        let input = "1 + 2";
        let expression = Expr::parse(input, Precedence::AddMult).0;
        assert_eq!(
            expression,
            Expr::BinOp(Box::new(BinOp {
                op: Op::Add,
                left: Expr::Literal(1),
                right: Expr::Literal(2)
            }))
        );
    }

    {
        let input = "1 + (2 * 3)";
        let expression = Expr::parse(input, Precedence::AddMult).0;
        assert_eq!(
            expression,
            Expr::BinOp(Box::new(BinOp {
                op: Op::Add,
                left: Expr::Literal(1),
                right: Expr::BinOp(Box::new(BinOp {
                    op: Op::Mult,
                    left: Expr::Literal(2),
                    right: Expr::Literal(3),
                }))
            }))
        );
    }

    {
        let input = "1 * 2 + 3";
        let expression = Expr::parse(input, Precedence::AddMult).0;
        assert_eq!(
            expression,
            Expr::BinOp(Box::new(BinOp {
                op: Op::Mult,
                left: Expr::Literal(1),
                right: Expr::BinOp(Box::new(BinOp {
                    op: Op::Add,
                    left: Expr::Literal(2),
                    right: Expr::Literal(3),
                })),
            }))
        );
    }
}

#[test]
fn test_evalulate_parsed_expression() {
    assert_eq!(
        Expr::BinOp(Box::new(BinOp {
            op: Op::Add,
            left: Expr::BinOp(Box::new(BinOp {
                op: Op::Add,
                left: Expr::Literal(1),
                right: Expr::BinOp(Box::new(BinOp {
                    op: Op::Mult,
                    left: Expr::Literal(2),
                    right: Expr::Literal(3)
                }))
            })),
            right: Expr::BinOp(Box::new(BinOp {
                op: Op::Mult,
                left: Expr::Literal(4),
                right: Expr::BinOp(Box::new(BinOp {
                    op: Op::Add,
                    left: Expr::Literal(5),
                    right: Expr::Literal(6)
                }))
            }))
        }))
        .evaluate(),
        51
    );
}

#[test]
fn test_parse_and_evaluate_expression_leftright() {
    assert_eq!(
        Expr::parse("1 + 2 * 3 + 4 * 5 + 6", Precedence::LeftRight)
            .0
            .evaluate(),
        71
    );
    assert_eq!(
        Expr::parse("1 + (2 * 3) + (4 * (5 + 6))", Precedence::LeftRight)
            .0
            .evaluate(),
        51
    );
    assert_eq!(
        Expr::parse("2 * 3 + (4 * 5)", Precedence::LeftRight)
            .0
            .evaluate(),
        26
    );
    assert_eq!(
        Expr::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)", Precedence::LeftRight)
            .0
            .evaluate(),
        437
    );
    assert_eq!(
        Expr::parse(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            Precedence::LeftRight
        )
        .0
        .evaluate(),
        12240
    );
    assert_eq!(
        Expr::parse(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            Precedence::LeftRight
        )
        .0
        .evaluate(),
        13632
    );
}

#[test]
fn test_parse_and_evaluate_expression_addmult() {
    assert_eq!(
        Expr::parse("1 + 2 * 3 + 4 * 5 + 6", Precedence::AddMult)
            .0
            .evaluate(),
        231
    );

    assert_eq!(
        Expr::parse("1 + (2 * 3) + (4 * (5 + 6))", Precedence::AddMult)
            .0
            .evaluate(),
        51
    );
    assert_eq!(
        Expr::parse("2 * 3 + (4 * 5)", Precedence::AddMult)
            .0
            .evaluate(),
        46
    );
    assert_eq!(
        Expr::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)", Precedence::AddMult)
            .0
            .evaluate(),
        1445
    );
    assert_eq!(
        Expr::parse(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) ",
            Precedence::AddMult
        )
        .0
        .evaluate(),
        669060
    );
    assert_eq!(
        Expr::parse(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            Precedence::AddMult
        )
        .0
        .evaluate(),
        23340
    );
}
