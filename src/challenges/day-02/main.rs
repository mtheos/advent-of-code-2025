use aoc_2025::helpers::{read_file, Parsed, Parser};

pub const NAME: &str = "Day 02 - Gift Shop";

fn main() {
    println!("{}", NAME);
    let input = read_file("./src/challenges/day-02/input.txt", RangeParser {});
    let result = run_easy(&input);
    println!("Count: {}", result.invalid_count);
    println!("Count: {}", result.invalid_sum);
    let result = run_hard(&input);
    println!("Count: {}", result.invalid_count);
    println!("Count: {}", result.invalid_sum);
}

fn run_easy(input: &Vec<Range>) -> Answer {
    let mut invalid_count = 0;
    let mut invalid_sum = 0;
    input.iter().for_each(|range| {
        for id in range.start..=range.end {
            let len = id.ilog10() + 1;
            if len % 2 != 0 {
                continue;
            }
            let half_len = len / 2;
            let nibble = get_n_digits(id, half_len);
            let repeated = repeat_nibble(nibble, len);
            if repeated == id {
                invalid_count += 1;
                invalid_sum += id;
            }
        }
    });
    Answer {
        invalid_count,
        invalid_sum,
    }
}

fn run_hard(input: &Vec<Range>) -> Answer {
    let mut invalid_count = 0;
    let mut invalid_sum = 0;
    input.iter().for_each(|range| {
        for id in range.start..=range.end {
            let len = id.ilog10() + 1;
            let half_len = len / 2;
            let mut digit_count = 1;
            while digit_count <= half_len {
                let nibble = get_n_digits(id, digit_count);
                let repeated = repeat_nibble(nibble, len);
                if repeated == id {
                    invalid_count += 1;
                    invalid_sum += id;
                    break;
                }
                digit_count += 1;
            }
        }
    });
    Answer {
        invalid_count,
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
    invalid_count: u32,
    invalid_sum: u64,
}

struct Range {
    start: u64,
    end: u64,
}

struct RangeParser {}

impl Parser<Range> for RangeParser {
    fn parse(&self, line: &str) -> Parsed<Range> {
        let ranges = line
            .split(",")
            .map(|range| {
                let (start, end) = range.split_once("-").unwrap();
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                Range { start, end }
            })
            .collect::<Vec<Range>>();
        Parsed::Many(ranges)
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_n_digits, Parser};
    use crate::{repeat_nibble, run_easy, run_hard, Range, RangeParser};

    #[test]
    fn test_sample_input_easy() {
        let sample_input =
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let parser = RangeParser {};
        let input = parser.parse(sample_input).many();
        let result = run_easy(&input);
        assert_eq!(result.invalid_count, 8);
        assert_eq!(result.invalid_sum, 1227775554);
    }

    #[test]
    fn test_sample_input_hard() {
        let sample_input =
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let parser = RangeParser {};
        let input = parser.parse(sample_input).many();
        let result = run_hard(&input);
        assert_eq!(result.invalid_count, 13);
        assert_eq!(result.invalid_sum, 4174379265);
    }

    #[test]
    fn test_parser() {
        let parser = RangeParser {};
        let result = parser.parse("11-22").many().pop().unwrap();
        assert_eq!(result.start, 11);
        assert_eq!(result.end, 22);
        let result = parser.parse("824824821-824824827").many().pop().unwrap();
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
        let result = run_easy(&input);
        assert_eq!(result.invalid_count, 1);
    }

    #[test]
    fn test_hard_1() {
        let input = vec![Range {
            start: 100,
            end: 125,
        }];
        let result = run_hard(&input);
        assert_eq!(result.invalid_count, 1);
    }
}
