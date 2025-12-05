use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

pub struct Reader {
    iter: RefCell<Box<dyn Iterator<Item = String>>>,
}

impl Reader {
    pub fn from_vec(lines: Vec<&str>) -> Self {
        Self {
            iter: RefCell::new(Box::new(
                lines
                    .into_iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>()
                    .into_iter(),
            )),
        }
    }

    pub fn single(line: &str) -> Self {
        Self {
            iter: RefCell::new(Box::new(vec![line.to_owned()].into_iter())),
        }
    }

    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file.try_clone().unwrap());
        let iter = reader.lines().map(|l| l.unwrap());
        let iter = RefCell::new(Box::new(iter));
        Self { iter }
    }
}

impl Iterator for Reader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.borrow_mut().next()
    }
}

pub fn time_it<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}
