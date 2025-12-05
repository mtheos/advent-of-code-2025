use aoc_2025::helpers::{time_it, Reader};
use std::cmp::{max, min};

pub const NAME: &str = "Day 05 - Cafeteria";
pub const PREFIX: &str = "./src/challenges/day-05";

fn main() {
    let (_, duration) = time_it(||do_it());
    println!("total: {duration:?}");
}

fn do_it() {
println!("{}", NAME);
    let mut reader = Reader::from_file(format!("{PREFIX}/input.txt").as_str());
    let range_parser = RangeParser {};
    let item_parser = ItemParser {};
    let input = CombiParser {
        range_parser,
        item_parser,
    }
    .parse(&mut reader);
    let (result, duration) = time_it(|| run_easy(&input));
    println!("Easy: {duration:?}");
    println!("Unspoiled: {}", result.unspoiled_food);
    println!("Total: {}", result.total_unspoiled_foods);
    let (result, duration) = time_it(|| run_hard(&input));
    println!("Hard: {duration:?}");
    println!("Unspoiled: {}", result.unspoiled_food);
    println!("Total: {}", result.total_unspoiled_foods);
}

fn run_easy(database: &Database) -> Answer {
    let mut unspoiled_food = 0;
    for item in &database.items {
        for range in &database.ranges {
            if range.start <= *item && range.end >= *item {
                unspoiled_food += 1;
                break;
            }
        }
    }
    Answer {
        unspoiled_food,
        total_unspoiled_foods: 0,
    }
}

fn run_hard(database: &Database) -> Answer {
    let mut total_unspoiled_foods = 0;
    for range in &database.ranges {
        total_unspoiled_foods += range.end - range.start + 1;
    }
    Answer {
        unspoiled_food: 0,
        total_unspoiled_foods,
    }
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut merged_ranges: Vec<Range> = Vec::new();
    let mut i = 0;
    let mut j: usize;
    loop {
        if i == ranges.len() {
            break;
        }
        let pushed;
        let mut left = ranges[i];
        j = i + 1;
        loop {
            if j == ranges.len() {
                pushed = false;
                break;
            }
            let right = ranges[j];
            if left.can_merge(&right) {
                left = left.merge(&right);
                i = j;
            } else {
                pushed = true;
                merged_ranges.push(left);
                break;
            }
            j += 1
        }
        if !pushed {
            merged_ranges.push(left);
        }
        i += 1;
    }
    merged_ranges
}

struct Answer {
    unspoiled_food: u64,
    total_unspoiled_foods: u64,
}

#[derive(Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn can_merge(&self, other: &Range) -> bool {
        if self.start > other.start && self.start > other.end {
            false
        } else if self.end < other.end && self.end < other.start {
            false
        } else {
            true
        }
    }

    fn merge(&self, other: &Range) -> Self {
        if !self.can_merge(other) {
            panic!("Unexpected case!");
        }
        let start = min(self.start, other.start);
        let end = max(self.end, other.end);
        Range { start, end }
    }
}

struct Database {
    ranges: Vec<Range>,
    items: Vec<u64>,
}

struct RangeParser {}

struct ItemParser {}

struct CombiParser {
    range_parser: RangeParser,
    item_parser: ItemParser,
}

impl RangeParser {
    fn parse(&self, reader: &mut Reader) -> Vec<Range> {
        let mut ranges = reader
            .take_while(|line| line != "")
            .map(|line| {
                let (start, end) = line.split_once("-").unwrap();
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                Range { start, end }
            })
            .collect::<Vec<Range>>();
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        ranges
    }
}

impl ItemParser {
    fn parse(&self, reader: &mut Reader) -> Vec<u64> {
        reader
            .take_while(|line| line != "")
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }
}

impl CombiParser {
    fn parse(&self, reader: &mut Reader) -> Database {
        let ranges = self.range_parser.parse(reader);
        let items = self.item_parser.parse(reader);
        let ranges = merge_ranges(ranges);
        Database { ranges, items }
    }
}

#[cfg(test)]
mod tests {
    use crate::{run_easy, run_hard, CombiParser, Range, PREFIX};
    use crate::{ItemParser, RangeParser};
    use aoc_2025::helpers::Reader;

    #[test]
    fn test_sample_input_easy() {
        let range_parser = RangeParser {};
        let item_parser = ItemParser {};
        let input = CombiParser {
            range_parser,
            item_parser,
        }
        .parse(&mut Reader::from_file(
            format!("{PREFIX}/sample.txt").as_str(),
        ));
        let result = run_easy(&input);
        assert_eq!(result.unspoiled_food, 3);
    }

    #[test]
    fn test_sample_input_hard() {
        let range_parser = RangeParser {};
        let item_parser = ItemParser {};
        let input = CombiParser {
            range_parser,
            item_parser,
        }
        .parse(&mut Reader::from_file(
            format!("{PREFIX}/sample.txt").as_str(),
        ));
        let result = run_hard(&input);
        assert_eq!(result.total_unspoiled_foods, 14);
    }

    #[test]
    fn test_range_parser() {
        let result = RangeParser {}.parse(&mut Reader::from_vec(vec!["11-15", "35-41"]));
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].start, 11);
        assert_eq!(result[0].end, 15);
    }

    #[test]
    fn test_item_parser() {
        let result = ItemParser {}.parse(&mut Reader::from_vec(vec!["1", "3"]));
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 3);
    }

    #[test]
    fn test_combined_parser() {
        let mut reader = Reader::from_vec(vec!["11-15", "35-41", "", "1", "3"]);
        let ranges = RangeParser {}.parse(&mut reader);
        let items = ItemParser {}.parse(&mut reader);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].start, 11);
        assert_eq!(ranges[0].end, 15);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], 1);
        assert_eq!(items[1], 3);
    }

    #[test]
    #[should_panic]
    fn test_merge_does_not_merge_distinct_higher() {
        let r1 = &Range { start: 10, end: 20 };
        let r2 = &Range { start: 25, end: 30 };
        r1.merge(r2);
    }

    #[test]
    #[should_panic]
    fn test_merge_does_not_merge_distinct_lower() {
        let r1 = &Range { start: 10, end: 20 };
        let r2 = &Range { start: 25, end: 30 };
        r2.merge(r1);
    }

    #[test]
    fn test_merge_overlapping_lower() {
        let r1 = &Range { start: 10, end: 20 };
        let r2 = &Range { start: 5, end: 15 };
        let merged = r2.merge(r1);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.end, 20);
    }

    #[test]
    fn test_merge_overlapping_higher() {
        let r1 = &Range { start: 10, end: 20 };
        let r2 = &Range { start: 15, end: 25 };
        let merged = r2.merge(r1);
        assert_eq!(merged.start, 10);
        assert_eq!(merged.end, 25);
    }

    #[test]
    fn test_merge_contained() {
        let r1 = &Range { start: 10, end: 20 };
        let r2 = &Range { start: 12, end: 18 };
        let merged = r2.merge(r1);
        assert_eq!(merged.start, 10);
        assert_eq!(merged.end, 20);
    }

    #[test]
    fn test_merge_encompassing() {
        let r1 = &Range { start: 10, end: 20 };
        let r2 = &Range { start: 5, end: 25 };
        let merged = r2.merge(r1);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.end, 25);
    }
}
