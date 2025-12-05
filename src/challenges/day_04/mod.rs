use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};
use std::ops::{Index, IndexMut};
use strum_macros::Display;

const NAME: &str = "Printing Department";
const DAY: &str = "04";

pub struct State {
    input: FactoryFloor,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = FactoryFloorParser {}.parse(reader);
        State { input }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }

    fn run_easy(&mut self) -> String {
        let Answer { available_rolls } = do_easy(&self);
        format!("Available: {available_rolls}")
    }

    fn run_hard(&mut self) -> String {
        let Answer { available_rolls } = do_hard(&self);
        format!("Available: {available_rolls}")
    }
}

fn do_easy(state: &State) -> Answer {
    let mut available_rolls = 0;
    for r in 0..state.input.rows {
        for c in 0..state.input.cols {
            if state.input[(r, c)] == Contents::Roll && is_available((r, c), &state.input) {
                available_rolls += 1;
            }
        }
    }
    Answer { available_rolls }
}

fn do_hard(state: &State) -> Answer {
    let input = &mut state.input.clone();
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
    use crate::challenges::day_04::{
        do_easy, do_hard, is_available, Contents, FactoryFloorParser, State, DAY,
    };
    use crate::helpers::{Reader, PREFIX};

    #[test]
    fn test_sample_input_easy() {
        let input =
            FactoryFloorParser {}.parse(Reader::from_file(format!("{PREFIX}_{DAY}/sample.txt").as_str()));
        let state = State { input };
        let result = do_easy(&state);
        assert_eq!(result.available_rolls, 13);
    }

    #[test]
    fn test_sample_input_hard() {
        let input =
            FactoryFloorParser {}.parse(Reader::from_file(format!("{PREFIX}_{DAY}/sample.txt").as_str()));
        let state = State { input };
        let result = do_hard(&state);
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
