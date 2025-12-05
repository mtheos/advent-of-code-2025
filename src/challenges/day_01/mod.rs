use crate::helpers::{Reader, PREFIX};
use crate::Challenge;

const NAME: &str = "Secret Entrance";
const DAY: &str = "01";

pub struct State {
    input: Vec<Sequence>,
    dial_limit: i32,
    dial_position: i32,
}

impl State {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        let reader = Reader::from_file(format!("{PREFIX}_{DAY}/input.txt").as_str());
        let input = DialParser { dial_limit: 99 }.parse(reader);
        State {
            input,
            dial_limit: 99,
            dial_position: 50,
        }
    }
}

impl Challenge for State {
    fn preamble(&self) -> String {
        format!("Day {DAY} - {NAME}")
    }
    fn run_easy(&mut self) -> String {
        let Answer { zero_count, .. } = do_easy(self);
        format!("Zeros: {zero_count}")
    }

    fn run_hard(&mut self) -> String {
        let Answer { zero_count, .. } = do_hard(self);
        format!("Zeros: {zero_count}")
    }
}

fn do_easy(state: &State) -> Answer {
    let mut zero_count = 0;
    let dial_limit = state.dial_limit + 1;
    let mut dial_position = state.dial_position;
    state.input.iter().for_each(|sequence| {
        dial_position += sequence.magnitude;
        dial_position = dial_position.rem_euclid(dial_limit);
        if dial_position == 0 {
            zero_count += 1;
        }
    });
    Answer {
        zero_count,
    }
}

fn do_hard(state: &State) -> Answer {
    let mut zero_count = 0;
    let dial_limit = state.dial_limit + 1;
    let mut dial_position = state.dial_position;
    state.input.iter().for_each(|sequence| {
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
        zero_count,
    }
}

struct Answer {
    zero_count: i32,
}

struct Sequence {
    rollovers: i32,
    magnitude: i32,
}

struct DialParser {
    dial_limit: i32,
}

impl DialParser {
    fn parse(&self, reader: Reader) -> Vec<Sequence> {
        reader
            .map(|line| {
                let rollover_value = self.dial_limit + 1;
                let value = line[1..].parse::<i32>().unwrap();
                let (magnitude, rollovers) = if value > self.dial_limit {
                    (
                        value.rem_euclid(rollover_value),
                        value.div_euclid(rollover_value),
                    )
                } else {
                    (value, 0)
                };
                let magnitude = if line.starts_with('L') {
                    -magnitude
                } else {
                    magnitude
                };
                Sequence {
                    rollovers,
                    magnitude,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_01::{do_easy, do_hard, DialParser, Sequence, State, DAY};
    use crate::helpers::{PREFIX, Reader};

    #[test]
    fn test_sample_input_easy() {
        let input = DialParser { dial_limit: 99 }.parse(Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_easy(&mut state);
        assert_eq!(result.zero_count, 3);
    }

    #[test]
    fn test_sample_input_hard() {
        let input = DialParser { dial_limit: 99 }.parse(Reader::from_file(
            format!("{PREFIX}_{DAY}/sample.txt").as_str(),
        ));
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_hard(&mut state);
        assert_eq!(result.zero_count, 6);
    }

    #[test]
    fn test_parser() {
        let parser = DialParser { dial_limit: 99 };
        let result = parser.parse(Reader::single("L41")).pop().unwrap();
        assert_eq!(result.magnitude, -41);
        assert_eq!(result.rollovers, 0);
        let result = parser.parse(Reader::single("R12")).pop().unwrap();
        assert_eq!(result.magnitude, 12);
        assert_eq!(result.rollovers, 0);
        let result = parser.parse(Reader::single("R115")).pop().unwrap();
        assert_eq!(result.magnitude, 15);
        assert_eq!(result.rollovers, 1);
        let result = parser.parse(Reader::single("R200")).pop().unwrap();
        assert_eq!(result.magnitude, 0);
        assert_eq!(result.rollovers, 2);
    }

    #[test]
    fn test_easy_1() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: 15,
        }];
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_easy(&mut state);
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
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_easy(&mut state);
        assert_eq!(result.zero_count, 1);
    }

    #[test]
    fn test_easy_3() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: -60,
        }];
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_easy(&mut state);
        assert_eq!(result.zero_count, 0);
    }

    #[test]
    fn test_hard_1() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: 15,
        }];
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_hard(&mut state);
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
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_hard(&mut state);
        assert_eq!(result.zero_count, 1);
    }

    #[test]
    fn test_hard_3() {
        let input = vec![Sequence {
            rollovers: 0,
            magnitude: -60,
        }];
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_hard(&mut state);
        assert_eq!(result.zero_count, 1);
    }

    #[test]
    fn test_hard_4() {
        let input = DialParser { dial_limit: 99 }.parse(Reader::from_vec(vec!["R551", "L10"]));
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_hard(&mut state);
        assert_eq!(result.zero_count, 7);
    }

    #[test]
    fn test_hard_5() {
        let input = DialParser { dial_limit: 99 }.parse(Reader::from_vec(vec!["L50", "L10"]));
        let mut state = State {
            input,
            dial_limit: 99,
            dial_position: 50,
        };
        let result = do_hard(&mut state);
        assert_eq!(result.zero_count, 1);
    }
}
