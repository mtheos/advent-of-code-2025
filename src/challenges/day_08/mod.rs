use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

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
        format!("Part 2: No Result")
    }
}

fn connect_circuits(
    state: &State,
    mut junctions_to_connect: usize,
    junctions_to_count: usize,
) -> Answer {
    let junctions = &state.input.junctions;
    let mut circuits: Vec<Circuit> = Vec::new();
    let distance_cache: HashMap<(&Junction, &Junction), u64> = precompute_distances(junctions);
    while junctions_to_connect > 0 {
        let (first, second) = find_closest_pair(junctions, &circuits, &distance_cache).unwrap();
        let first_idx = get_circuit_idx(&first, &circuits);
        let second_idx = get_circuit_idx(&second, &circuits);
        match (first_idx, second_idx) {
            (Some(f), Some(s)) => {
                let (hi, lo) = if f > s { (f, s) } else { (s, f) };
                let c2 = circuits.remove(hi);
                circuits[lo].merge(&c2);
            }
            (Some(f), None) => circuits[f].add(second),
            (None, Some(s)) => circuits[s].add(first),
            (None, None) => {
                let circuit = Circuit {
                    junctions: HashSet::from([first, second]),
                };
                circuits.push(circuit);
            }
        }
        junctions_to_connect -= 1;
    }
    circuits.sort_unstable_by_key(|c| Reverse(c.junctions.len()));
    let largest_circuits_space = circuits
        .iter()
        .take(junctions_to_count)
        .map(|c| c.junctions.len())
        .reduce(|a, b| a * b)
        .unwrap();
    Answer {
        largest_circuits_space,
    }
}

fn find_closest_pair<'a>(
    junctions: &'a Vec<Junction>,
    circuits: &Vec<Circuit>,
    distance_cache: &HashMap<(&'a Junction, &'a Junction), u64>,
) -> Option<(&'a Junction, &'a Junction)> {
    let mut distance: u64 = u64::MAX;
    let mut first: Option<&Junction> = None;
    let mut second: Option<&Junction> = None;
    for i in 0..junctions.len() - 1 {
        let first_candidate = &junctions[i];
        for j in i + 1..junctions.len() {
            let second_candidate = &junctions[j];
            let cache_key = &(first_candidate, second_candidate);
            let circuit =
                get_circuit_idx(first_candidate, circuits).and_then(|idx| circuits.get(idx));
            let contained = match circuit {
                Some(c) => c.contains(second_candidate),
                None => false,
            };
            if !contained {
                let distance_candidate = distance_cache[cache_key];
                if distance_candidate < distance {
                    distance = distance_candidate;
                    first = Some(first_candidate);
                    second = Some(second_candidate);
                }
            }
        }
    }

    match (first, second) {
        (Some(first), Some(second)) => Some((first, second)),
        (None, None) => None,
        (None, Some(_)) | (Some(_), None) => panic!("Did not expect to only match 1 candidate"),
    }
}

fn precompute_distances(junctions: &Vec<Junction>) -> HashMap<(&Junction, &Junction), u64> {
    let mut distance_cache: HashMap<(&Junction, &Junction), u64> = HashMap::new();
    for i in 0..junctions.len() - 1 {
        let first = &junctions[i];
        for j in i + 1..junctions.len() {
            let second = &junctions[j];
            let cache_key = &(first, second);
            match first.distance_squared(second) {
                0 => panic!("That's the same circuit!"),
                d => distance_cache.insert(*cache_key, d),
            };
        }
    }
    distance_cache
}

fn get_circuit_idx(junction: &Junction, circuits: &Vec<Circuit>) -> Option<usize> {
    circuits.iter().position(|c| c.contains(junction))
}

struct Answer {
    largest_circuits_space: usize,
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
        connect_circuits, find_closest_pair, Circuit, CoordinateParser, Junction, State, DAY,
    };
    use crate::helpers::{Reader, PREFIX};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_sample_input_easy() {
        let input = CoordinateParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = connect_circuits(&state, 10, 3);
        assert_eq!(result.largest_circuits_space, 40);
    }

    #[test]
    fn test_sample_input_hard() {
        let input = CoordinateParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = connect_circuits(&state, 100, 3);
        assert_eq!(result.largest_circuits_space, 40);
    }

    #[test]
    fn test_find_closest_pair() {
        let junctions = vec![
            Junction { x: 5, y: 0, z: 0 },
            Junction { x: 25, y: 0, z: 0 },
            Junction { x: 10, y: 0, z: 0 },
        ];
        let (first, second) =
            find_closest_pair(&junctions, &Vec::new(), &mut HashMap::new()).unwrap();
        assert_eq!(*first, Junction { x: 5, y: 0, z: 0 });
        assert_eq!(*second, Junction { x: 10, y: 0, z: 0 });
    }

    #[test]
    fn test_find_closest_pair_with_circuits() {
        let junctions = vec![
            Junction { x: 5, y: 0, z: 0 },
            Junction { x: 10, y: 0, z: 0 },
            Junction { x: 25, y: 0, z: 0 },
        ];
        let circuit = Circuit {
            junctions: HashSet::from([
                &Junction { x: 5, y: 0, z: 0 },
                &Junction { x: 10, y: 0, z: 0 },
            ]),
        };
        let circuits = vec![circuit];
        let (first, second) =
            find_closest_pair(&junctions, &circuits, &mut HashMap::new()).unwrap();
        assert_eq!(*first, Junction { x: 5, y: 0, z: 0 });
        assert_eq!(*second, Junction { x: 25, y: 0, z: 0 });
    }

    #[test]
    fn test_find_closest_pair_all_connected() {
        let junctions = vec![
            Junction { x: 5, y: 0, z: 0 },
            Junction { x: 10, y: 0, z: 0 },
            Junction { x: 25, y: 0, z: 0 },
        ];
        let circuit = Circuit {
            junctions: HashSet::from_iter(&junctions),
        };
        let circuits = vec![circuit];
        let result = find_closest_pair(&junctions, &circuits, &mut HashMap::new());
        assert!(result.is_none());
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
