extern crate log;

use std::{io};
use std::fs::File;
use std::io::BufRead;

use log::{debug, info, trace};

pub fn main() {
    env_logger::init();

    info!("Starting day...");

    info!("Reading the file");
    let file = File::open("files/day_09.txt").unwrap();
    let reader = io::BufReader::new(file).lines();
    let mut heightmap = reader.into_iter().map(|line| {
        let mut line = line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        trace!("Reading line {:?}", line);
        line.insert(0, 10);
        line.push(10);
        line
    }).collect::<Vec<Vec<u32>>>();

    let line_length = heightmap[0].len();
    heightmap.insert(0, vec![10;line_length]);
    heightmap.push(vec![10;line_length]);

    let mut result = 0;
    for j in 1..line_length {
        for i in 1..heightmap.len() {
            let current = heightmap[i][j];
            if current < heightmap[i-1][j]
                && current < heightmap[i+1][j]
                && current < heightmap[i][j-1]
                && current < heightmap[i][j+1]
            {
                result += 1 + current;
            }
        }
    }


    println!("Result : {}", result);

}




