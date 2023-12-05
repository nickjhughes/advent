use std::{collections::HashMap, fs, ops::RangeInclusive};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let almanac = Almanac::parse(&input);
    almanac.min_initial_seed_location().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let almanac = Almanac::parse(&input);
    almanac.min_initial_seed_location_ranges().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input05").expect("Failed to open input file")
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Category {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => anyhow::bail!("invalid category {:?}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
struct MapRange {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl MapRange {
    fn src_range(&self) -> RangeInclusive<u64> {
        self.src_start..=self.src_start + (self.len - 1)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<(Category, Category), Vec<MapRange>>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let seeds = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut maps: HashMap<(Category, Category), Vec<MapRange>> = HashMap::new();
        let mut current_map_key = None;
        for line in lines.skip(1) {
            if line.is_empty() {
                current_map_key = None;
            } else if let Some(current_map_key) = current_map_key.as_ref() {
                let map = maps.get_mut(current_map_key).unwrap();
                let mut parts = line.split_whitespace();
                map.push(MapRange {
                    dst_start: parts.next().unwrap().parse::<u64>().unwrap(),
                    src_start: parts.next().unwrap().parse::<u64>().unwrap(),
                    len: parts.next().unwrap().parse::<u64>().unwrap(),
                });
            } else {
                let mut parts = line.split_whitespace().next().unwrap().split('-');
                let src_category = Category::try_from(parts.next().unwrap()).unwrap();
                parts.next();
                let dst_category = Category::try_from(parts.next().unwrap()).unwrap();
                current_map_key = Some((src_category, dst_category));
                maps.insert((src_category, dst_category), Vec::new());
            }
        }

        Almanac { seeds, maps }
    }

    fn seed_location(&self, seed: u64) -> u64 {
        let mut current_category = Category::Seed;
        let mut current_value = seed;
        while current_category != Category::Location {
            let new_category = {
                let mut new_category = None;
                for key in self.maps.keys() {
                    if key.0 == current_category {
                        new_category = Some(key.1);
                        break;
                    }
                }
                new_category.unwrap()
            };
            let map = self.maps.get(&(current_category, new_category)).unwrap();

            for range in map.iter() {
                if (range.src_start..range.src_start + range.len).contains(&current_value) {
                    let offset = current_value - range.src_start;
                    current_value = range.dst_start + offset;
                    break;
                }
            }

            current_category = new_category;
        }
        current_value
    }

    fn min_initial_seed_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.seed_location(*seed))
            .min()
            .unwrap()
    }

    fn min_initial_seed_location_ranges(&self) -> u64 {
        let mut locations = Vec::with_capacity(self.seeds.len() / 2);
        for seed_range in self.seeds.chunks_exact(2) {
            let seed_range = seed_range[0]..=seed_range[0] + (seed_range[1] - 1);
            let mut current_ranges = vec![seed_range.clone()];
            let mut current_category = Category::Seed;
            while current_category != Category::Location {
                let new_category = {
                    let mut new_category = None;
                    for key in self.maps.keys() {
                        if key.0 == current_category {
                            new_category = Some(key.1);
                            break;
                        }
                    }
                    new_category.unwrap()
                };
                let map = self.maps.get(&(current_category, new_category)).unwrap();

                let mut new_ranges = Vec::new();
                while let Some(current_range) = current_ranges.pop() {
                    if *current_range.end() == u64::MAX {
                        panic!()
                    }

                    let mut found_mapping = false;
                    for map_range in map {
                        let src_range = map_range.src_range();
                        if src_range.start() <= current_range.end()
                            && src_range.end() >= current_range.start()
                        {
                            let src_overlap_start = src_range.start().max(current_range.start());
                            let src_overlap_end = src_range.end().min(current_range.end());
                            let dst_start =
                                map_range.dst_start + src_overlap_start - src_range.start();
                            let dst_end = map_range.dst_start + src_overlap_end - src_range.start();
                            let dst_range = dst_start..=dst_end;

                            new_ranges.push(dst_range);

                            if src_overlap_start > current_range.start() {
                                let pre_range =
                                    *current_range.start()..=(src_overlap_start.saturating_sub(1));
                                if !pre_range.is_empty() {
                                    current_ranges.push(pre_range);
                                }
                            }

                            if current_range.end() > src_overlap_end {
                                let post_range =
                                    (src_overlap_end.saturating_add(1))..=*current_range.end();
                                if !post_range.is_empty() {
                                    current_ranges.push(post_range);
                                }
                            }

                            found_mapping = true;
                            break;
                        }
                    }

                    if !found_mapping {
                        // We didn't find a mapped range, so we just pass it through directly
                        new_ranges.push(current_range);
                    }
                }
                current_ranges = new_ranges;
                current_category = new_category;
            }
            let min_location = current_ranges
                .iter()
                .map(|r| *r.start())
                .min()
                .unwrap_or(u64::MAX);
            locations.push(min_location);
        }
        *locations.iter().min().unwrap()
    }
}

#[test]
fn test_parse_almanac() {
    let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n";
    let almanac = Almanac::parse(input);
    assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
    assert_eq!(
        almanac.maps.get(&(Category::Seed, Category::Soil)),
        Some(&vec![
            MapRange {
                dst_start: 50,
                src_start: 98,
                len: 2,
            },
            MapRange {
                dst_start: 52,
                src_start: 50,
                len: 48,
            }
        ])
    );
}

#[test]
fn test_seed_location() {
    let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n";
    let almanac = Almanac::parse(input);
    assert_eq!(almanac.seed_location(79), 82);
    assert_eq!(almanac.seed_location(14), 43);
    assert_eq!(almanac.seed_location(55), 86);
    assert_eq!(almanac.seed_location(13), 35);
}

#[test]
fn test_min_initial_seed_location() {
    let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n";
    let almanac = Almanac::parse(input);
    assert_eq!(almanac.min_initial_seed_location(), 35);
}

#[test]
fn test_min_initial_seed_location_ranges() {
    let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n";
    let almanac = Almanac::parse(input);
    assert_eq!(almanac.min_initial_seed_location_ranges(), 46);
}
