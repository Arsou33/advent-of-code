use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::{Captures, Regex};


#[derive(Debug)]
struct SubPosition {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Debug)]
enum Movement {
    Forward,
    Down,
    Up,
}

impl SubPosition {
    fn apply_movement(&mut self, movement: Movement, amount: i32) {
        println!("Move {:?} of {}", movement, amount);
        match movement {
            Movement::Forward => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            },
            Movement::Down => self.aim += amount,
            Movement::Up => self.aim -= amount
        }
    }
}

fn main() -> std::io::Result<()> {

    let file = File::open("day_2.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut current_pos = SubPosition {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    let regexp = Regex::new(r"^(?P<movement>\w+) (?P<amount>\d+)$").unwrap();

    for line in lines {
        let line = line.unwrap();

        let captures =  regexp.captures(line.as_str()).expect("Unable to capture string");
        let movement  = extract_movement(&captures);
        let amount = captures.name("amount").expect("Unable to read amount").as_str();
        current_pos.apply_movement(movement, amount.parse().expect("Unable to parse amount"));
    }

    println!("Result : {:?}", current_pos);
    println!("Code result : {}", current_pos.depth * current_pos.horizontal);

    Ok(())


}

fn extract_movement(captures: &Captures) -> Movement {
    let movement = captures.name("movement").expect("Unable to read movement").as_str();
    match movement {
        "forward" => Movement::Forward,
        "down" => Movement::Down,
        "up" => Movement::Up,
        str=> panic!("Error : {}", str),
    }
}