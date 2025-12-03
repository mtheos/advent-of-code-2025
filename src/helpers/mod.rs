use std::fs::read_to_string;

pub trait Parser<T> {
    fn parse(&self, line: &str) -> T;
}

pub fn read_file<T, P>(file_path: &str, parser: P) -> Vec<T>
where
    P: Parser<T>,
{
    let mut result = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        result.push(parser.parse(line));
    }
    result
}
