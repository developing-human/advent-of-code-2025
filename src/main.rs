use std::process::exit;

use crate::problems::*;

pub mod problems {
    pub mod problem1;
    pub mod problem2;
}

fn main() {
    let first_arg: String = std::env::args().nth(1).expect("problem number is required");
    let filename = format!("inputs/{}.txt", first_arg);
    let input = std::fs::read_to_string(&filename).unwrap_or_else(|_| {
        eprintln!("file does not exist: {filename}");
        exit(1);
    });

    let output = match first_arg.as_str() {
        "1" => problem1::solve(&input),
        "2" => problem2::solve(&input),
        _ => {
            eprintln!("{first_arg} is not yet implemented");
            exit(1);
        }
    };

    println!("{output:?}");
}
