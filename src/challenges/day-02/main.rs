use aoc_2025::helpers;
use aoc_2025::helpers::{Parsed, Parser};

pub const NAME: &str = "Gift Shop";

fn main() {
    let input = helpers::read_file("./src/challenges/day-02/input.txt", RangeParser {});
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
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }
            let (left, right) = id_str.split_at(id_str.len() / 2);
            if left == right {
                invalid_count += 1;
                invalid_sum += id;
            }
        }
    });
    Answer {invalid_count, invalid_sum}
}

fn run_hard(input: &Vec<Range>) -> Answer {
    todo!()
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
       let ranges = line.split(",")
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
    use crate::{run_easy, run_hard, Range, RangeParser};
    use aoc_2025::helpers::Parser;

    #[test]
    fn test_sample_input_easy() {
        let raw_input =
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let parser = RangeParser {};
        let input = parser.parse(raw_input).many();
        let result = run_easy(&input);
        assert_eq!(result.invalid_count, 8);
        assert_eq!(result.invalid_sum, 1227775554);
    }

    #[test]
    fn test_sample_input_hard() {
        let raw_input =
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let parser = RangeParser {};
        let input = parser.parse(raw_input).many();
        let result = run_hard(&input);
        assert_eq!(result.invalid_count, 6);
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
    fn test_easy_1() {
        let input = vec![Range { start: 0, end: 15 }];
        let result = run_easy(&input);
        assert_eq!(result.invalid_count, 0);
    }

    #[test]
    fn test_hard_1() {
        let input = vec![Range { start: 0, end: 15 }];
        let result = run_hard(&input);
        assert_eq!(result.invalid_count, 0);
    }
}
