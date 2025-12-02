use std::fs::{read_to_string};

pub const VERSION: &str = "0.0.1";

pub fn read_file<T, F>(file_path: &str, transformer: F) -> Vec<T>
where F: Fn(&str) -> T {
  let mut result = Vec::new();
  for line in read_to_string(file_path).unwrap().lines() {
    result.push(transformer(line));
  }
  result
}
