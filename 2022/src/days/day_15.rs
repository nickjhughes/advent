use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use std::{collections::HashSet, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let sensors = parse_sensors(&contents);
    let result = row_coverage(2000000, &sensors);
    format!("{}", result)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let sensors = parse_sensors(&contents);
    let beacon = beacon_location(4000000, &sensors);
    let tuning_frequency = calc_tuning_frequency(&beacon);
    format!("{}", tuning_frequency)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input15").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map(tuple((opt(tag("-")), digit1)), |(neg, digits)| {
        let mut full_string = String::new();
        if neg.is_some() {
            full_string.push('-');
        }
        full_string.push_str(digits);
        full_string
            .parse::<i32>()
            .expect("Failed to parse coordinate")
    })(input)
}

impl Point {
    #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((tag("x="), parse_i32, tag(", y="), parse_i32)),
            |(_, x, _, y)| Self { x, y },
        )(input)
    }

    fn manhatten_dist(&self, other: &Point) -> u32 {
        (other.x - self.x).unsigned_abs() + (other.y - self.y).unsigned_abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    position: Point,
    nearest_beacon: Point,
}

impl Sensor {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("Sensor at "),
                Point::parse,
                tag(": closest beacon is at "),
                Point::parse,
            )),
            |(_, position, _, nearest_beacon)| Self {
                position,
                nearest_beacon,
            },
        )(input)
    }

    fn distance_to_beacon(&self) -> u32 {
        self.position.manhatten_dist(&self.nearest_beacon)
    }

    fn row_coverage(&self, y: i32) -> Option<(i32, i32)> {
        // The range of x coordinates (inclusive) of the given row that this beacon can see
        let distance_to_beacon = self.distance_to_beacon() as i32;
        let distance_to_row = (y - self.position.y).abs();
        let range = (
            self.position.x - distance_to_beacon + distance_to_row,
            self.position.x + distance_to_beacon - distance_to_row,
        );
        if range.1 >= range.0 {
            Some(range)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RangePoint {
    Start(i32),
    End(i32),
}

fn parse_sensors(contents: &str) -> Vec<Sensor> {
    let (rest, sensors) =
        terminated(separated_list1(newline, Sensor::parse), opt(newline))(contents)
            .expect("Failed to parse sensors");
    assert!(rest.is_empty());
    sensors
}

fn combine_ranges(ranges: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    for (start, end) in ranges {
        points.push(RangePoint::Start(*start));
        points.push(RangePoint::End(*end));
    }
    points.sort_by_key(|p| match p {
        RangePoint::Start(val) => (*val, 0),
        RangePoint::End(val) => (*val, 1),
    });

    let mut disjoint_ranges: Vec<(i32, i32)> = Vec::new();
    let mut current_start = None;
    let mut range_count = 0;
    for point in points {
        match point {
            RangePoint::Start(start) => {
                range_count += 1;
                if let Some(last_range) = disjoint_ranges.last() {
                    if last_range.1 == start - 1 {
                        // End of previous range is start of next range, so combine them
                        current_start = Some(last_range.0);
                        disjoint_ranges.pop();
                    } else if current_start.is_none() {
                        current_start = Some(start);
                    }
                } else if current_start.is_none() {
                    current_start = Some(start);
                }
            }
            RangePoint::End(end) => {
                range_count -= 1;
                if range_count == 0 {
                    disjoint_ranges.push((current_start.unwrap(), end));
                    current_start = None;
                }
            }
        }
    }
    disjoint_ranges
}

fn measure_ranges(ranges: &[(i32, i32)]) -> u32 {
    let disjoint_ranges = combine_ranges(ranges);
    disjoint_ranges
        .iter()
        .map(|(start, end)| (end - start + 1) as u32)
        .sum::<u32>()
}

fn row_coverage_ranges(y: i32, sensors: &[Sensor]) -> Vec<(i32, i32)> {
    sensors
        .iter()
        .filter_map(|s| s.row_coverage(y))
        .collect::<Vec<(i32, i32)>>()
}

fn row_coverage(y: i32, sensors: &[Sensor]) -> u32 {
    let coverage_ranges = row_coverage_ranges(y, sensors);
    let ranges_size = measure_ranges(&coverage_ranges);
    let beacon_locations = sensors
        .iter()
        .filter(|s| s.nearest_beacon.y == y)
        .map(|s| s.nearest_beacon.y)
        .collect::<HashSet<i32>>();
    ranges_size - beacon_locations.len() as u32
}

fn range_overlap(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    let start = a.0.max(b.0);
    let end = a.1.min(b.1);
    if start <= end {
        Some((start, end))
    } else {
        None
    }
}

fn beacon_location(max_coords: i32, sensors: &[Sensor]) -> Point {
    for y in 0..max_coords {
        let ranges = row_coverage_ranges(y, sensors);
        let disjoint_ranges = combine_ranges(&ranges);
        let mut restricted_ranges = Vec::new();
        for range in &disjoint_ranges {
            let restricted_range = range_overlap((0, max_coords), *range);
            if let Some(restricted_range) = restricted_range {
                restricted_ranges.push(restricted_range);
            }
        }
        if restricted_ranges.len() == 1 && restricted_ranges[0] == (0, max_coords) {
            continue;
        }
        if !restricted_ranges.is_empty() {
            assert_eq!(restricted_ranges.len(), 2);
            return Point::new(restricted_ranges[0].1 + 1, y);
        }
    }
    panic!("Failed to locate beacon");
}

fn calc_tuning_frequency(beacon: &Point) -> u64 {
    beacon.x as u64 * 4000000 + beacon.y as u64
}

#[test]
fn test_parse_i32() {
    {
        let result = parse_i32("1234");
        assert!(result.is_ok());
        let (rest, num) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(num, 1234);
    }

    {
        let result = parse_i32("-5678");
        assert!(result.is_ok());
        let (rest, num) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(num, -5678);
    }
}

#[test]
fn test_parse_point() {
    let result = Point::parse("x=-2, y=15");
    assert!(result.is_ok());
    let (rest, point) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(point, Point::new(-2, 15));
}

#[test]
fn test_parse_sensor() {
    let result = Sensor::parse("Sensor at x=17, y=20: closest beacon is at x=21, y=22");
    assert!(result.is_ok());
    let (rest, sensor) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        sensor,
        Sensor {
            position: Point::new(17, 20),
            nearest_beacon: Point::new(21, 22)
        }
    );
}

#[test]
fn test_parse_sensors() {
    let contents = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3";
    let sensors = parse_sensors(contents);
    assert_eq!(sensors.len(), 3);

    assert_eq!(
        sensors[0],
        Sensor {
            position: Point::new(2, 18),
            nearest_beacon: Point::new(-2, 15)
        }
    );
    assert_eq!(
        sensors[1],
        Sensor {
            position: Point::new(9, 16),
            nearest_beacon: Point::new(10, 16)
        }
    );
    assert_eq!(
        sensors[2],
        Sensor {
            position: Point::new(13, 2),
            nearest_beacon: Point::new(15, 3)
        }
    );
}

#[test]
fn test_sensor_row_coverage() {
    let contents = "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n";
    let sensors = parse_sensors(contents);
    let sensor = &sensors[0];
    let coverage = sensor.row_coverage(10);
    assert!(coverage.is_some());
    assert_eq!(coverage.unwrap(), (2, 14));
}

#[test]
fn test_row_coverage() {
    let contents = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3\n";
    let sensors = parse_sensors(contents);
    let coverage = row_coverage(10, &sensors);
    assert_eq!(coverage, 26);
}

#[test]
fn test_measure_ranges() {
    let ranges = vec![(-2, 0), (-1, 3), (5, 7)];
    let result = measure_ranges(&ranges);
    assert_eq!(result, 9);
}

#[test]
fn test_range_overlap() {
    assert_eq!(range_overlap((-2, 24), (0, 20)), Some((0, 20)));
    assert_eq!(range_overlap((-2, 10), (11, 20)), None);
    assert_eq!(range_overlap((-2, 10), (0, 20)), Some((0, 10)));
}

#[test]
fn test_calc_tuning_frequency() {
    let beacon = Point::new(14, 11);
    let tuning_frequency = calc_tuning_frequency(&beacon);
    assert_eq!(tuning_frequency, 56000011);
}

#[test]
fn test_beacon_location() {
    let contents = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3\n";
    let sensors = parse_sensors(contents);
    let beacon = beacon_location(20, &sensors);
    assert_eq!(beacon, Point::new(14, 11));
}
