use std::process::exit;

use crate::problems::*;

pub mod problems {
    pub mod problem1;
    pub mod problem2;
}

pub mod shared;

fn main() {
    let first_arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("ERROR: problem number is required");
        exit(1);
    });

    let filename = format!("inputs/{}.txt", first_arg);
    let input = std::fs::read_to_string(&filename).unwrap_or_else(|_| {
        eprintln!("ERROR: file does not exist: {filename}");
        exit(1);
    });

    match first_arg.as_str() {
        "1" => println!("{:?}", problem1::solve(&input)),
        "2" => println!("{:?}", problem2::solve(&input)),
        _ => {
            eprintln!("ERROR: {first_arg} is not yet implemented");
            exit(1);
        }
    };
}
