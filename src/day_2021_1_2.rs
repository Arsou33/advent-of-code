use std::fs::File;
use std::io;
use std::io::BufRead;


pub fn run() -> std::io::Result<()> {

    let file = File::open("day_1.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut counters = [0,0,0]; // 3 sliding windows counter
    let mut nb_increase = -3;
    let mut idx = 0;
    for line in lines {
        let current_value :i32 = line.unwrap().parse().unwrap();

        let previous_window_count = counters[idx%3];
        counters[idx%3] = 0; // Reset the previous sliding counter
        for count in &mut counters { // All 3 counters increments by current line value
            *count += current_value;
        }

        idx += 1;
        let current_window_count = counters[(idx)%3];
        if current_window_count > previous_window_count {
            nb_increase += 1;
        }


    }


    println!("Result : {}", nb_increase);

    Ok(())


}