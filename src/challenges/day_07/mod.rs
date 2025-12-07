use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use strum_macros::Display;

const NAME: &str = "Laboratories";
const DAY: &str = "07";

pub struct State {
    input: Grid,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let mut reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = GridParser {}.parse(&mut reader);
        State { input }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }

    fn run_easy(&mut self) -> String {
        let Answer { tachyon_splits, .. } = run_manifold(self);
        format!("Tachyon Splits: {tachyon_splits}")
    }

    fn run_hard(&mut self) -> String {
        let Answer { tachyon_timelines, .. } = run_manifold(self);
        format!("Tachyon Timelines: {tachyon_timelines}")
    }
}

fn run_manifold(state: &mut State) -> Answer {
    let mut input = state.input.clone();
    let start = input
        .get_row(0)
        .iter()
        .position(|x| *x == Contents::Emitter)
        .unwrap();

    let mut tachyon_splits = 0;
    let mut tachyons = HashSet::new();
    tachyons.insert(start);
    let mut quantum_tachyons: HashMap<(usize, usize), usize> = HashMap::new();
    quantum_tachyons.insert((0, start), 1);

    let cols = input.cols;
    let rows = input.rows;
    let mut next_row = 1_usize;
    while next_row < rows {
        let mut next_tachyons: HashSet<usize> = HashSet::new();
        let row: &mut [Contents] = input.get_row_mut(next_row);
        for col in tachyons.iter() {
            let incoming_beams = quantum_tachyons[&(next_row - 1, *col)];
            match row[*col] {
                Contents::Splitter => {
                    tachyon_splits += 1;
                    if *col > 0 {
                        row[col - 1] = Contents::Tachyon;
                        let beams = quantum_tachyons.get(&(next_row, col - 1)).unwrap_or(&0);
                        quantum_tachyons.insert((next_row, col - 1), beams + incoming_beams);
                        next_tachyons.insert(col - 1);
                    }
                    if col + 1 < cols {
                        row[col + 1] = Contents::Tachyon;
                        let beams = quantum_tachyons.get(&(next_row, col + 1)).unwrap_or(&0);
                        quantum_tachyons.insert((next_row, col + 1), beams + incoming_beams);
                        next_tachyons.insert(col + 1);
                    }
                }
                Contents::Empty => {
                    row[*col] = Contents::Tachyon;
                    quantum_tachyons.insert((next_row, *col), incoming_beams);
                    next_tachyons.insert(*col);
                }
                Contents::Tachyon => {
                    let beams = quantum_tachyons.get(&(next_row, *col)).unwrap_or(&0);
                    quantum_tachyons.insert((next_row, *col), incoming_beams + beams);
                }
                Contents::Emitter => panic!("Unexpected state: Emitter"),
            }
        }
        next_row += 1;
        tachyons = next_tachyons;
    }
    // _display_tachyons(&quantum_tachyons, &input);
    let tachyon_timelines = quantum_tachyons
        .iter()
        .filter(|((r, _), _)| *r == input.rows - 1)
        .map(|(_, v)| *v)
        .sum::<usize>() as u64;
    Answer {
        tachyon_splits,
        tachyon_timelines,
    }
}

fn _display_tachyons(quantum_tachyons: &HashMap<(usize, usize), usize>, input: &Grid) {
    for row in 0..input.rows {
        for col in 0..input.cols {
            match input.get((row, col)) {
                Contents::Empty => print!("."),
                Contents::Splitter => print!("^"),
                Contents::Emitter => print!("S"),
                Contents::Tachyon => {
                    let t = quantum_tachyons.get(&(row, col)).unwrap_or(&0);
                    let c = match *t {
                        0..10 => t.to_string(),
                        10 => "X".to_string(),
                        11 => "Y".to_string(),
                        _ => "Z".to_string(),
                    };
                    print!("{}", c)
                }
            }
        }
        println!();
    }
    println!();
}

struct Answer {
    tachyon_splits: u64,
    tachyon_timelines: u64,
}

#[derive(Display, Clone, Debug, PartialEq)]
enum Contents {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "^")]
    Splitter,
    #[strum(serialize = "S")]
    Emitter,
    #[strum(serialize = "|")]
    Tachyon,
}

#[derive(Clone)]
struct Grid {
    cols: usize,
    rows: usize,
    manifold: Vec<Contents>,
}

impl Grid {
    fn idx(&self, (r, c): (usize, usize)) -> Option<usize> {
        if r < self.rows && c < self.cols {
            Some(r * self.cols + c)
        } else {
            None
        }
    }

    fn get(&self, (r, c): (usize, usize)) -> &Contents {
        self.idx((r, c)).map(|x| &self.manifold[x]).unwrap()
    }

    fn get_row(&self, r: usize) -> &[Contents] {
        let row_start = r * self.cols;
        &self.manifold[row_start..(row_start + self.cols)]
    }

    fn get_row_mut(&mut self, r: usize) -> &mut [Contents] {
        let row_start = r * self.cols;
        &mut self.manifold[row_start..(row_start + self.cols)]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.get((row, col)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct GridParser {}

impl GridParser {
    fn parse(&self, reader: &mut Reader) -> Grid {
        let mut rows = 0;
        let mut cols = 0;
        let manifold = reader
            .collect::<Vec<String>>()
            .iter()
            .flat_map(|row| {
                cols = row.len();
                rows += 1;
                row.chars().into_iter().map(|x| match x {
                    '.' => Contents::Empty,
                    '^' => Contents::Splitter,
                    'S' => Contents::Emitter,
                    _ => panic!("Unexpected case"),
                })
            })
            .collect::<Vec<Contents>>();
        Grid {
            rows,
            cols,
            manifold,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_07::{run_manifold, Contents, GridParser, State, DAY};
    use crate::helpers::{Reader, PREFIX};

    #[test]
    fn test_sample_input_easy() {
        let input = GridParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let mut state = State { input };
        let result = run_manifold(&mut state);
        assert_eq!(result.tachyon_splits, 21);
    }

    #[test]
    fn test_sample_input_hard() {
        let input = GridParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let mut state = State { input };
        let result = run_manifold(&mut state);
        assert_eq!(result.tachyon_timelines, 40);
    }

    #[test]
    fn test_problem_parser() {
        let result = GridParser {}.parse(&mut Reader::from_vec(vec!["..S..", ".^.^.", "^...^"]));
        assert_eq!(result.rows, 3);
        assert_eq!(result.cols, 5);
        assert_eq!(*result.get((0, 2)), Contents::Emitter);
        assert_eq!(*result.get((1, 1)), Contents::Splitter);
        assert_eq!(*result.get((1, 0)), Contents::Empty);
        assert_eq!(*result.get((2, 4)), Contents::Splitter);
        assert_eq!(*result.get((2, 2)), Contents::Empty);
        assert_eq!(
            *result.get_row(2),
            [
                Contents::Splitter,
                Contents::Empty,
                Contents::Empty,
                Contents::Empty,
                Contents::Splitter
            ]
        );
    }
}
