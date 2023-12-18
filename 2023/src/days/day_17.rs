use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    map.minimal_heat_loss().to_string()
}

pub fn part2() -> String {
    let _input = get_input_file_contents();
    "".into()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input17").expect("Failed to open input file")
}

#[derive(Debug)]
struct Map {
    blocks: Vec<u8>,
    width: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut blocks = Vec::new();
        let mut width = None;
        for line in input.lines() {
            if width.is_none() {
                width = Some(line.len());
            }
            blocks.extend(line.chars().map(|ch| ch as u8 - b'0'));
        }
        Map {
            blocks,
            width: width.unwrap(),
        }
    }

    fn height(&self) -> usize {
        self.blocks.len() / self.width
    }

    fn minimal_heat_loss(&self) -> u32 {
        let mut open_set: HashSet<usize> = HashSet::new();
        open_set.insert(0);

        let mut came_from: HashMap<usize, usize> = HashMap::new();

        let mut g_score: HashMap<usize, u32> = HashMap::new();
        g_score.insert(0, 0);

        let mut f_score: HashMap<usize, u32> = HashMap::new();
        f_score.insert(0, self.heuristic(0));

        while !open_set.is_empty() {
            let mut min_node = None;
            let mut min_f_score = u32::MAX;
            for node in open_set.iter() {
                if let Some(f) = f_score.get(node) {
                    if *f < min_f_score {
                        min_node = Some(*node);
                        min_f_score = *f;
                    }
                }
            }
            let current = open_set.take(&min_node.unwrap()).unwrap();
            let current_row = current / self.width;
            let current_col = current % self.width;

            if current_col == self.width - 1 && current_row == self.height() - 1 {
                // Goal reached, reconstruct path
                let mut path = Vec::new();
                path.push(current);
                let mut node = current;
                let mut heat_loss = self.blocks[node] as u32;
                while let Some(prev) = came_from.get(&node) {
                    path.push(*prev);
                    heat_loss += self.blocks[*prev] as u32;
                    node = *prev;
                }

                // TODO: Print grid + path
                for row in 0..self.height() {
                    for col in 0..self.width {
                        if path.contains(&(row * &self.width + col)) {
                            print!("#");
                        } else {
                            print!("{}", self.blocks[row * self.width + col]);
                        }
                    }
                    println!();
                }
                println!();

                return heat_loss;
            }

            // Keep track of the last three directions
            let mut all_left = false;
            let mut all_right = false;
            let mut all_up = false;
            let mut all_down = false;
            if let Some(prev) = came_from.get(&current) {
                if let Some(prev2) = came_from.get(prev) {
                    if let Some(prev3) = came_from.get(prev2) {
                        all_right = *prev == current.saturating_sub(1)
                            && *prev2 == prev.saturating_sub(1)
                            && *prev3 == prev2.saturating_sub(1);
                        all_left =
                            *prev == current + 1 && *prev2 == prev + 1 && *prev3 == prev2 + 1;
                        all_down = *prev == current.saturating_sub(self.width)
                            && *prev2 == prev.saturating_sub(self.width)
                            && *prev3 == prev2.saturating_sub(self.width);
                        all_up = *prev == current + self.width
                            && *prev2 == prev - self.width
                            && *prev3 == prev2 - self.width;
                    }
                }
            }

            // Go through neighbors
            let mut neighbors = Vec::new();
            // Up
            if !all_up
                && current_row > 0
                && came_from.get(&current) != Some(&((current_row - 1) * self.width + current_col))
            {
                neighbors.push((current_row - 1) * self.width + current_col);
            }
            // Down
            if !all_down
                && current_row < self.height() - 1
                && came_from.get(&current) != Some(&((current_row + 1) * self.width + current_col))
            {
                neighbors.push((current_row + 1) * self.width + current_col);
            }
            // Left
            if !all_left
                && current_col > 0
                && came_from.get(&current) != Some(&(current_row * self.width + current_col - 1))
            {
                neighbors.push(current_row * self.width + current_col - 1);
            }
            // Right
            if !all_right
                && current_col < self.height() - 1
                && came_from.get(&current) != Some(&(current_row * self.width + current_col + 1))
            {
                neighbors.push(current_row * self.width + current_col + 1);
            }
            for neighbor in neighbors {
                let tentative_g_score =
                    g_score.get(&current).unwrap() + self.blocks[neighbor] as u32;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + self.heuristic(neighbor));
                    if !open_set.contains(&neighbor) {
                        open_set.insert(neighbor);
                    }
                }
            }
        }

        panic!("no path found")
    }

    fn heuristic(&self, index: usize) -> u32 {
        0
        // let row = (index / self.width) as u32;
        // let col = (index % self.width) as u32;
        // let goal_row = (self.height() - 1) as u32;
        // let goal_col = (self.width - 1) as u32;
        // (goal_col - col) + (goal_row - row)
    }
}

#[test]
fn test_parse() {
    let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533\n";
    let map = Map::parse(input);
    assert_eq!(map.width, 13);
}

#[test]
fn test_minimal_heat_loss() {
    let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533\n";
    let map = Map::parse(input);
    assert_eq!(map.minimal_heat_loss(), 102);
}
