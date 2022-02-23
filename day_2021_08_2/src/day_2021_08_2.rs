extern crate log;

use std::{io};
use std::fs::File;
use std::io::BufRead;

use log::{debug, info, trace};

pub fn main() {
    env_logger::init();

    info!("Starting day...");

    info!("Reading the file");
    let file = File::open("files/day_08.txt").unwrap();
    let reader = io::BufReader::new(file).lines();
    let mut result = 0;
    reader.into_iter().for_each(|line| {
        let line = line.unwrap();
        trace!("Reading line {}", line);
        let split: Vec<&str> = line.split('|').collect();
        let mut unknown_digits: Vec<Vec<char>> = split[0].split_whitespace().map(|d| {
            let mut segments: Vec<char>  = d.chars().collect();
            segments.sort();
            segments
        }).collect();
        println!("Digit: {:?}", unknown_digits);

        let mut digits : [Vec<char>;10] = Default::default();

        debug!("Step 1 : Determining digit based on length");

            let mut i = 0;
            while i < unknown_digits.len() {
                let unknown_digit_len = unknown_digits[i].len();
                match unknown_digit_len {
                    2 => digits[1] = unknown_digits.remove(i),
                    3 => digits[7] = unknown_digits.remove(i),
                    4 => digits[4] = unknown_digits.remove(i),
                    7 => digits[8] = unknown_digits.remove(i),
                    _ => i += 1
                }
            }

        debug!("Step 2 : Apply rules");
        {

            let mut digit4_1 = Vec::new(); // Segments in 4 but not in 1
            for &segment in digits[4].iter() {
                if !digits[1].contains(&segment) {
                    digit4_1.push(segment);
                }
            }


            let mut i = 0;
            while i < unknown_digits.len() {
                let unknown_digit = &unknown_digits[i];
                match unknown_digit.len() {
                    5 => {
                        let digit1 = &digits[1];
                        if unknown_digit.contains(&digit1[0]) && unknown_digit.contains(&digit1[1]) {
                            digits[3] = unknown_digits.remove(i);
                        } else if unknown_digit.contains(&digit4_1[0]) && unknown_digit.contains(&digit4_1[1]) {
                            digits[5] = unknown_digits.remove(i);
                        } else {
                            digits[2] = unknown_digits.remove(i);
                        }
                    }
                    6 => {
                        let digit1 = &digits[1];
                        if !unknown_digit.contains(&digit1[0]) || !unknown_digit.contains(&digit1[1]) {
                            digits[6] = unknown_digits.remove(i);
                        } else if digits[4].iter().all(|item| unknown_digit.contains(item)) {
                            digits[9] = unknown_digits.remove(i);
                        } else {
                            digits[0] = unknown_digits.remove(i);
                        }
                    }
                    _ => i += 1
                }
            }
        }

        info!("Line :{:?} ", digits);

        debug!("Extract display");
        let display = split[1].split_whitespace().map(|d| {
            let mut segments: Vec<char>  = d.chars().collect();
            segments.sort();
            segments
        }).map(|segments| {
            digits.iter().position(|s| s == &segments).unwrap()
        }).collect::<Vec<usize>>();


        result += display.iter().fold(0, |acc, x| acc*10+x);




    });

    // Number -> Segments
    // 1 ->     c     f   -> 2 : Length 2
    // 7 -> a   c     f   -> 3 : Length 3
    // 4 ->   b c d   f   -> 4 : Length 4
    // 8 -> a b c d e f g -> 7 : Length 7
    // 3 -> a   c d   f g -> 5 : Length 5 + Same segment as 1
    // 5 -> a b   d   f g -> 5 : Length 5 + Same segment as (4 - 1)
    // 2 -> a   c d e   g -> 5 : Last with length 5
    // 6 -> a b   d e f g -> 6 : Length 6 + Not same as 1
    // 9 -> a b c d   f g -> 6 : Length 6 + Same as 4
    // 0 -> a b c   e f g -> 6 : Last with length 6


    println!("Result : {}", result);

}




