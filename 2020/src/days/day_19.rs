use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let rules_and_messages = RulesAndMessages::parse(&input);
    rules_and_messages
        .messages
        .iter()
        .filter(|msg| rules_and_messages.matches_rule_zero(msg))
        .count()
        .to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut rules_and_messages = RulesAndMessages::parse(&input);
    rules_and_messages.replace();
    rules_and_messages
        .messages
        .iter()
        .filter(|msg| rules_and_messages.matches_rule_zero(msg))
        .count()
        .to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input19").expect("Failed to open input file")
}

#[derive(Debug, PartialEq)]
struct RulesAndMessages {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
    replaced: bool,
}

#[derive(Debug, PartialEq)]
enum Rule {
    Char(char),
    Ref(usize),
    List(Vec<Rule>),
    Or(Box<Rule>, Box<Rule>),
}

impl RulesAndMessages {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        // Rules
        let mut rules = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (index_str, rule_str) = line.split_once(": ").unwrap();
            rules.insert(index_str.parse::<usize>().unwrap(), Rule::parse(rule_str));
        }
        // Messages
        let messages = lines.map(|l| l.to_owned()).collect();
        RulesAndMessages {
            rules,
            messages,
            replaced: false,
        }
    }

    fn replace(&mut self) {
        self.rules.insert(8, Rule::parse("42 | 42 8"));
        self.rules.insert(11, Rule::parse("42 31 | 42 11 31"));
        self.replaced = true;
    }

    fn matches_rule_zero(&self, message: &str) -> bool {
        self.matches(message, self.rules.get(&0).unwrap(), Some(0)) == Some(message.len())
    }

    fn matches(&self, message: &str, rule: &Rule, id: Option<usize>) -> Option<usize> {
        if self.replaced && (id == Some(8) || id == Some(11)) {
            // TODO: Special handling for these
            todo!()
        }

        if message.is_empty() {
            None
        } else {
            match rule {
                Rule::Char(ch) => {
                    if message.starts_with(*ch) {
                        Some(1)
                    } else {
                        None
                    }
                }
                Rule::Ref(id) => self.matches(message, self.rules.get(id).unwrap(), Some(*id)),
                Rule::List(rules) => {
                    let mut message = &message[0..];
                    let mut total_consumed = 0;
                    for subrule in rules {
                        if let Some(consumed) = self.matches(message, subrule, None) {
                            message = &message[consumed..];
                            total_consumed += consumed;
                        } else {
                            return None;
                        }
                    }
                    Some(total_consumed)
                }
                Rule::Or(r1, r2) => self
                    .matches(message, r1, None)
                    .or_else(|| self.matches(message, r2, None)),
            }
        }
    }
}

impl Rule {
    fn parse(input: &str) -> Self {
        if input.starts_with('"') {
            // Single character
            Rule::Char(input.chars().nth(1).unwrap())
        } else if input.contains('|') {
            // Or
            let (first_str, second_str) = input.split_once(" | ").unwrap();
            let first_rule = Rule::parse(first_str);
            let second_rule = Rule::parse(second_str);
            Rule::Or(Box::new(first_rule), Box::new(second_rule))
        } else if input.contains(' ') {
            // List
            Rule::List(input.split_whitespace().map(Rule::parse).collect())
        } else {
            // Ref
            Rule::Ref(input.parse::<usize>().unwrap())
        }
    }
}

#[test]
fn test_parse() {
    let input = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb\n";
    let rules_and_messages = RulesAndMessages::parse(input);
    let rules = {
        let mut map = HashMap::new();
        map.insert(
            0,
            Rule::List(vec![Rule::Ref(4), Rule::Ref(1), Rule::Ref(5)]),
        );
        map.insert(
            1,
            Rule::Or(
                Box::new(Rule::List(vec![Rule::Ref(2), Rule::Ref(3)])),
                Box::new(Rule::List(vec![Rule::Ref(3), Rule::Ref(2)])),
            ),
        );
        map.insert(
            2,
            Rule::Or(
                Box::new(Rule::List(vec![Rule::Ref(4), Rule::Ref(4)])),
                Box::new(Rule::List(vec![Rule::Ref(5), Rule::Ref(5)])),
            ),
        );
        map.insert(
            3,
            Rule::Or(
                Box::new(Rule::List(vec![Rule::Ref(4), Rule::Ref(5)])),
                Box::new(Rule::List(vec![Rule::Ref(5), Rule::Ref(4)])),
            ),
        );
        map.insert(4, Rule::Char('a'));
        map.insert(5, Rule::Char('b'));
        map
    };
    assert_eq!(
        rules_and_messages,
        RulesAndMessages {
            rules,
            messages: vec![
                "ababbb".into(),
                "bababa".into(),
                "abbbab".into(),
                "aaabbb".into(),
                "aaaabbb".into(),
            ],
            replaced: false,
        }
    );
}

#[test]
fn test_rule_matches() {
    let input = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb\n";
    let rules_and_messages = RulesAndMessages::parse(input);
    assert_eq!(
        rules_and_messages
            .messages
            .iter()
            .filter(|msg| rules_and_messages.matches_rule_zero(msg))
            .count(),
        2
    );
}

#[test]
#[ignore]
fn test_rule_matches_replacement() {
    let input = "42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba\n";
    let mut rules_and_messages = RulesAndMessages::parse(input);
    rules_and_messages.replace();

    assert!(rules_and_messages.matches_rule_zero("bbabbbbaabaabba"));
    assert!(rules_and_messages.matches_rule_zero("babbbbaabbbbbabbbbbbaabaaabaaa"));
    assert!(rules_and_messages.matches_rule_zero("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"));
    assert!(rules_and_messages.matches_rule_zero("bbbbbbbaaaabbbbaaabbabaaa"));
    assert!(rules_and_messages.matches_rule_zero("bbbababbbbaaaaaaaabbababaaababaabab"));
    assert!(rules_and_messages.matches_rule_zero("ababaaaaaabaaab"));
    assert!(rules_and_messages.matches_rule_zero("ababaaaaabbbaba"));
    assert!(rules_and_messages.matches_rule_zero("baabbaaaabbaaaababbaababb"));
    assert!(rules_and_messages.matches_rule_zero("abbbbabbbbaaaababbbbbbaaaababb"));
    assert!(rules_and_messages.matches_rule_zero("aaaaabbaabaaaaababaa"));
    assert!(rules_and_messages.matches_rule_zero("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"));
    assert!(rules_and_messages.matches_rule_zero("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"));

    assert!(!rules_and_messages.matches_rule_zero("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"));
    assert!(!rules_and_messages.matches_rule_zero("aaaabbaaaabbaaa"));
    assert!(!rules_and_messages.matches_rule_zero("babaaabbbaaabaababbaabababaaab"));
}
