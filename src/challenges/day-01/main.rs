use aoc_2025::helpers::{Parsed, Parser, read_file};

pub const NAME: &str = "Secret Entrance";

fn main() {
    let input = read_file(
        "./src/challenges/day-01/input.txt",
        DialParser { dial_limit: 99 },
    );
    let result = run_easy(&input, 50, 99);
    println!("Pos  : {}", result.dial_position);
    println!("Count: {}", result.zero_count);
    let result = run_hard(&input, 50, 99);
    println!("Pos  : {}", result.dial_position);
    println!("Count: {}", result.zero_count);
}

fn run_easy(input: &Vec<Sequence>, mut dial_position: i32, dial_limit: i32) -> Answer {
    let mut zero_count = 0;
    let dial_limit = dial_limit + 1;
    input.iter().for_each(|sequence| {
        dial_position += sequence.magnitude;
        dial_position = dial_position.rem_euclid(dial_limit);
        if dial_position == 0 {
            zero_count += 1;
        }
    });
    Answer {
        dial_position,
        zero_count,
    }
}

fn run_hard(input: &Vec<Sequence>, mut dial_position: i32, dial_limit: i32) -> Answer {
    let mut zero_count = 0;
    let dial_limit = dial_limit + 1;
    input.iter().for_each(|sequence| {
        zero_count += sequence.rollovers;
        let dial_was_zero = dial_position == 0;
        dial_position += sequence.magnitude;
        let rolled_over = dial_position >= dial_limit || dial_position <= 0 && !dial_was_zero;
        dial_position = dial_position.rem_euclid(dial_limit);
        if rolled_over {
            zero_count += 1;
        }
    });
    Answer {
        dial_position,
        zero_count,
    }
}

struct Answer {
    dial_position: i32,
    zero_count: i32,
}

struct Sequence {
    rollovers: i32,
    magnitude: i32,
}

struct DialParser {
    dial_limit: i32,
}

impl Parser<Sequence> for DialParser {
    fn parse(&self, line: &str) -> Parsed<Sequence> {
        let rollover_value = self.dial_limit + 1;
        let value = line[1..].parse::<i32>().unwrap();
        let magnitude;
        let rollovers;
        if value > self.dial_limit {
            rollovers = value.div_euclid(rollover_value);
            magnitude = value.rem_euclid(rollover_value);
        } else {
            rollovers = 0;
            magnitude = value;
        }
        Parsed::One(Sequence {
          rollovers,
          magnitude: if line.starts_with('L') {
            -magnitude
          } else {
            magnitude
          },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{run_easy, run_hard, DialParser, Sequence};
    use aoc_2025::helpers::Parser;

    #[test]
    fn test_sample_input_easy() {
        let raw_input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let parser = DialParser { dial_limit: 99 };
        let input = raw_input
            .iter()
            .map(|x| parser.parse(x).one())
            .collect::<Vec<Sequence>>();
        let result = run_easy(&input, 50, 99);
        assert_eq!(result.dial_position, 32);
        assert_eq!(result.zero_count, 3);
    }

    #[test]
    fn test_sample_input_hard() {
        let raw_input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let parser = DialParser { dial_limit: 99 };
        let input = raw_input
            .iter()
            .map(|x| parser.parse(x).one())
            .collect::<Vec<Sequence>>();
        let result = run_hard(&input, 50, 99);
        assert_eq!(result.dial_position, 32);
        assert_eq!(result.zero_count, 6);
    }

    #[test]
    fn test_parser() {
        let parser = DialParser { dial_limit: 99 };
        let result = parser.parse("L41").one();
        assert_eq!(result.magnitude, -41);
        assert_eq!(result.rollovers, 0);
        let result = parser.parse("R12").one();
        assert_eq!(result.magnitude, 12);
        assert_eq!(result.rollovers, 0);
        let result = parser.parse("R115").one();
        assert_eq!(result.magnitude, 15);
        assert_eq!(result.rollovers, 1);
        let result = parser.parse("R200").one();
        assert_eq!(result.magnitude, 0);
        assert_eq!(result.rollovers, 2);
    }

    #[test]
    fn test_easy_1() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: 15,
        }];
        let result = run_easy(&input, 50, 99);
        assert_eq!(result.dial_position, 65);
        assert_eq!(result.zero_count, 0);
    }

    #[test]
    fn test_easy_2() {
        let input = vec![
            Sequence {
                rollovers: 0,
                magnitude: 15,
            },
            Sequence {
                rollovers: 0,
                magnitude: 35,
            },
            Sequence {
                rollovers: 0,
                magnitude: 40,
            },
        ];
        let result = run_easy(&input, 50, 99);
        assert_eq!(result.dial_position, 40);
        assert_eq!(result.zero_count, 1);
    }

    #[test]
    fn test_easy_3() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: -60,
        }];
        let result = run_easy(&input, 50, 99);
        assert_eq!(result.dial_position, 90);
        assert_eq!(result.zero_count, 0);
    }

    #[test]
    fn test_hard_1() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: 15,
        }];
        let result = run_hard(&input, 50, 99);
        assert_eq!(result.dial_position, 65);
        assert_eq!(result.zero_count, 0);
    }

    #[test]
    fn test_hard_2() {
        let input = vec![
            Sequence {
                rollovers: 0,
                magnitude: 15,
            },
            Sequence {
                rollovers: 0,
                magnitude: 30,
            },
            Sequence {
                rollovers: 0,
                magnitude: 45,
            },
        ];
        let result = run_hard(&input, 50, 99);
        assert_eq!(result.dial_position, 40);
        assert_eq!(result.zero_count, 1);
    }

    #[test]
    fn test_hard_3() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: -60,
        }];
        let result = run_hard(&input, 50, 99);
        assert_eq!(result.dial_position, 90);
        assert_eq!(result.zero_count, 1);
    }

    #[test]
    fn test_hard_4() {
        let parser = DialParser { dial_limit: 99 };
        let input = vec![parser.parse("R551").one(), parser.parse("L10").one()];
        let result = run_hard(&input, 50, 99);
        assert_eq!(result.dial_position, 91);
        assert_eq!(result.zero_count, 7);
    }

    #[test]
    fn test_hard_5() {
        let parser = DialParser { dial_limit: 99 };
        let input = vec![parser.parse("L50").one(), parser.parse("L10").one()];
        let result = run_hard(&input, 50, 99);
        assert_eq!(result.dial_position, 90);
        assert_eq!(result.zero_count, 1);
    }
}
