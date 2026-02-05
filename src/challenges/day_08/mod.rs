use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};
use std::cmp::Reverse;
use std::collections::HashSet;

const NAME: &str = "Playground";
const DAY: &str = "08";

pub struct State {
    input: Input,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let mut reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = CoordinateParser {}.parse(&mut reader);
        State { input }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }

    fn run_easy(&mut self) -> String {
        let result = connect_circuits(&self, 1000, 3);
        format!("Largest Circuits space: {}", result.largest_circuits_space)
    }

    fn run_hard(&mut self) -> String {
        let max = self.input.junctions.len() * (self.input.junctions.len() - 1) / 2;
        let result = connect_circuits(&self, max, 1);
        format!("Last span: {}", result.last_span)
    }
}

fn connect_circuits(
    state: &State,
    mut max_junctions_to_connect: usize,
    circuits_to_count: usize,
) -> Answer {
    let junctions = &state.input.junctions;
    let mut circuits: Vec<Circuit> = Vec::new();
    let items: Vec<JunctionPair> = compute_distances(junctions);
    let mut it = items.iter();
    let mut last_span = 0u64;
    while max_junctions_to_connect > 0 {
        let item = it.next().unwrap();
        let first_idx = get_circuit_idx(&item.first, &circuits);
        let second_idx = get_circuit_idx(&item.second, &circuits);
        last_span = item.first.x * item.second.x;
        match (first_idx, second_idx) {
            (Some(f), Some(s)) if f != s => {
                let (hi, lo) = if f > s { (f, s) } else { (s, f) };
                let c2 = circuits.remove(hi);
                circuits[lo].merge(&c2);
            }
            (Some(f), Some(s)) if f == s => {}
            (Some(f), Some(s)) => panic!("Match statement was non-exhaustive. Found {f} and {s}"),
            (Some(f), None) => circuits[f].add(item.second),
            (None, Some(s)) => circuits[s].add(item.first),
            (None, None) => {
                let circuit = Circuit {
                    junctions: HashSet::from([item.first, item.second]),
                };
                circuits.push(circuit);
            }
        }
        if circuits.len() == 1 && circuits[0].junctions.len() == state.input.junctions.len() {
            break;
        }
        max_junctions_to_connect -= 1;
    }
    circuits.sort_unstable_by_key(|c| Reverse(c.junctions.len()));
    let largest_circuits_space = circuits
        .iter()
        .take(circuits_to_count)
        .map(|c| c.junctions.len())
        .reduce(|a, b| a * b)
        .unwrap();
    Answer {
        largest_circuits_space,
        last_span,
    }
}

fn compute_distances(junctions: &Vec<Junction>) -> Vec<JunctionPair<'_>> {
    let mut distances: Vec<JunctionPair> = Vec::new();
    for i in 0..junctions.len() - 1 {
        let first = &junctions[i];
        for j in i + 1..junctions.len() {
            let second = &junctions[j];
            match first.distance_squared(second) {
                0 => panic!("That's the same circuit!"),
                distance_squared => distances.push(JunctionPair {
                    first,
                    second,
                    distance_squared,
                }),
            };
        }
    }
    let mut sorted = distances;
    sorted.sort_by_key(|p| p.distance_squared);
    sorted
}

#[derive(Debug, PartialEq)]
struct JunctionPair<'a> {
    first: &'a Junction,
    second: &'a Junction,
    distance_squared: u64,
}

fn get_circuit_idx(junction: &Junction, circuits: &Vec<Circuit>) -> Option<usize> {
    circuits.iter().position(|c| c.contains(junction))
}

struct Answer {
    largest_circuits_space: usize,
    last_span: u64,
}

struct Input {
    junctions: Vec<Junction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Circuit<'a> {
    junctions: HashSet<&'a Junction>,
}

impl<'a> Circuit<'a> {
    fn merge(&mut self, other: &Circuit<'a>) -> () {
        self.junctions.extend(&other.junctions);
    }
    fn add(&mut self, junction: &'a Junction) -> () {
        self.junctions.insert(junction);
    }
    fn contains(&self, junction: &Junction) -> bool {
        self.junctions.contains(junction)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Junction {
    x: u64,
    y: u64,
    z: u64,
}

impl Junction {
    fn distance_squared(&self, other: &Junction) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct CoordinateParser {}

impl CoordinateParser {
    fn parse(&self, reader: &mut Reader) -> Input {
        let junctions = reader
            .map(|line| {
                let [x, y, z] = line
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()[..]
                else {
                    panic!("Line did not split into 3 coords")
                };
                Junction { x, y, z }
            })
            .collect();
        Input { junctions }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_08::{
        compute_distances, connect_circuits, CoordinateParser, Junction, JunctionPair, State, DAY,
    };
    use crate::helpers::{Reader, PREFIX};

    #[test]
    fn test_sample_input_easy() {
        let input = CoordinateParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = connect_circuits(&state, 10, 3);
        assert_eq!(result.largest_circuits_space, 40);
        assert_eq!(result.last_span, 891504);
    }

    #[test]
    fn test_sample_input_hard() {
        let input = CoordinateParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = connect_circuits(&state, 1000, 3);
        assert_eq!(result.largest_circuits_space, state.input.junctions.len());
        assert_eq!(result.last_span, 25272);
    }

    #[test]
    fn test_compute_distances() {
        let junctions = vec![
            Junction { x: 5, y: 0, z: 0 },
            Junction { x: 25, y: 0, z: 0 },
            Junction { x: 10, y: 0, z: 0 },
        ];
        let distances = compute_distances(&junctions);
        assert_eq!(distances.len(), 3);
        assert_eq!(
            distances[0],
            JunctionPair {
                first: &Junction { x: 5, y: 0, z: 0 },
                second: &Junction { x: 10, y: 0, z: 0 },
                distance_squared: 25
            }
        );
        assert_eq!(
            distances[1],
            JunctionPair {
                first: &Junction { x: 25, y: 0, z: 0 },
                second: &Junction { x: 10, y: 0, z: 0 },
                distance_squared: 225
            }
        );
        assert_eq!(
            distances[2],
            JunctionPair {
                first: &Junction { x: 5, y: 0, z: 0 },
                second: &Junction { x: 25, y: 0, z: 0 },
                distance_squared: 400
            }
        );
    }

    #[test]
    fn test_problem_parser() {
        let result =
            CoordinateParser {}.parse(&mut Reader::from_vec(vec!["162,817,812", "57,618,57"]));
        assert_eq!(
            result.junctions[0],
            Junction {
                x: 162,
                y: 817,
                z: 812
            }
        );
        assert_eq!(
            result.junctions[1],
            Junction {
                x: 57,
                y: 618,
                z: 57
            }
        );
    }
}
