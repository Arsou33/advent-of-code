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
            .map(|c| {
                let v = c.to_digit(10).unwrap();
                (v, v == 9)
            })
            .collect::<Vec<(u32,bool)>>();
        trace!("Reading line {:?}", line);
        line.insert(0, (9, true));
        line.push((9,true));
        line
    }).collect::<Vec<Vec<(u32,bool)>>>();

    let line_length = heightmap[0].len();
    heightmap.insert(0, vec![(9,true);line_length]);
    heightmap.push(vec![(9,true);line_length]);

    let mut bassins = Vec::new();
    for j in 1..line_length {
        for i in 1..heightmap.len() {
            let new_bassins = visit(&mut heightmap, i, j);
            if new_bassins > 0 {
                bassins.push(new_bassins);
            }
        }
    }

    bassins.sort();
    let result = bassins.iter().rev().take(3).fold(1, |acc, x| acc * x);

    println!("Result : {:?}", result);

}


fn visit(heightmap : &mut Vec<Vec<(u32, bool)>>, i: usize, j: usize) -> u32 {
    let current = heightmap[i][j];
    if current.1 {
        0
    } else {
        heightmap[i][j].1 = true;
        1
        + visit(heightmap, i-1, j)
        + visit(heightmap, i+1, j)
        + visit(heightmap, i, j-1)
        + visit(heightmap, i, j+1)
    }
}





