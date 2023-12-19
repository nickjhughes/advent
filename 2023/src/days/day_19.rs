use hashbrown::HashMap;
use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let (workflows, parts) = parse(&input);
    parts
        .iter()
        .filter(|p| p.is_accepted(&workflows))
        .map(|p| p.components_sum())
        .sum::<u32>()
        .to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let (workflows, _) = parse(&input);
    acceptance_combinations(&workflows).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input19").expect("Failed to open input file")
}

fn parse(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let mut lines = input.lines();

    let mut workflows = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let workflow = Workflow::parse(line);
        workflows.insert(workflow.name, workflow);
    }

    let mut parts = Vec::new();
    for line in lines {
        parts.push(Part::parse(line));
    }

    (workflows, parts)
}

fn acceptance_combinations(workflows: &HashMap<&str, Workflow>) -> u64 {
    let mut total_combinations = 0;
    for ranges in acceptance_ranges(workflows).iter() {
        let mut combinations = 1;
        for (comp_min, comp_max) in ranges.iter() {
            combinations *= (comp_max - comp_min + 1) as u64;
        }
        total_combinations += combinations;
    }
    total_combinations
}

fn acceptance_ranges(workflows: &HashMap<&str, Workflow>) -> Vec<[(u32, u32); 4]> {
    let mut all_ranges = Vec::new();
    for workflow in workflows.values() {
        for (i, rule) in workflow.rules.iter().enumerate() {
            if rule.workflow() == "A" {
                let mut ranges = [(1, 4000), (1, 4000), (1, 4000), (1, 4000)];

                // The current rule must be true
                rule.modify_ranges(&mut ranges, true);
                // Each previous rule in this workflow must be false
                for prev_rule in workflow.rules.iter().take(i) {
                    prev_rule.modify_ranges(&mut ranges, false);
                }

                // Now follow the previous workflows back to "in"
                let mut cur_workflow_name = workflow.name;
                loop {
                    let mut prev_workflow = None;
                    let mut prev_workflow_rule_idx = None;
                    for maybe_prev_workflow in workflows.values() {
                        for (j, prev_workflow_rule) in maybe_prev_workflow.rules.iter().enumerate()
                        {
                            if prev_workflow_rule.workflow() == cur_workflow_name {
                                prev_workflow = Some(maybe_prev_workflow);
                                prev_workflow_rule_idx = Some(j);
                                break;
                            }
                        }
                        if prev_workflow.is_some() {
                            break;
                        }
                    }
                    let prev_workflow = prev_workflow.unwrap();
                    let prev_workflow_rule_idx = prev_workflow_rule_idx.unwrap();

                    // Modify ranges
                    prev_workflow.rules[prev_workflow_rule_idx].modify_ranges(&mut ranges, true);
                    for prev_workflow_rule in
                        prev_workflow.rules.iter().take(prev_workflow_rule_idx)
                    {
                        prev_workflow_rule.modify_ranges(&mut ranges, false);
                    }

                    if prev_workflow.name == "in" {
                        break;
                    }
                    cur_workflow_name = prev_workflow.name;
                }

                all_ranges.push(ranges);
            }
        }
    }
    all_ranges
}

#[derive(Debug, PartialEq)]
struct Workflow<'input> {
    name: &'input str,
    rules: Vec<Rule<'input>>,
}

#[derive(Debug, PartialEq)]
enum Rule<'input> {
    Comparison {
        component: Component,
        cmp_op: CmpOp,
        value: u32,
        workflow: &'input str,
    },
    Direct(&'input str),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Component {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq)]
enum CmpOp {
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl<'input> Workflow<'input> {
    fn parse(input: &'input str) -> Self {
        let (name, rules_str) = input.split_once('{').unwrap();
        let rules = rules_str
            .trim_end_matches('}')
            .split(',')
            .map(Rule::parse)
            .collect();
        Workflow { name, rules }
    }

    fn process(&self, part: &Part) -> &str {
        for rule in self.rules.iter() {
            if rule.matches(part) {
                return rule.workflow();
            }
        }
        panic!("ran out of rules")
    }
}

impl<'input> Rule<'input> {
    fn parse(input: &'input str) -> Self {
        if let Some((cmp, workflow)) = input.split_once(':') {
            if let Some((comp_str, value_str)) = cmp.split_once('<') {
                Rule::Comparison {
                    component: Component::parse(comp_str),
                    cmp_op: CmpOp::LessThan,
                    value: value_str.parse::<u32>().unwrap(),
                    workflow,
                }
            } else if let Some((comp_str, value_str)) = cmp.split_once('>') {
                Rule::Comparison {
                    component: Component::parse(comp_str),
                    cmp_op: CmpOp::GreaterThan,
                    value: value_str.parse::<u32>().unwrap(),
                    workflow,
                }
            } else {
                panic!("invalid comparison operator in rule {input}")
            }
        } else {
            Rule::Direct(input)
        }
    }

    fn workflow(&self) -> &str {
        match self {
            Rule::Comparison { workflow, .. } => workflow,
            Rule::Direct(workflow) => workflow,
        }
    }

    fn matches(&self, part: &Part) -> bool {
        match self {
            Rule::Comparison {
                component,
                cmp_op,
                value,
                ..
            } => match cmp_op {
                CmpOp::GreaterThan => part.get_component(component) > *value,
                CmpOp::LessThan => part.get_component(component) < *value,
            },
            Rule::Direct(_) => true,
        }
    }

    fn modify_ranges(&self, ranges: &mut [(u32, u32); 4], result: bool) {
        match self {
            Rule::Comparison {
                component,
                cmp_op,
                value,
                ..
            } => match cmp_op {
                CmpOp::GreaterThan => {
                    if result {
                        ranges[*component as usize].0 = ranges[*component as usize].0.max(value + 1)
                    } else {
                        ranges[*component as usize].1 = ranges[*component as usize].1.min(*value)
                    }
                }
                CmpOp::LessThan => {
                    if result {
                        ranges[*component as usize].1 = ranges[*component as usize].1.min(value - 1)
                    } else {
                        ranges[*component as usize].0 = ranges[*component as usize].0.max(*value)
                    }
                }
            },
            Rule::Direct(_) => {}
        }
    }
}

impl Part {
    fn parse(input: &str) -> Self {
        let mut parts = input
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',');
        Part {
            x: parts
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap(),
            m: parts
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap(),
            a: parts
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap(),
            s: parts
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap(),
        }
    }

    fn components_sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn get_component(&self, component: &Component) -> u32 {
        match component {
            Component::X => self.x,
            Component::M => self.m,
            Component::A => self.a,
            Component::S => self.s,
        }
    }

    fn is_accepted(&self, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut workflow_name = "in";
        while workflow_name != "R" && workflow_name != "A" {
            let workflow = workflows.get(workflow_name).expect("workflow not found");
            workflow_name = workflow.process(self);
        }
        workflow_name == "A"
    }
}

impl Component {
    fn parse(input: &str) -> Self {
        match input {
            "x" => Component::X,
            "m" => Component::M,
            "a" => Component::A,
            "s" => Component::S,
            _ => panic!("invalid component {input}"),
        }
    }
}

#[test]
fn test_parse() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}\n";
    let (workflows, parts) = parse(input);
    assert_eq!(workflows.len(), 11);
    assert_eq!(parts.len(), 5);
}

#[test]
fn test_parse_workflow() {
    let input = "rfg{s<537:gd,x>2440:R,A}";
    let workflow = Workflow::parse(input);
    assert_eq!(
        workflow,
        Workflow {
            name: "rfg",
            rules: vec![
                Rule::Comparison {
                    component: Component::S,
                    cmp_op: CmpOp::LessThan,
                    value: 537,
                    workflow: "gd"
                },
                Rule::Comparison {
                    component: Component::X,
                    cmp_op: CmpOp::GreaterThan,
                    value: 2440,
                    workflow: "R"
                },
                Rule::Direct("A")
            ]
        }
    );
}

#[test]
fn test_parse_part() {
    let input = "{x=787,m=2655,a=1222,s=2876}";
    let part = Part::parse(input);
    assert_eq!(
        part,
        Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876
        }
    );
}

#[test]
fn test_part_is_accepted() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}\n";
    let (workflows, parts) = parse(input);

    assert!(parts[0].is_accepted(&workflows));
    assert!(!parts[1].is_accepted(&workflows));
    assert!(parts[2].is_accepted(&workflows));
    assert!(!parts[3].is_accepted(&workflows));
    assert!(parts[4].is_accepted(&workflows));
}

#[test]
fn test_sum_components_of_accepted_parts() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}\n";
    let (workflows, parts) = parse(input);
    assert_eq!(
        parts
            .iter()
            .filter(|p| p.is_accepted(&workflows))
            .map(|p| p.components_sum())
            .sum::<u32>(),
        19114
    );
}

#[test]
fn test_acceptance_combinations() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}\n";
    let (workflows, _) = parse(input);
    assert_eq!(acceptance_combinations(&workflows), 167409079868000);
}
