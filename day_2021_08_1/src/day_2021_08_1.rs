extern crate log;

use std::{fs, io};
use std::fs::File;
use std::io::BufRead;

use log::{info, trace};

pub fn main() {
    env_logger::init();

    info!("Starting day...");

    info!("Reading the file");
    let file = File::open("files/day_08.txt").unwrap();
    let reader = io::BufReader::new(file).lines();
    let mut result = 0;
    reader.into_iter().map(|line| {
        let line = line.unwrap();
        trace!("Reading line {}", line);
        let split : Vec<&str> = line.split('|').collect();
        let digits = split[1].split_whitespace().for_each(|digit|{
            if [2,4,3,7].contains(&digit.len()) {
                result += 1;
            }
        });

    }).for_each(drop);

    // Number -> Nb Segments
    // 1 -> 2
    // 4 -> 4
    // 7 -> 3
    // 8 -> 7

    println!("Result : {}", result);

}


struct Display {
    available: [Digit;10],
    lit: [u32;4],
}

struct Digit {
    on: u8,
}


