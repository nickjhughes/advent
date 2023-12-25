use hashbrown::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let graph = Graph::parse(&input);
    let component_sizes = graph.min_cut_component_sizes();
    assert_eq!(component_sizes.len(), 2);
    component_sizes.iter().product::<usize>().to_string()
}

pub fn part2() -> String {
    let _input = get_input_file_contents();
    "".into()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input25").expect("Failed to open input file")
}

#[derive(Debug, Clone)]
struct Graph<'input> {
    nodes: Vec<&'input str>,
    edges: Vec<(usize, usize)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Component<'input>(&'input str);

impl<'input> Graph<'input> {
    fn parse(input: &'input str) -> Self {
        let mut nodes = Vec::new();
        let mut node_indices = HashMap::new();
        for line in input.lines() {
            let (node, connected_nodes) = line.split_once(':').unwrap();
            if !nodes.contains(&node) {
                nodes.push(node);
                node_indices.insert(node, nodes.len() - 1);
            }
            for connected_node in connected_nodes.split_whitespace() {
                if !nodes.contains(&connected_node) {
                    nodes.push(connected_node);
                    node_indices.insert(connected_node, nodes.len() - 1);
                }
            }
        }

        let mut edges = Vec::new();
        for line in input.lines() {
            let (node, connected_nodes) = line.split_once(':').unwrap();
            let node_index = node_indices.get(&node).unwrap();
            for connected_node in connected_nodes.split_whitespace() {
                let connected_node_index = node_indices.get(&connected_node).unwrap();
                edges.push((*node_index, *connected_node_index));
                edges.push((*connected_node_index, *node_index));
            }
        }

        Graph { nodes, edges }
    }

    fn min_cut_component_sizes(&self) -> Vec<usize> {
        let mut rng = thread_rng();

        loop {
            let mut max_node = self.nodes.len();
            let mut nodes: HashSet<usize> = (0..self.nodes.len()).collect();
            let mut edges = self.edges.clone();
            let mut merged_nodes: HashMap<usize, (usize, usize)> = HashMap::new();
            while nodes.len() > 2 {
                let (n1, n2) = *edges.choose(&mut rng).unwrap();
                // Remove nodes and replace with merged node
                nodes.remove(&n1);
                nodes.remove(&n2);
                nodes.insert(max_node);
                merged_nodes.insert(max_node, (n1, n2));
                // Remove and replace edges
                edges.retain(|e| *e != (n1, n2) && *e != (n2, n1));
                for edge in edges.iter_mut() {
                    if edge.0 == n1 || edge.0 == n2 {
                        edge.0 = max_node;
                    } else if edge.1 == n1 || edge.1 == n2 {
                        edge.1 = max_node;
                    }
                }
                max_node += 1;
            }
            if edges.len() == 6 {
                let mut components = Vec::new();
                for component_node in nodes.iter() {
                    let mut component = HashSet::new();
                    let mut nodes_to_expand = vec![*component_node];
                    while let Some(node) = nodes_to_expand.pop() {
                        if node < self.nodes.len() {
                            component.insert(node);
                        } else {
                            let constituent_nodes = merged_nodes.get(&node).unwrap();
                            nodes_to_expand.push(constituent_nodes.0);
                            nodes_to_expand.push(constituent_nodes.1);
                        }
                    }
                    components.push(component);
                }
                return components.iter().map(|c| c.len()).collect();
            }
        }
    }
}

#[test]
fn test_parse_graph() {
    let input = "jqt: rhn xhk nvd\nrsh: frs pzl lsr\nxhk: hfx\ncmg: qnr nvd lhk bvb\nrhn: xhk bvb hfx\nbvb: xhk hfx\npzl: lsr hfx nvd\nqnr: nvd\nntq: jqt hfx bvb xhk\nnvd: lhk\nlsr: lhk\nrzs: qnr cmg lsr rsh\nfrs: qnr lhk lsr\n";
    let graph = Graph::parse(input);
    assert_eq!(graph.nodes.len(), 15);
}

#[test]
fn test_minimum_cut() {
    let input = "jqt: rhn xhk nvd\nrsh: frs pzl lsr\nxhk: hfx\ncmg: qnr nvd lhk bvb\nrhn: xhk bvb hfx\nbvb: xhk hfx\npzl: lsr hfx nvd\nqnr: nvd\nntq: jqt hfx bvb xhk\nnvd: lhk\nlsr: lhk\nrzs: qnr cmg lsr rsh\nfrs: qnr lhk lsr\n";
    let graph = Graph::parse(input);
    let mut component_sizes = graph.min_cut_component_sizes();
    component_sizes.sort();
    assert_eq!(component_sizes, vec![6, 9]);
}
