pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

pub trait Challenge {
    fn preamble(&self) -> String;
    fn run_easy(&mut self) -> String;
    fn run_hard(&mut self) -> String;
}
