use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{eof, map, opt},
    multi::{many_till, separated_list0, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};
use std::{cmp::Ordering, fmt, fs, iter::zip};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let packet_pairs = parse_packet_pairs(&contents);
    let index_sum = ordered_index_sum(&packet_pairs);
    format!("{}", index_sum)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let mut packets = parse_all_packets(&contents);
    packets.extend(generate_divider_packets());
    packets.sort();
    let decoder_key = calc_decoder_key(&packets);
    format!("{}", decoder_key)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input13").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(u8),
}

impl Packet {
    fn parse(input: &str) -> IResult<&str, Self> {
        terminated(Self::parse_list_variant, newline)(input)
    }

    fn parse_list_variant(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                char('['),
                separated_list0(
                    tag(","),
                    alt((Self::parse_integer_variant, Self::parse_list_variant)),
                ),
                char(']'),
            ),
            Self::List,
        )(input)
    }

    fn parse_integer_variant(input: &str) -> IResult<&str, Self> {
        map(digit1, |value: &str| {
            Self::Integer(value.parse::<u8>().expect("Failed to parse integer"))
        })(input)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match self {
            Packet::Integer(left_int) => match other {
                Packet::Integer(right_int) => left_int.cmp(right_int),
                Packet::List(_) => {
                    let list_self = Packet::List(vec![self.clone()]);
                    list_self.cmp(other)
                }
            },
            Packet::List(left_values) => match other {
                Packet::Integer(_) => {
                    let list_other = Packet::List(vec![other.clone()]);
                    self.cmp(&list_other)
                }
                Packet::List(right_values) => {
                    for (left_value, right_value) in zip(left_values, right_values) {
                        let order = left_value.cmp(right_value);
                        if order != Ordering::Equal {
                            return order;
                        }
                    }
                    left_values.len().cmp(&right_values.len())
                }
            },
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::List(values) => {
                let mut values_str = String::new();
                for (i, v) in values.iter().enumerate() {
                    values_str.push_str(&format!("{}", v));
                    if i != values.len() - 1 {
                        values_str.push(',');
                    }
                }
                write!(f, "[{}]", values_str)
            }
            Packet::Integer(int) => write!(f, "{}", int),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(pair(Packet::parse, Packet::parse), |(left, right)| Self {
            left,
            right,
        })(input)
    }

    fn is_ordered(&self) -> bool {
        match self.left.cmp(&self.right) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => panic!("Packets are equal"),
        }
    }
}

fn parse_packet_pairs(contents: &str) -> Vec<PacketPair> {
    let (rest, packet_pairs) = separated_list1(newline, PacketPair::parse)(contents)
        .expect("Failed to parse packet pairs");
    assert!(rest.is_empty());
    packet_pairs
}

fn ordered_index_sum(pairs: &[PacketPair]) -> usize {
    let mut index_sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.is_ordered() {
            index_sum += i + 1;
        }
    }
    index_sum
}

fn parse_all_packets(contents: &str) -> Vec<Packet> {
    let (rest, packets) = map(
        many_till(
            map(tuple((Packet::parse, opt(newline))), |(packet, _)| packet),
            eof,
        ),
        |(packets, _)| packets,
    )(contents)
    .expect("Failed to parse all packets");
    assert!(rest.is_empty());
    packets
}

fn generate_divider_packets() -> Vec<Packet> {
    vec![
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]),
    ]
}

fn calc_decoder_key(packets: &[Packet]) -> usize {
    let divider_packets = generate_divider_packets();
    let index1 = packets.binary_search(&divider_packets[0]).unwrap() + 1;
    let index2 = packets.binary_search(&divider_packets[1]).unwrap() + 1;
    index1 * index2
}

#[test]
fn test_parse_all_packets() {
    let contents = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n";
    let packets = parse_all_packets(contents);
    assert_eq!(packets.len(), 4);

    assert_eq!(
        packets[0],
        Packet::List(vec![
            Packet::Integer(1),
            Packet::Integer(1),
            Packet::Integer(3),
            Packet::Integer(1),
            Packet::Integer(1)
        ])
    );

    assert_eq!(
        packets[1],
        Packet::List(vec![
            Packet::Integer(1),
            Packet::Integer(1),
            Packet::Integer(5),
            Packet::Integer(1),
            Packet::Integer(1)
        ],),
    );

    assert_eq!(
        packets[2],
        Packet::List(vec![
            Packet::List(vec![Packet::Integer(1)]),
            Packet::List(vec![
                Packet::Integer(2),
                Packet::Integer(3),
                Packet::Integer(4),
            ]),
        ]),
    );

    assert_eq!(
        packets[3],
        Packet::List(vec![
            Packet::List(vec![Packet::Integer(1)]),
            Packet::Integer(4),
        ])
    );
}

#[test]
fn test_parse_packet_pairs() {
    let contents = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n";
    let packet_pairs = parse_packet_pairs(contents);
    assert_eq!(packet_pairs.len(), 2);

    assert_eq!(
        packet_pairs[0],
        PacketPair {
            left: Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(3),
                Packet::Integer(1),
                Packet::Integer(1)
            ]),
            right: Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(5),
                Packet::Integer(1),
                Packet::Integer(1)
            ]),
        }
    );

    assert_eq!(
        packet_pairs[1],
        PacketPair {
            left: Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::List(vec![
                    Packet::Integer(2),
                    Packet::Integer(3),
                    Packet::Integer(4),
                ]),
            ]),
            right: Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::Integer(4),
            ]),
        }
    );
}

#[test]
fn test_parse_packet_pair() {
    let input = "[1,1,3,1,1]\n[1,1,5,1,1]\n";
    let result = PacketPair::parse(input);
    assert!(result.is_ok());
    let (rest, pair) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        pair,
        PacketPair {
            left: Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(3),
                Packet::Integer(1),
                Packet::Integer(1)
            ]),
            right: Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(5),
                Packet::Integer(1),
                Packet::Integer(1)
            ]),
        }
    )
}

#[test]
fn test_parse_packet() {
    {
        let input = "[1,1,3,1,1]\n";
        let result = Packet::parse(input);
        assert!(result.is_ok());
        let (rest, packet) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            packet,
            Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(3),
                Packet::Integer(1),
                Packet::Integer(1)
            ])
        );
    }

    {
        let input = "[]\n";
        let result = Packet::parse(input);
        assert!(result.is_ok());
        let (rest, packet) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(packet, Packet::List(vec![]));
    }

    {
        let input = "[[]]\n";
        let result = Packet::parse(input);
        assert!(result.is_ok());
        let (rest, packet) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(packet, Packet::List(vec![Packet::List(vec![])]));
    }

    {
        let input = "[[4,4],4,4]\n";
        let result = Packet::parse(input);
        assert!(result.is_ok());
        let (rest, packet) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            packet,
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(4), Packet::Integer(4)]),
                Packet::Integer(4),
                Packet::Integer(4)
            ])
        );
    }
}

#[test]
fn test_is_ordered() {
    let contents = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n";
    let packet_pairs = parse_packet_pairs(contents);

    assert!(packet_pairs[0].is_ordered());
    assert!(packet_pairs[1].is_ordered());
    assert!(!packet_pairs[2].is_ordered());
    assert!(packet_pairs[3].is_ordered());
    assert!(!packet_pairs[4].is_ordered());
    assert!(packet_pairs[5].is_ordered());
    assert!(!packet_pairs[6].is_ordered());
    assert!(!packet_pairs[7].is_ordered());
}

#[test]
fn test_ordered_index_sum() {
    let contents = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n";
    let packet_pairs = parse_packet_pairs(contents);
    let index_sum = ordered_index_sum(&packet_pairs);
    assert_eq!(index_sum, 13);
}

#[test]
fn test_sort_with_divider_packets() {
    let contents = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n";
    let mut packets = parse_all_packets(&contents);
    packets.extend(generate_divider_packets());
    packets.sort();

    let mut result = String::new();
    for packet in &packets {
        result.push_str(&format!("{}\n", packet));
    }
    assert_eq!(result, "[]\n[[]]\n[[[]]]\n[1,1,3,1,1]\n[1,1,5,1,1]\n[[1],[2,3,4]]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[[1],4]\n[[2]]\n[3]\n[[4,4],4,4]\n[[4,4],4,4,4]\n[[6]]\n[7,7,7]\n[7,7,7,7]\n[[8,7,6]]\n[9]\n");
}

#[test]
fn test_calc_decoder_key() {
    let contents = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n";
    let mut packets = parse_all_packets(&contents);
    packets.extend(generate_divider_packets());
    packets.sort();
    let decoder_key = calc_decoder_key(&packets);
    assert_eq!(decoder_key, 140);
}
