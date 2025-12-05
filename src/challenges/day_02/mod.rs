use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};

const NAME: &str = "Gift Shop";
const DAY: &str = "02";

pub struct State {
    input: Vec<Range>,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = RangeParser {}.parse(reader);
        State { input }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }

    fn run_easy(&mut self) -> String {
        let Answer {
            invalid_sum,
        } = do_easy(self);
        format!("Invalid Sum: {invalid_sum}")
    }

    fn run_hard(&mut self) -> String {
        let Answer {
            invalid_sum,
        } = do_hard(self);
        format!("Invalid Sum: {invalid_sum}")
    }
}

fn do_easy(state: &State) -> Answer {
    let mut invalid_sum = 0;
    state.input.iter().for_each(|range| {
        for id in range.start..=range.end {
            let len = id.ilog10() + 1;
            if len % 2 != 0 {
                continue;
            }
            let half_len = len / 2;
            let nibble = get_n_digits(id, half_len);
            let repeated = repeat_nibble(nibble, len);
            if repeated == id {
                invalid_sum += id;
            }
        }
    });
    Answer {
        invalid_sum,
    }
}

fn do_hard(state: &State) -> Answer {
    let mut invalid_sum = 0;
    state.input.iter().for_each(|range| {
        for id in range.start..=range.end {
            let len = id.ilog10() + 1;
            let half_len = len / 2;
            let mut digit_count = 1;
            while digit_count <= half_len {
                let nibble = get_n_digits(id, digit_count);
                let repeated = repeat_nibble(nibble, len);
                if repeated == id {
                    invalid_sum += id;
                    break;
                }
                digit_count += 1;
            }
        }
    });
    Answer {
        invalid_sum,
    }
}

fn get_n_digits(number: u64, num_digits: u32) -> u64 {
    let digits = number.ilog10() + 1;
    let mag = digits - num_digits;
    number.div_euclid(10_u64.pow(mag))
}

fn repeat_nibble(nibble: u64, total_len: u32) -> u64 {
    let digits = nibble.ilog10() + 1;
    let mag = 10_u64.pow(digits);
    let count = total_len.div_euclid(digits);
    let gp = 1 * ((mag.pow(count) - 1) / (mag - 1));
    gp * nibble
}

struct Answer {
    invalid_sum: u64,
}

struct Range {
    start: u64,
    end: u64,
}

struct RangeParser {}

impl RangeParser {
    fn parse(&self, reader: Reader) -> Vec<Range> {
        reader
            .into_iter()
            .next()
            .unwrap()
            .split(",")
            .map(|range| {
                let (start, end) = range.split_once("-").unwrap();
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                Range { start, end }
            })
            .collect::<Vec<Range>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_02::{
        do_easy, do_hard, get_n_digits, repeat_nibble, Range, RangeParser, State, DAY,
    };
    use crate::helpers::{Reader, PREFIX};

    #[test]
    fn test_sample_input_easy() {
        let input = RangeParser {}.parse(Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = do_easy(&state);
        assert_eq!(result.invalid_sum, 1227775554);
    }

    #[test]
    fn test_sample_input_hard() {
        let input = RangeParser {}.parse(Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let state = State { input };
        let result = do_hard(&state);
        assert_eq!(result.invalid_sum, 4174379265);
    }

    #[test]
    fn test_parser() {
        let parser = RangeParser {};
        let result = parser.parse(Reader::single("11-22")).pop().unwrap();
        assert_eq!(result.start, 11);
        assert_eq!(result.end, 22);
        let result = parser
            .parse(Reader::single("824824821-824824827"))
            .pop()
            .unwrap();
        assert_eq!(result.start, 824824821);
        assert_eq!(result.end, 824824827);
    }

    #[test]
    fn test_get_n_digits() {
        let res = get_n_digits(345678, 1);
        assert_eq!(res, 3);
        let res = get_n_digits(345678, 2);
        assert_eq!(res, 34);
        let res = get_n_digits(345678, 3);
        assert_eq!(res, 345);
        let res = get_n_digits(345678, 4);
        assert_eq!(res, 3456);
    }

    #[test]
    fn test_repeat_nibble() {
        let res = repeat_nibble(1, 4);
        assert_eq!(res, 1111);
        let res = repeat_nibble(34, 4);
        assert_eq!(res, 3434);
        let res = repeat_nibble(345, 9);
        assert_eq!(res, 345345345);
        let res = repeat_nibble(3456, 8);
        assert_eq!(res, 34563456);
    }

    #[test]
    fn test_easy_1() {
        let input = vec![Range { start: 1, end: 15 }];
        let state = State { input };
        let result = do_easy(&state);
        assert_eq!(result.invalid_sum, 11);
    }

    #[test]
    fn test_hard_1() {
        let input = vec![Range {
            start: 100,
            end: 125,
        }];
        let state = State { input };
        let result = do_hard(&state);
        assert_eq!(result.invalid_sum, 111);
    }
}
