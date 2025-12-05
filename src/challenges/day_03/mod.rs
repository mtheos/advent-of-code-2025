use crate::challenges::Challenge;
use crate::helpers::{Reader, PREFIX};
use std::ops::{Index, Range};

const NAME: &str = "Lobby";
const DAY: &str = "03";

pub struct State {
    input: Vec<BatteryBank>,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = BatteryBankParser {}.parse(reader);
        State { input }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }

    fn run_easy(&mut self) -> String {
        let Answer { max_joltage } = sum_joltage(&self, 2);
        format!("Max Joltage: {max_joltage}")
    }

    fn run_hard(&mut self) -> String {
        let Answer { max_joltage } = sum_joltage(&self, 12);
        format!("Max Joltage: {max_joltage}")
    }
}

fn sum_joltage(state: &State, batteries_per_bank: u8) -> Answer {
    let mut max_joltage = 0;
    state.input.iter().for_each(|battery_bank| {
        let max = find_all_the_joltage(battery_bank, batteries_per_bank);
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
    use crate::challenges::day_03::{
        find_all_the_joltage, sum_joltage, BatteryBank, BatteryBankParser, State, DAY,
    };
    use crate::helpers::{Reader, PREFIX};

    #[test]
    fn test_sample_input_easy() {
        let input =
            BatteryBankParser {}.parse(Reader::from_file(format!("{PREFIX}_{DAY}/sample.txt").as_str()));
        let state = State { input };
        let result = sum_joltage(&state, 2);
        assert_eq!(result.max_joltage, 357);
    }

    #[test]
    fn test_sample_input_hard() {
        let input =
            BatteryBankParser {}.parse(Reader::from_file(format!("{PREFIX}_{DAY}/sample.txt").as_str()));
        let state = State { input };
        let result = sum_joltage(&state, 12);
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
        let state = State { input };
        let result = sum_joltage(&state, 2);
        assert_eq!(result.max_joltage, 54);
    }

    #[test]
    fn test_hard_1() {
        let input = vec![BatteryBank {
            batteries: vec![2, 1, 3, 5, 4, 2, 2, 3, 4, 5, 6, 7, 8],
        }];
        let state = State { input };
        let result = sum_joltage(&state, 12);
        assert_eq!(result.max_joltage, 235422345678);
    }
}
