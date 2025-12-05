mod challenges;
mod helpers;

use std::{env, process};
use crate::challenges::day_01;
use crate::challenges::day_02;
use crate::challenges::day_03;
use crate::challenges::day_04;
use crate::challenges::day_05;
use crate::challenges::Challenge;
use crate::helpers::time_it;

fn main() {
    let mut challenges: Vec<fn() -> Box<dyn Challenge>> = Vec::new();
    challenges.push(|| Box::new(day_01::State::new()));
    challenges.push(|| Box::new(day_02::State::new()));
    challenges.push(|| Box::new(day_03::State::new()));
    challenges.push(|| Box::new(day_04::State::new()));
    challenges.push(|| Box::new(day_05::State::new()));

    let arg = env::args().nth(1).or(Some("0".to_string())).unwrap();
    let day = arg.parse::<usize>().unwrap();

    match day {
        0 => challenges.iter_mut().for_each(|ctor| {
            run_and_print_challenge(ctor);
        }),
        _ => {
            if day - 1 >= challenges.len() {
                eprintln!("There are only {} challenges!", challenges.len());
                process::exit(1);
            }
            let mut ctor = challenges[day - 1];
            run_and_print_challenge(&mut ctor);
        }
    }
}

fn run_and_print_challenge(ctor: &mut fn() -> Box<dyn Challenge>) {
    let (mut c, duration) = time_it(|| ctor());
    let mut lines: Vec<String> = Vec::new();

    lines.push(format!("|| {}", c.preamble()));
    lines.push(format!("|| Construction: {:?}", duration));
    lines.push("||".to_string());

    let (res, duration) = time_it(|| c.run_easy());
    lines.push(format!("|| Easy: {:?}", duration));
    lines.push(format!("|| {res}"));
    lines.push("||".to_string());

    let (res, duration) = time_it(|| c.run_hard());
    lines.push(format!("|| Hard: {:?}", duration));
    lines.push(format!("|| {res}"));

    pretty_print(lines);
}

fn pretty_print(lines: Vec<String>) {
    let longest = lines.iter().map(|x| x.len()).max().unwrap() + 2;
    let frame = "=".repeat(longest + 2);
    println!("{frame}");
    for line in lines {
        print!("{line}");
        let padding = longest - line.chars().count();
        if padding > 0 {
            let pad = " ".repeat(padding);
            print!("{pad}");
        }
        println!("||");
    }
    println!("{frame}");
    println!();
}
