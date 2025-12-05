use aoc_2025::helpers::{time_it, Reader};
use std::ops::{Index, IndexMut};
use strum_macros::Display;

pub const NAME: &str = "Day 04 - Printing Department";
pub const PREFIX: &str = "./src/challenges/day-04";

fn main() {
    println!("{}", NAME);
    let reader = Reader::from_file(format!("{PREFIX}/input.txt").as_str());
    let input = FactoryFloorParser {}.parse(reader);
    let (result, duration) = time_it(|| run_easy(&input));
    println!("Easy: {duration:?}");
    println!("Available: {}", result.available_rolls);
    let (result, duration) = time_it(|| run_hard(&mut input.clone()));
    println!("Hard: {duration:?}");
    println!("Available: {}", result.available_rolls);
}

fn run_easy(input: &FactoryFloor) -> Answer {
    let mut available_rolls = 0;
    for r in 0..input.rows {
        for c in 0..input.cols {
            if input[(r, c)] == Contents::Roll && is_available((r, c), input) {
                available_rolls += 1;
            }
        }
    }
    Answer { available_rolls }
}

fn run_hard(input: &mut FactoryFloor) -> Answer {
    let mut available_rolls = 0;
    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    loop {
        for r in 0..input.rows {
            for c in 0..input.cols {
                if input[(r, c)] == Contents::Roll && is_available((r, c), input) {
                    to_remove.push((r, c))
                }
            }
        }
        if to_remove.is_empty() {
            break;
        } else {
            available_rolls += to_remove.len();
            to_remove.iter().for_each(|&roll| {
                input[roll] = Contents::Marked;
            });
            to_remove.clear();
        }
    }
    let available_rolls = available_rolls as u64;
    Answer { available_rolls }
}

fn is_available((r, c): (usize, usize), factory_floor: &FactoryFloor) -> bool {
    let (r, c) = (r as i32, c as i32);
    let steps: [i32; 3] = [-1, 0, 1];
    let mut filled = 0;
    for x in steps {
        for y in steps {
            if x == 0 && y == 0 {
                continue;
            }
            match factory_floor.safe_get(((r + x) as usize, (c + y) as usize)) {
                Contents::Empty => {}
                Contents::Roll => filled += 1,
                Contents::Marked => {}
            }
        }
    }
    filled < 4
}

struct Answer {
    available_rolls: u64,
}

#[derive(Display, Clone, Debug, PartialEq)]
enum Contents {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "@")]
    Roll,
    #[strum(serialize = "X")]
    Marked,
}

#[derive(Clone)]
struct FactoryFloor {
    cols: usize,
    rows: usize,
    floor: Vec<Contents>,
}

impl FactoryFloor {
    fn idx(&self, (r, c): (usize, usize)) -> Option<usize> {
        if r < self.rows && c < self.cols {
            Some(r * self.cols + c)
        } else {
            None
        }
    }

    fn safe_get(&self, (r, c): (usize, usize)) -> &Contents {
        self.idx((r, c))
            .map(|x| &self.floor[x])
            .unwrap_or(&Contents::Empty)
    }
}

impl Index<(usize, usize)> for FactoryFloor {
    type Output = Contents;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.idx(index).map(|x| &self.floor[x]).unwrap()
    }
}

impl IndexMut<(usize, usize)> for FactoryFloor {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.idx(index).map(|x| &mut self.floor[x]).unwrap()
    }
}

struct FactoryFloorParser {}

impl FactoryFloorParser {
    fn parse(&self, reader: Reader) -> FactoryFloor {
        let mut cols = 0;
        let mut rows = 0;
        let floor = reader
            .map(|line| {
                cols = line.len();
                rows += 1;
                let floor = line
                    .chars()
                    .map(|char| match char {
                        '.' => Contents::Empty,
                        '@' => Contents::Roll,
                        'X' => Contents::Marked,
                        _ => panic!("Unreachable state (...or so I hoped)"),
                    })
                    .collect::<Vec<Contents>>();
                FactoryFloor { cols, rows, floor }
            })
            .flat_map(|x| x.floor)
            .collect();
        FactoryFloor { cols, rows, floor }
    }
}

#[cfg(test)]
mod tests {
    use crate::PREFIX;
    use crate::{is_available, run_easy, run_hard, Contents, FactoryFloorParser};
    use aoc_2025::helpers::Reader;

    #[test]
    fn test_sample_input_easy() {
        let input =
            FactoryFloorParser {}.parse(Reader::from_file(format!("{PREFIX}/sample.txt").as_str()));
        let result = run_easy(&input);
        assert_eq!(result.available_rolls, 13);
    }

    #[test]
    fn test_sample_input_hard() {
        let input =
            FactoryFloorParser {}.parse(Reader::from_file(format!("{PREFIX}/sample.txt").as_str()));
        let result = run_hard(&mut input.clone());
        assert_eq!(result.available_rolls, 43);
    }

    #[test]
    fn test_parser() {
        let parser = FactoryFloorParser {};
        let result = parser.parse(Reader::from_vec(vec!["..@@.@@@@.", "@@..@....@"]));
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 10);
        assert_eq!(result.safe_get((1, 3)).clone(), Contents::Empty);
        assert_eq!(result.safe_get((0, 3)).clone(), Contents::Roll);
    }

    #[test]
    fn test_is_available() {
        let parser = FactoryFloorParser {};
        let input = parser.parse(Reader::from_vec(vec![
            "..........",
            "...@......",
            "..@.......",
            ".@.@..@.@.",
            ".......@..",
            "......@.@.",
        ]));
        assert_eq!(input.safe_get((2, 2)).clone(), Contents::Roll);
        assert_eq!(input.safe_get((4, 7)).clone(), Contents::Roll);
        let res = is_available((2, 2), &input);
        assert_eq!(res, true);
        let res = is_available((4, 7), &input);
        assert_eq!(res, false);
    }
}
