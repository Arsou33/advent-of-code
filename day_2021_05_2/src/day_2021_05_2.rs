extern crate log;

use std::fmt::Formatter;
use std::fs::File;
use std::{fmt, io};
use std::cmp::max;
use std::collections::HashMap;
use std::io::BufRead;

use log::{debug, error, info, trace};
use regex::Regex;


pub fn main() {
    env_logger::init();

    info!("Starting day...");

    let file = File::open("files/day_05.txt").unwrap();
    let reader = io::BufReader::new(file).lines();

    info!("Reading the file");
    let regexp = Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap();

    let lines : Vec<Line> = reader.into_iter().map(|line| {
        let line = line.unwrap();
        trace!("Reading line {}", line);
        let captures =  regexp.captures(line.as_str()).expect("Unable to capture line");
         Line {
            x1 : captures.name("x1").unwrap().as_str().parse().unwrap(),
            y1 : captures.name("y1").unwrap().as_str().parse().unwrap(),
            x2 : captures.name("x2").unwrap().as_str().parse().unwrap(),
            y2 : captures.name("y2").unwrap().as_str().parse().unwrap(),
        }
    }).collect();

    info!("Calculating diagram");
    let mut diagram = HashMap::new();
    for line in lines {
        for d in 0..=line.length() {
            *diagram.entry((line.x1 + line.dx1()* d, line.y1 + line.dy1()* d)).or_insert(0) += 1;
        }

    }

    info!("Dump diagram");
    for j in 0isize..=9 {
        for i in 0isize..=9 {
            print!("{}", match diagram.get(&(i,j)) {
                Some(v) => (*v).to_string(),
                None => String::from(".")
            });
        }
        println!()
    }

    info!("Determining dangerous zone");
    let result = diagram.iter().filter(|&(_,v)| *v > 1 ).count();

    println!("Result : {}", result);

}

struct Line {
    x1 : isize,
    y1 : isize,
    x2 : isize,
    y2 : isize,
}


impl Line {

    fn dx1(&self) -> isize {
        if self.x2 > self.x1 {
            1
        } else if self.x2 < self.x1 {
            -1
        } else {
            0
        }
    }

    fn dy1(&self) -> isize {
        if self.y2 > self.y1 {
            1
        } else if self.y2 < self.y1 {
            -1
        } else {
            0
        }
    }

    fn length(&self) -> isize {
        max((self.x2 - self.x1).abs(), (self.y2 - self.y1).abs())
    }

}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {} -> {}, {})", self.x1, self.y1, self.x2, self.y2)
    }
}

