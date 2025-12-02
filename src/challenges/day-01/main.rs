use aoc_2025::helpers;
use aoc_2025::helpers::Parser;

pub const NAME: &str = "Secret Entrance";

fn main() {
    let input = helpers::read_file("./src/challenges/day-01/input.txt", DialParser{ dial_limit: 99});
    run_easy(&input, 50, 99);
    run_hard(&input, 50, 99);
}

fn run_easy(input: &Vec<Sequence>, mut dial_position: i32, dial_limit: i32) {
    let mut zero_count = 0;
    let rollover = dial_limit + 1;
    input.iter().for_each(|sequence| {
        dial_position += sequence.magnitude;
        dial_position = dial_position.rem_euclid(rollover);
        if dial_position == 0 {
            zero_count += 1;
        }
    });
    println!("{}", dial_position);
    println!("{}", zero_count);
}

fn run_hard(input: &Vec<Sequence>, mut dial_position: i32, dial_limit: i32) {
    let mut zero_count = 0;
    let rollover = dial_limit + 1;
    input.iter().for_each(|sequence| {
        dial_position += sequence.magnitude;
        let prev = dial_position;
        dial_position = dial_position.rem_euclid(rollover);
        zero_count += sequence.rollovers;
        if prev != dial_position {
            zero_count += 1;
        }
    });
    println!("{}", dial_position);
    println!("{}", zero_count);
}

struct Sequence {
    rollovers: i32,
    magnitude: i32,
}

struct DialParser {
    dial_limit: i32,
}

impl Parser<Sequence> for DialParser {
    fn parse(&self, line: &str) -> Sequence {
        let rollover_value = self.dial_limit + 1;
        let value = line[1..].parse::<i32>().unwrap();
        let magnitude;
        let rollovers;
        if value > self.dial_limit {
            rollovers = value / rollover_value;
            magnitude = value % rollover_value;
        } else {
            rollovers = 0;
            magnitude = value;
        }
        Sequence {
            rollovers,
            magnitude: if line.starts_with('L') {
                -magnitude
            } else {
                magnitude
            },
        }
    }
}
