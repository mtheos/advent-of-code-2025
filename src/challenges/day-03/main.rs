use aoc_2025::helpers::{time_it, Reader};
use std::ops::{Index, Range};

pub const NAME: &str = "Day 03 - Lobby";

fn main() {
    println!("{}", NAME);
    let reader = Reader::from_file("./src/challenges/day-03/input.txt");
    let input = BatteryBankParser {}.parse(reader);
    let (result, duration) = time_it(|| run_easy(&input));
    println!("Easy: {duration:?}");
    println!("Joltage: {}", result.max_joltage);
    let (result, duration) = time_it(|| run_hard(&input));
    println!("Hard: {duration:?}");
    println!("Joltage: {}", result.max_joltage);
}

fn run_easy(input: &Vec<BatteryBank>) -> Answer {
    let mut max_joltage = 0;
    input.iter().for_each(|battery_bank| {
        let max = find_all_the_joltage(battery_bank, 2);
        max_joltage += max;
    });
    Answer { max_joltage }
}

fn run_hard(input: &Vec<BatteryBank>) -> Answer {
    let mut max_joltage = 0;
    input.iter().for_each(|battery_bank| {
        let max = find_all_the_joltage(battery_bank, 12);
        max_joltage += max;
    });
    Answer { max_joltage }
}

fn find_all_the_joltage(battery_bank: &BatteryBank, mut battery_count: u8) -> u64 {
    let mut max_joltage = 0;
    let mut start = 0;
    let max = battery_bank.len();
    while battery_count > 0 {
        let next = start + best_battery(&battery_bank[start..max - (battery_count - 1) as usize]);
        max_joltage += battery_bank[next] as u64 * 10_u64.pow((battery_count - 1) as u32);
        start = next + 1;
        battery_count -= 1;
    }
    max_joltage
}

fn best_battery(batteries: &[u8]) -> usize {
    let mut max = 0;
    let mut idx = 0;
    for i in 0..batteries.len() {
        let next = batteries[i];
        if next > max {
            max = next;
            idx = i;
        }
    }
    idx
}

struct Answer {
    max_joltage: u64,
}

struct BatteryBank {
    batteries: Vec<u8>,
}

impl BatteryBank {
    fn len(&self) -> usize {
        self.batteries.len()
    }
}

impl Index<usize> for BatteryBank {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.batteries[index]
    }
}

impl Index<Range<usize>> for BatteryBank {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.batteries[index]
    }
}

struct BatteryBankParser {}

impl BatteryBankParser {
    fn parse(&self, reader: Reader) -> Vec<BatteryBank> {
        reader
            .map(|line| {
                let batteries = line
                    .chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>();
                BatteryBank { batteries }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::find_all_the_joltage;
    use crate::{run_easy, run_hard, BatteryBank, BatteryBankParser};
    use aoc_2025::helpers::Reader;

    #[test]
    fn test_sample_input_easy() {
        let sample_input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let input = BatteryBankParser {}.parse(Reader::from_vec(sample_input));
        let result = run_easy(&input);
        assert_eq!(result.max_joltage, 357);
    }

    #[test]
    fn test_sample_input_hard() {
        let sample_input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let input = BatteryBankParser {}.parse(Reader::from_vec(sample_input));
        let result = run_hard(&input);
        assert_eq!(result.max_joltage, 3121910778619);
    }

    #[test]
    fn test_parser() {
        let result = BatteryBankParser {}
            .parse(Reader::single("12345"))
            .pop()
            .unwrap();
        for i in 0..5 {
            assert_eq!(result[i], (i + 1) as u8);
        }
    }

    #[test]
    fn test_find_all_the_joltage() {
        let battery_bank = BatteryBank {
            batteries: vec![1, 3, 5, 4, 2],
        };
        let res = find_all_the_joltage(&battery_bank, 2);
        assert_eq!(res, 54);
    }

    #[test]
    fn test_easy_1() {
        let input = vec![BatteryBank {
            batteries: vec![1, 3, 5, 4, 2],
        }];
        let result = run_easy(&input);
        assert_eq!(result.max_joltage, 54);
    }

    #[test]
    fn test_hard_1() {
        let input = vec![BatteryBank {
            batteries: vec![2, 1, 3, 5, 4, 2, 2, 3, 4, 5, 6, 7, 8],
        }];
        let result = run_hard(&input);
        assert_eq!(result.max_joltage, 235422345678);
    }
}
