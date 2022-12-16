use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let volcano = Volcano::parse(&contents);
    let most_pressure_released = find_most_pressure_released(State::new(&volcano), &volcano);
    format!("{}", most_pressure_released)
}

pub fn part2() -> String {
    let _contents = get_input_file_contents();
    format!("")
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input16").expect("Failed to open input file")
}

#[derive(Debug)]
struct Volcano {
    initial_valve: usize,
    flow_rates: Vec<u32>,
    connections: Vec<(usize, usize)>,
    shortest_paths: HashMap<(usize, usize), u32>,
}

impl Volcano {
    fn parse(contents: &str) -> Self {
        let mut flow_rates = Vec::new();
        let mut valve_names = Vec::new();
        let mut name_connections = Vec::new();
        for line in contents.split('\n') {
            if line.is_empty() {
                continue;
            }

            let name = line[6..8].to_string();
            valve_names.push(name.clone());

            let flow_rate = line[23..line.find(';').expect("Failed to find flow rate")]
                .parse::<u32>()
                .expect("Failed to parse flow rate");
            flow_rates.push(flow_rate);

            let connections_index = if line.contains("tunnels lead to valves ") {
                line.find("tunnels lead to valves ").unwrap() + "tunnels lead to valves ".len()
            } else if line.contains("tunnel leads to valve ") {
                line.find("tunnel leads to valve ").unwrap() + "tunnel leads to valve ".len()
            } else {
                panic!("Failed to find tunnels")
            };
            let connected_names = line[connections_index..].split(", ");
            for connected_name in connected_names {
                name_connections.push((name.clone(), connected_name.to_string()));
            }
        }

        let mut connections = Vec::new();
        for (valve1, valve2) in &name_connections {
            let index1 = valve_names
                .iter()
                .position(|v| v == valve1)
                .expect("Failed to find connected valve");
            let index2 = valve_names
                .iter()
                .position(|v| v == valve2)
                .expect("Failed to find connected valve");
            connections.push((index1, index2));
        }

        let initial_valve = valve_names
            .iter()
            .position(|v| *v == "AA")
            .expect("Failed to find initial valve");

        let mut volcano = Volcano {
            initial_valve,
            flow_rates,
            connections,
            shortest_paths: HashMap::new(),
        };

        for valve1 in 0..volcano.flow_rates.len() {
            for valve2 in 0..volcano.flow_rates.len() {
                volcano
                    .shortest_paths
                    .insert((valve1, valve2), volcano.calc_shortest_path(valve1, valve2));
            }
        }
        volcano
    }

    fn neighbors(&self, valve: usize) -> Vec<usize> {
        self.connections
            .iter()
            .filter(|(valve1, _)| valve1 == &valve)
            .map(|(_, valve2)| *valve2)
            .collect::<Vec<usize>>()
    }

    fn shortest_path(&self, start: usize, end: usize) -> u32 {
        self.shortest_paths[&(start, end)]
    }

    fn calc_shortest_path(&self, start: usize, end: usize) -> u32 {
        let mut queue = Vec::new();
        let mut distance = Vec::new();
        for valve in 0..self.flow_rates.len() {
            distance.push(std::u32::MAX);
            queue.push(valve);
        }
        distance[start] = 0;

        while !queue.is_empty() {
            let mut min_dist_index = 0;
            let mut min_dist = std::u32::MAX;
            for (index, valve) in queue.iter().enumerate() {
                if distance[*valve] < min_dist {
                    min_dist = distance[*valve];
                    min_dist_index = index;
                }
            }
            if min_dist == std::u32::MAX {
                // No more valves are reachable from the start valve
                break;
            }
            let valve = queue.remove(min_dist_index);

            let neighbors = self.neighbors(valve);
            for neighbor in neighbors {
                let new_distance = distance[valve] + 1;
                if new_distance < distance[neighbor] {
                    distance[neighbor] = new_distance;
                }
            }
        }

        distance[end]
    }
}

#[derive(Debug, Clone)]
struct State {
    time_remaining: u32,
    current_valve: usize,
    open_valves: HashSet<usize>,
    pressure_released: u32,
}

impl State {
    fn new(volcano: &Volcano) -> Self {
        Self {
            time_remaining: 30,
            current_valve: volcano.initial_valve,
            open_valves: HashSet::new(),
            pressure_released: 0,
        }
    }
}

fn find_most_pressure_released(state: State, volcano: &Volcano) -> u32 {
    let mut possible_pressure_releases = Vec::new();
    for (valve, flow_rate) in volcano.flow_rates.iter().enumerate() {
        if *flow_rate == 0 {
            possible_pressure_releases.push(0);
            continue;
        }
        if state.open_valves.contains(&valve) {
            possible_pressure_releases.push(0);
            continue;
        }
        let shortest_path = volcano.shortest_path(state.current_valve, valve);
        if shortest_path >= state.time_remaining {
            possible_pressure_releases.push(0);
            continue;
        }
        let possible_pressure_release = (state.time_remaining - shortest_path - 1) * flow_rate;
        possible_pressure_releases.push(possible_pressure_release);
    }

    let mut best_result = None;
    for (possible_valve, pressure_release) in possible_pressure_releases.iter().enumerate() {
        if *pressure_release == 0 {
            continue;
        }

        let mut new_state = state.clone();
        new_state.time_remaining -=
            volcano.shortest_path(new_state.current_valve, possible_valve) + 1;
        new_state.current_valve = possible_valve;
        new_state.pressure_released += pressure_release;
        new_state.open_valves.insert(possible_valve);

        let result = find_most_pressure_released(new_state, volcano);
        if best_result.is_none() || result > best_result.unwrap() {
            best_result = Some(result);
        }
    }
    best_result.unwrap_or(state.pressure_released)
}

#[test]
fn test_parse_volcano() {
    let contents = "Valve AA has flow rate=0; tunnels lead to valves DD, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnel leads to valve CC\n";
    let volcano = Volcano::parse(contents);

    assert_eq!(volcano.initial_valve, 0);

    assert_eq!(volcano.flow_rates.len(), 4);
    assert_eq!(volcano.flow_rates, vec![0, 13, 2, 20]);

    assert_eq!(volcano.connections.len(), 7);
    assert_eq!(
        volcano.connections,
        vec![(0, 3), (0, 1), (1, 2), (1, 0), (2, 3), (2, 1), (3, 2),]
    );
}

#[test]
fn test_neighbors() {
    let contents = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II\n";
    let volcano = Volcano::parse(contents);

    {
        let neighbors = volcano.neighbors(0);
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&3));
        assert!(neighbors.contains(&8));
        assert!(neighbors.contains(&1));
    }

    {
        let neighbors = volcano.neighbors(1);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&0));
        assert!(neighbors.contains(&2));
    }
}

#[test]
fn test_calc_shortest_path() {
    let contents = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II\n";
    let volcano = Volcano::parse(contents);

    {
        let path_length = volcano.calc_shortest_path(0, 1);
        assert_eq!(path_length, 1);
    }

    {
        let path_length = volcano.calc_shortest_path(0, 7);
        assert_eq!(path_length, 5);
    }
}

#[test]
fn test_find_most_pressure_released() {
    let contents = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II\n";
    let volcano = Volcano::parse(contents);
    let most_pressure_released = find_most_pressure_released(State::new(&volcano), &volcano);
    assert_eq!(most_pressure_released, 1651);
}
