use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};

const NAME: &str = "Movie Theater";
const DAY: &str = "09";

pub struct State {
    input: Input,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let mut reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = Parser {}.parse(&mut reader);
        State { input }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }

    fn run_easy(&mut self) -> String {
        format!("Part 1: No Result")
    }

    fn run_hard(&mut self) -> String {
        format!("Part 2: No Result")
    }
}

struct Answer {
}

struct Input {
}

struct Parser {}

impl Parser {
    fn parse(&self, reader: &mut Reader) -> Input {
        Input {
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_09::{Parser, State, DAY};
    use crate::challenges::Challenge;
    use crate::helpers::{Reader, PREFIX};

    #[test]
    fn test_sample_input_easy() {
        let input = Parser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let mut state = State { input };
        let result = state.run_easy();
        assert_eq!(21, 21);
    }

    #[test]
    fn test_sample_input_hard() {
        let input = Parser {}.parse(&mut Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let mut state = State { input };
        let result = state.run_hard();
        assert_eq!(40, 40);
    }

    #[test]
    fn test_problem_parser() {
        let result = Parser {}.parse(&mut Reader::from_vec(vec!["..S..", ".^.^.", "^...^"]));
        assert_eq!(3, 3);
        assert_eq!(5, 5);
    }
}
