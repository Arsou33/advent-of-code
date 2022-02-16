extern crate log;

use std::{fs, io};

use log::{debug, error, info, trace};

pub fn main() {
    env_logger::init();

    info!("Starting day...");

    info!("Reading the file");
    let fishes = Vec::from_iter(
        fs::read_to_string("files/day_06.txt").expect("Unable to read file")
            .split(',')
            .map(|d| d.parse::<usize>().expect("Unable to parse number")));

    let mut fishes_by_age = [0; 9];

    for age in fishes {
        fishes_by_age[age] += 1;
    }

    info!("Calculating population");
    for day in 1..=256 {
        let nb_recycle = fishes_by_age[0];
        for age in 1..=8 {
            fishes_by_age[age - 1] = fishes_by_age[age]
        }
        fishes_by_age[6] += nb_recycle;
        fishes_by_age[8] = nb_recycle;
        let nb_fishes : usize = fishes_by_age.iter().sum();
        info!("After {} days {:?}, sum: {}", day, fishes_by_age, nb_fishes);
    }


    let nb_fishes : usize = fishes_by_age.iter().sum();
    println!("Result : {}", nb_fishes);

}
