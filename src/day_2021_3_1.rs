use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn run() -> std::io::Result<()> {

    let file = File::open("day_3.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut nb_line = 0;
    let mut counters = vec![0];
    let mut values = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if nb_line == 0 {
            counters = vec![0; line.len()];
        }

        for (i, bit) in line.chars().enumerate() {
            let bit = match bit {
                '0' => 0,
                '1' => 1,
                x => panic!("Unknown bit char {}", x),
            };
            counters[i] += bit;
        }
        values.push(line);
        nb_line += 1;
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();


    for count_1 in counters {
        let count_0 = nb_line - count_1;

        if count_1 > count_0 {
            // 1 is most common
            gamma.push('1');
            epsilon.push('0');
        } else {
            // 0 is most common
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!("Gamma : {}", gamma);
    println!("Epsilon : {}", epsilon);

    let gamma = i32::from_str_radix(&gamma, 2).expect("Unable to parse gamma");
    let epsilon = i32::from_str_radix(&epsilon, 2).expect("Unable to parse epsilon");


    println!("Result : {}", gamma * epsilon);

    Ok(())

}