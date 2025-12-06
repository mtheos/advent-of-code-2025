use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};

const NAME: &str = "Trash Compactor";
const DAY: &str = "06";

pub struct State {
    input: PartialProblem,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let mut reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = ProblemParser {}.parse(&mut reader);
        State { input }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }

    fn run_easy(&mut self) -> String {
        let Answer { sum_of_problems } = do_easy(&self);
        format!("Sum of Problems: {sum_of_problems}")
    }

    fn run_hard(&mut self) -> String {
        let Answer { sum_of_problems } = do_hard(&self);
        format!("Sum of Problems: {sum_of_problems}")
    }
}

fn do_easy(state: &State) -> Answer {
    let PartialProblem { lines, operands } = &state.input;
    let numbers = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u64>>>();
    let sum_of_problems = (0..operands.len())
        .map(|i| {
            let numbers = numbers
                .iter()
                .map(|n| n[i])
                .collect::<Vec<u64>>();
            let operand = operands[i].clone();
            solve_problem(&Problem { numbers, operand })
        })
        .sum();
    Answer { sum_of_problems }
}

fn do_hard(state: &State) -> Answer {
    let PartialProblem { lines, operands } = &state.input;
    let sum_of_problems = make_stupid_numbers(lines, operands)
        .iter()
        .map(|problem| solve_problem(problem))
        .sum();
    Answer { sum_of_problems }
}

fn make_stupid_numbers(lines: &Vec<String>, operands: &Vec<Operand>) -> Vec<Problem> {
    let mut problems = Vec::with_capacity(operands.len());
    let mut i = 0;
    while problems.len() < operands.len() {
        let mut numbers: Vec<u64> = Vec::with_capacity(10);
        let operand = operands[problems.len()].clone();
        loop {
            match make_stupid_number(i, lines) {
                Some(num) => numbers.push(num),
                None => {
                    problems.push(Problem { numbers, operand });
                    i += 1;
                    break;
                }
            }
            i += 1;
        }
    }
    problems
}

fn make_stupid_number(i: usize, lines: &Vec<String>) -> Option<u64> {
    let mut result: u64 = 0;
    let mut found = false;
    for line in lines {
        if let Some(ch) = line.as_bytes().get(i) {
            if ch.is_ascii_digit() {
                result = result * 10 + (ch - b'0') as u64;
                found = true;
            }
        };
    }
    if found {
        Some(result)
    } else {
        None
    }
}

fn solve_problem(problem: &Problem) -> u64 {
    match problem.operand {
        Operand::Add => problem.numbers.iter().sum(),
        Operand::Mul => problem.numbers.iter().product(),
    }
}

struct Answer {
    sum_of_problems: u64,
}

#[derive(Clone, PartialEq, Debug)]
enum Operand {
    Add,
    Mul,
}

#[derive(Clone)]
struct Problem {
    numbers: Vec<u64>,
    operand: Operand,
}

#[derive(Clone)]
struct PartialProblem {
    lines: Vec<String>,
    operands: Vec<Operand>,
}

struct ProblemParser {}

impl ProblemParser {
    fn parse(&self, reader: &mut Reader) -> PartialProblem {
        let mut lines = reader.collect::<Vec<String>>();
        let operands = lines
            .pop()
            .unwrap()
            .split_whitespace()
            .map(|x| match x {
                "+" => Operand::Add,
                "*" => Operand::Mul,
                _ => panic!("Unknown operator"),
            })
            .collect::<Vec<Operand>>();
        PartialProblem { lines, operands }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_06::{do_easy, do_hard, Operand, ProblemParser, State, DAY};
    use crate::helpers::{Reader, PREFIX};

    #[test]
    fn test_sample_input_easy() {
        let input = ProblemParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = do_easy(&state);
        assert_eq!(result.sum_of_problems, 4277556);
    }

    #[test]
    fn test_sample_input_hard() {
        let input = ProblemParser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = do_hard(&state);
        assert_eq!(result.sum_of_problems, 3263827);
    }

    #[test]
    fn test_problem_parser() {
        let result = ProblemParser {}.parse(&mut Reader::from_vec(vec![
            "1 5", "2 6", "3 7", "4 8", "+ *",
        ]));
        assert_eq!(result.operands.len(), 2);
        assert_eq!(result.operands[0], Operand::Add);
        assert_eq!(result.operands[1], Operand::Mul);
        assert_eq!(result.lines.len(), 4);
        assert_eq!(result.lines[0], "15");
        assert_eq!(result.lines[1], "26");
        assert_eq!(result.lines[2], "37");
        assert_eq!(result.lines[3], "48");
    }
}
