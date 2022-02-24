extern crate log;

use std::{io};
use std::fs::File;
use std::io::BufRead;

use log::{info};
use crate::LineStatus::{Corrupted, Incomplete, Valid};
use crate::Mark::{Closing, Opening};
use crate::MarkType::{Braces, Bracket, Parenthesis, Tag};

pub fn main() {
    env_logger::init();

    info!("Starting day...");

    info!("Reading the file");
    let file = File::open("files/day_10.txt").unwrap();
    let reader = io::BufReader::new(file).lines();

    let mut result = 0;
    reader.into_iter().for_each(|line| {
        let line = line.unwrap();
        let mut line_iter = line.chars().into_iter();
        let mut status = None;
        let mut opened = Vec::new();

        while status.is_none() {
            status = if let Some(c) = line_iter.next() {
                let mark = Mark::from(c);
                match mark {
                    Opening(mar_type) => {
                        opened.push(mar_type);
                        None
                    }
                    Closing(mark_type) => {
                        match opened.pop() {
                            None => Some(Corrupted(Closing(mark_type))), // Closing tag out
                            Some(expected_mark_type) => if expected_mark_type == mark_type {
                                None
                            } else {
                                Some(Corrupted(Closing(mark_type)))
                            }
                        }
                    }
                }
            } else {
                Some(Incomplete)
            }
        }
        let status = status.expect("No status ???");
        info!("Line : {} -> {:?}", line, status);
        if let Corrupted(Closing(mark)) = status {
            result += mark as u32;

        }
    });


    println!("Result : {:?}", result);

}

#[derive(Debug)]
enum LineStatus {
    Corrupted(Mark),
    Incomplete,
    Valid
}

#[derive(Debug, PartialEq)]
enum MarkType {
    Parenthesis = 3,
    Bracket = 57,
    Braces = 1197,
    Tag = 25137
}

#[derive(Debug)]
enum Mark {
    Opening(MarkType),
    Closing(MarkType)
}


impl Mark {
    fn from(char: char) -> Mark {
        match char {
            '(' => Opening(Parenthesis),
            ')' => Closing(Parenthesis),
            '[' => Opening(Bracket),
            ']' => Closing(Bracket),
            '{' => Opening(Braces),
            '}' => Closing(Braces),
            '<' => Opening(Tag),
            '>' => Closing(Tag),
            _ => panic!("Unknown char mark {}", char)
        }
    }
}





