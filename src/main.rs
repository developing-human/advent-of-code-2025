use std::{process::exit, time::Instant};

use crate::problems::*;

pub mod problems {
    pub mod problem1;
    pub mod problem10;
    pub mod problem11;
    pub mod problem2;
    pub mod problem3;
    pub mod problem4;
    pub mod problem5;
    pub mod problem6;
    pub mod problem7;
    pub mod problem8;
    pub mod problem9;
}

pub mod shared;

fn main() {
    let start = Instant::now();
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
        "3" => println!("{:?}", problem3::solve(&input)),
        "4" => println!("{:?}", problem4::solve(&input)),
        "5" => println!("{:?}", problem5::solve(&input)),
        "6" => println!("{:?}", problem6::solve(&input)),
        "7" => println!("{:?}", problem7::solve(&input)),
        "8" => println!("{:?}", problem8::solve(&input, 1000)),
        "9" => println!("{:?}", problem9::solve(&input)),
        "10" => println!("{:?}", problem10::solve(&input)),
        "11" => println!("{:?}", problem11::solve(&input)),
        _ => {
            eprintln!("ERROR: {first_arg} is not yet implemented");
            exit(1);
        }
    };
    println!("Took: {:?}", start.elapsed());
}
