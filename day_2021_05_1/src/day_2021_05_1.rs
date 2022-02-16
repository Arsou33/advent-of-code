extern crate log;

use std::fmt::Formatter;
use std::fs::File;
use std::{fmt, io};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::BufRead;

use log::{debug, error, info, Level, log_enabled, trace};
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
        }.normalize()
    }).collect();

    info!("Calculating diagram");
    let mut diagram = HashMap::new();
    for line in lines {
        if line.is_horizontal() {
            for i in line.x1..=line.x2 {
                *diagram.entry((i, line.y1)).or_insert(0) += 1;
            }
        } else if line.is_vertical() {
            for i in line.y1..=line.y2 {
                *diagram.entry((line.x1, i)).or_insert(0) += 1;
            }
        }
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
    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }
    fn normalize(&self) -> Line {
        Line {
            x1 : min(self.x1, self.x2),
            x2 : max(self.x1, self.x2),
            y1 : min(self.y1, self.y2),
            y2 : max(self.y1, self.y2),
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {} -> {}, {})", self.x1, self.y1, self.x2, self.y2)
    }
}

