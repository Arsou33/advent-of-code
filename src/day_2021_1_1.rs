use std::fs::File;
use std::io;
use std::io::BufRead;


pub fn run() -> std::io::Result<()> {

    let file = File::open("day_1.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut previous = 0;
    let mut counter = -1;
    for line in lines {
        let line = line.unwrap();
        //if let Ok(line) = line {
            let value : u32 = line.parse().unwrap();
            if value > previous {
                counter += 1;
            }
            previous = value;
        //}
    }


    println!("Result : {}", counter);

    Ok(())

}