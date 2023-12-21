use hashbrown::HashSet;
use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let garden = Garden::parse(&input);
    garden.garden_plots_reachable_after_n_steps(64).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let garden = Garden::parse(&input);
    garden
        .garden_plots_reachable_after_n_steps_fast(26501365)
        .to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input21").expect("Failed to open input file")
}

#[derive(Debug)]
struct Garden {
    tiles: Vec<Tile>,
    width: usize,
    start: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    GardenPlot,
    Rock,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let mut width = None;
        let mut start = None;
        let mut tiles = Vec::new();
        for (row, line) in input.lines().enumerate() {
            if width.is_none() {
                width = Some(line.len());
            }
            for (col, ch) in line.chars().enumerate() {
                let tile = match ch {
                    '.' => Tile::GardenPlot,
                    '#' => Tile::Rock,
                    'S' => {
                        start = Some(row * width.unwrap() + col);
                        Tile::GardenPlot
                    }
                    _ => panic!("invalid tile {ch}"),
                };
                tiles.push(tile);
            }
        }

        Garden {
            tiles,
            width: width.unwrap(),
            start: start.unwrap(),
        }
    }

    fn height(&self) -> usize {
        assert_eq!(self.tiles.len() / self.width, self.width);
        self.width
    }

    fn garden_plots_reachable_after_n_steps(&self, n: usize) -> usize {
        let mut plots: HashSet<usize> = HashSet::new();
        plots.insert(self.start);
        for _ in 0..n {
            // For each plot, add any of its 4 neighbors which are valid and remove itself
            let mut new_plots: HashSet<usize> = HashSet::new();
            for plot in plots.iter() {
                let row = plot / self.width;
                let col = plot % self.width;
                // North
                if row > 0 && self.tiles[(row - 1) * self.width + col] != Tile::Rock {
                    new_plots.insert((row - 1) * self.width + col);
                }
                // South
                if row < self.height() - 1 && self.tiles[(row + 1) * self.width + col] != Tile::Rock
                {
                    new_plots.insert((row + 1) * self.width + col);
                }
                // East
                if col < self.width - 1 && self.tiles[row * self.width + col + 1] != Tile::Rock {
                    new_plots.insert(row * self.width + col + 1);
                }
                // West
                if col > 0 && self.tiles[row * self.width + col - 1] != Tile::Rock {
                    new_plots.insert(row * self.width + col - 1);
                }
            }
            plots = new_plots;
        }
        plots.len()
    }

    fn garden_plots_reachable_after_n_steps_fast(&self, steps: usize) -> u64 {
        assert!(steps % 2 == 1);
        let n = steps / self.width;
        let remainder = steps % self.width;

        let garden_counts = self.expand_and_move_n_steps(2, 2 * self.width + remainder);
        let even_full = garden_counts[7];
        let odd_full = garden_counts[12];
        let top = garden_counts[2];
        let left = garden_counts[10];
        let right = garden_counts[14];
        let bottom = garden_counts[22];
        let top_left = garden_counts[1];
        let top_right = garden_counts[3];
        let bottom_left = garden_counts[15];
        let bottom_right = garden_counts[19];
        let partial_top_left = garden_counts[6];
        let partial_top_right = garden_counts[8];
        let partial_bottom_left = garden_counts[16];
        let partial_bottom_right = garden_counts[18];

        let full_gardens_count = 2 * (n - 1) * n + 1;
        let odd_full_garden_count = ((n - 1) * n) / 2 + ((n - 2) * (n - 1)) / 2;
        let even_full_garden_count = full_gardens_count - odd_full_garden_count;

        let full = even_full_garden_count * even_full + odd_full_garden_count * odd_full;
        let partials = top
            + left
            + right
            + bottom
            + n * (top_left + top_right + bottom_left + bottom_right)
            + (n - 1)
                * (partial_top_left
                    + partial_top_right
                    + partial_bottom_left
                    + partial_bottom_right);
        (full + partials) as u64
    }

    /// Return the number of reachable plots in each duplicate of the original garden.
    fn expand_and_move_n_steps(&self, expansion: usize, steps: usize) -> Vec<usize> {
        // Expand `expansion` times in each direction
        let expansion_size = 2 * expansion + 1;
        let mut expanded_tiles =
            vec![Tile::GardenPlot; expansion_size * expansion_size * self.tiles.len()];
        let full_width = expansion_size * self.width;
        let full_height = expansion_size * self.height();
        for garden_row in 0..expansion_size {
            for garden_col in 0..expansion_size {
                for row in 0..self.width {
                    for col in 0..self.height() {
                        let i = (garden_row * self.height() + row) * full_width
                            + garden_col * self.width
                            + col;
                        expanded_tiles[i] = self.tiles[row * self.width + col];
                    }
                }
            }
        }

        let mut plots: HashSet<usize> = HashSet::new();
        plots.insert(
            (expansion * self.height() + self.start / self.width) * full_width
                + expansion * self.width
                + self.start % self.width,
        );
        for _ in 0..steps {
            // For each plot, add any of its 4 neighbors which are valid and remove itself
            let mut new_plots: HashSet<usize> = HashSet::new();
            for plot in plots.iter() {
                let row = plot / full_width;
                let col = plot % full_width;
                // North
                if row > 0 && expanded_tiles[(row - 1) * full_width + col] != Tile::Rock {
                    new_plots.insert((row - 1) * full_width + col);
                }
                // South
                if row < full_height - 1
                    && expanded_tiles[(row + 1) * full_width + col] != Tile::Rock
                {
                    new_plots.insert((row + 1) * full_width + col);
                }
                // East
                if col < full_width - 1 && expanded_tiles[row * full_width + col + 1] != Tile::Rock
                {
                    new_plots.insert(row * full_width + col + 1);
                }
                // West
                if col > 0 && expanded_tiles[row * full_width + col - 1] != Tile::Rock {
                    new_plots.insert(row * full_width + col - 1);
                }
            }
            plots = new_plots;
        }

        // Count locations in each garden
        let mut garden_counts = Vec::new();
        for garden_row in 0..expansion_size {
            for garden_col in 0..expansion_size {
                let mut garden_plot_count = 0;
                for row in 0..self.width {
                    for col in 0..self.height() {
                        let i = (garden_row * self.height() + row) * full_width
                            + garden_col * self.width
                            + col;
                        if plots.contains(&i) {
                            garden_plot_count += 1;
                        }
                    }
                }
                garden_counts.push(garden_plot_count);
            }
        }
        garden_counts
    }
}

#[test]
fn test_parse_garden() {
    let input = "...........\n.....###.#.\n.###.##..#.\n..#.#...#..\n....#.#....\n.##..S####.\n.##..#...#.\n.......##..\n.##.#.####.\n.##..##.##.\n...........\n";
    let garden = Garden::parse(input);
    assert_eq!(garden.width, 11);
    assert_eq!(garden.start, 60);
}

#[test]
fn test_garden_plots_reachable_after_n_steps() {
    let input = "...........\n.....###.#.\n.###.##..#.\n..#.#...#..\n....#.#....\n.##..S####.\n.##..#...#.\n.......##..\n.##.#.####.\n.##..##.##.\n...........\n";
    let garden = Garden::parse(input);

    assert_eq!(garden.garden_plots_reachable_after_n_steps(1), 2);
    assert_eq!(garden.garden_plots_reachable_after_n_steps(2), 4);
    assert_eq!(garden.garden_plots_reachable_after_n_steps(3), 6);
    assert_eq!(garden.garden_plots_reachable_after_n_steps(6), 16);
}
