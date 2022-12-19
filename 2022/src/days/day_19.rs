use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let blueprints = parse_blueprints(&contents);
    let total_quality_level = calc_total_quality_level(24, &blueprints);
    format!("{}", total_quality_level)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let blueprints = parse_blueprints(&contents);
    let most_geodes_product = calc_most_geodes_product(32, &blueprints[0..3]);
    format!("{}", most_geodes_product)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input19").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter, Copy)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    resources: [u32; 4],
    robots: [u32; 4],
    minute: u32,
}

impl State {
    fn new() -> Self {
        let resources = [0, 0, 0, 0];
        let robots = [1, 0, 0, 0];
        Self {
            resources,
            robots,
            minute: 1,
        }
    }

    fn robot_build_options(&self, blueprint: &Blueprint) -> Vec<Option<Resource>> {
        let mut choices = vec![None];
        for robot_type in Resource::iter() {
            let mut can_afford = true;
            for (resource, cost) in &blueprint.robot_costs[&robot_type] {
                if *cost > self.resources[*resource as usize] {
                    can_afford = false;
                }
            }
            if can_afford {
                choices.push(Some(robot_type))
            }
        }
        choices
    }

    fn mine_resources(&mut self) {
        for resource in Resource::iter() {
            self.resources[resource as usize] += self.robots[resource as usize];
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    robot_costs: HashMap<Resource, HashMap<Resource, u32>>,
}

impl Blueprint {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("Blueprint "),
                digit1::<&str, _>,
                tag(": Each ore robot costs "),
                digit1,
                tag(" ore. Each clay robot costs "),
                digit1,
                tag(" ore. Each obsidian robot costs "),
                digit1,
                tag(" ore and "),
                digit1,
                tag(" clay. Each geode robot costs "),
                digit1,
                tag(" ore and "),
                digit1,
                tag(" obsidian."),
            )),
            |(
                _,
                blueprint_id,
                _,
                ore_ore,
                _,
                clay_ore,
                _,
                obsidian_ore,
                _,
                obsidian_clay,
                _,
                geode_ore,
                _,
                geode_obsidian,
                _,
            )| Self {
                id: blueprint_id.parse::<u32>().unwrap(),
                robot_costs: HashMap::from_iter(vec![
                    (
                        Resource::Ore,
                        HashMap::from_iter(vec![(Resource::Ore, ore_ore.parse::<u32>().unwrap())]),
                    ),
                    (
                        Resource::Clay,
                        HashMap::from_iter(vec![(Resource::Ore, clay_ore.parse::<u32>().unwrap())]),
                    ),
                    (
                        Resource::Obsidian,
                        HashMap::from_iter(vec![
                            (Resource::Ore, obsidian_ore.parse::<u32>().unwrap()),
                            (Resource::Clay, obsidian_clay.parse::<u32>().unwrap()),
                        ]),
                    ),
                    (
                        Resource::Geode,
                        HashMap::from_iter(vec![
                            (Resource::Ore, geode_ore.parse::<u32>().unwrap()),
                            (Resource::Obsidian, geode_obsidian.parse::<u32>().unwrap()),
                        ]),
                    ),
                ]),
            },
        )(input)
    }

    fn quality_level(&self, time_allowed: u32) -> u32 {
        self.id * self.calc_most_geodes_opened(time_allowed)
    }

    fn calc_most_geodes_opened(&self, time_allowed: u32) -> u32 {
        calc_most_geodes_opened(time_allowed, self)
    }
}

fn calc_most_geodes_opened(time_allowed: u32, blueprint: &Blueprint) -> u32 {
    let mut states_seen = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(State::new());

    let mut best_result = None;
    while !queue.is_empty() {
        let mut state = queue.pop_back().unwrap();

        // Ignore this branch if we've seen it before
        if states_seen.contains(&state) {
            continue;
        } else {
            states_seen.insert(state.clone());
        }

        // Have to spend resources to build a robot before we mine resources for this minute
        let robot_build_options = state.robot_build_options(blueprint);

        state.mine_resources();

        // Time is up
        if state.minute == time_allowed {
            let result = state.resources[Resource::Geode as usize];
            if best_result.is_none() || result > best_result.unwrap() {
                best_result = Some(result);
            }
            continue;
        }

        // Ignore this branch if we can't beat the best result so far
        let time_remaining = time_allowed + 1 - state.minute;
        let best_possible = state.resources[Resource::Geode as usize]
            + time_remaining * (state.robots[Resource::Geode as usize] + (time_remaining / 2));
        if best_possible < best_result.unwrap_or(0) {
            continue;
        }

        // Explore each option at this state (either do nothing, or build a robot we can afford)
        for robot_build_option in robot_build_options {
            let mut new_state = state.clone();
            new_state.minute += 1;
            if let Some(robot_build_option) = robot_build_option.as_ref() {
                new_state.robots[*robot_build_option as usize] += 1;
                for (resource, cost) in &blueprint.robot_costs[robot_build_option] {
                    new_state.resources[*resource as usize] -= cost;
                }
            }
            queue.push_back(new_state);
        }
    }

    best_result.unwrap()
}

fn parse_blueprints(contents: &str) -> Vec<Blueprint> {
    let (rest, blueprints) =
        terminated(separated_list1(newline, Blueprint::parse), opt(newline))(contents)
            .expect("Failed to parse blueprints");
    assert!(rest.is_empty());
    blueprints
}

fn calc_total_quality_level(time_allowed: u32, blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|b| b.quality_level(time_allowed))
        .sum::<u32>()
}

fn calc_most_geodes_product(time_allowed: u32, blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|b| b.calc_most_geodes_opened(time_allowed))
        .reduce(|a, b| a * b)
        .unwrap()
}

#[test]
fn test_parse_blueprint() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    let result = Blueprint::parse(input);
    assert!(result.is_ok());
    let (rest, blueprint) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        blueprint,
        Blueprint {
            id: 1,
            robot_costs: HashMap::from_iter(vec![
                (Resource::Ore, HashMap::from_iter(vec![(Resource::Ore, 4)])),
                (Resource::Clay, HashMap::from_iter(vec![(Resource::Ore, 2)])),
                (
                    Resource::Obsidian,
                    HashMap::from_iter(vec![(Resource::Ore, 3), (Resource::Clay, 14)]),
                ),
                (
                    Resource::Geode,
                    HashMap::from_iter(vec![(Resource::Ore, 2), (Resource::Obsidian, 7)]),
                ),
            ])
        }
    );
}

#[test]
fn test_parse_blueprints() {
    let contents = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let blueprints = parse_blueprints(contents);
    assert_eq!(blueprints.len(), 2);

    assert_eq!(
        blueprints[0],
        Blueprint {
            id: 1,
            robot_costs: HashMap::from_iter(vec![
                (Resource::Ore, HashMap::from_iter(vec![(Resource::Ore, 4)])),
                (Resource::Clay, HashMap::from_iter(vec![(Resource::Ore, 2)])),
                (
                    Resource::Obsidian,
                    HashMap::from_iter(vec![(Resource::Ore, 3), (Resource::Clay, 14)]),
                ),
                (
                    Resource::Geode,
                    HashMap::from_iter(vec![(Resource::Ore, 2), (Resource::Obsidian, 7)]),
                ),
            ])
        }
    );

    assert_eq!(
        blueprints[1],
        Blueprint {
            id: 2,
            robot_costs: HashMap::from_iter(vec![
                (Resource::Ore, HashMap::from_iter(vec![(Resource::Ore, 2)])),
                (Resource::Clay, HashMap::from_iter(vec![(Resource::Ore, 3)])),
                (
                    Resource::Obsidian,
                    HashMap::from_iter(vec![(Resource::Ore, 3), (Resource::Clay, 8)]),
                ),
                (
                    Resource::Geode,
                    HashMap::from_iter(vec![(Resource::Ore, 3), (Resource::Obsidian, 12)]),
                ),
            ])
        }
    );
}

#[test]
fn test_calc_most_geodes_opened_short() {
    let contents = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let blueprints = parse_blueprints(contents);
    assert_eq!(blueprints[0].calc_most_geodes_opened(24), 9);
    assert_eq!(blueprints[1].calc_most_geodes_opened(24), 12);
}

#[test]
fn test_quality_level() {
    let contents = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let blueprints = parse_blueprints(contents);
    assert_eq!(blueprints[0].calc_most_geodes_opened(24), 9);
    assert_eq!(blueprints[1].quality_level(24), 24);
}

#[test]
fn test_calc_total_quality_level() {
    let contents = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let blueprints = parse_blueprints(contents);
    assert_eq!(calc_total_quality_level(24, &blueprints), 33);
}

#[test]
fn test_calc_most_geodes_opened_long() {
    let contents = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let blueprints = parse_blueprints(contents);
    assert_eq!(blueprints[0].calc_most_geodes_opened(32), 56);
    assert_eq!(blueprints[1].calc_most_geodes_opened(32), 62);
}

#[test]
fn test_calc_most_geodes_product() {
    let contents = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let blueprints = parse_blueprints(contents);
    assert_eq!(calc_most_geodes_product(32, &blueprints), 3472);
}
