use std::fs::read_to_string;

pub trait Parser<T> {
    fn parse(&self, line: &str) -> Parsed<T>;
}

pub enum Parsed<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Parsed<T> {
    pub fn is_one(&self) -> bool {
        matches!(self, Self::One(_))
    }
    pub fn one(self) -> T {
        match self {
            Parsed::One(res) => res,
            Parsed::Many(_) => panic!("I feel violated"),
        }
    }

    pub fn many(self) -> Vec<T> {
        match self {
            Parsed::One(_) => panic!("I feel violated"),
            Parsed::Many(res) => res,
        }
    }
}

pub fn read_file<T, P>(file_path: &str, parser: P) -> Vec<T>
where
    P: Parser<T>,
{
    let mut result = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        match parser.parse(line) {
            Parsed::One(res) => result.push(res),
            Parsed::Many(res) => result.extend(res),
        }
    }
    result
}
