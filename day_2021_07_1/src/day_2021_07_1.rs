extern crate log;

use std::fs;

use log::{error, info, trace};

pub fn main() {
    env_logger::init();

    info!("Starting day...");

    info!("Reading the file");
    let mut positions = Vec::from_iter(
        fs::read_to_string("files/day_07.txt").expect("Unable to read file")
            .split(',')
            .map(|d| d.parse::<u32>().expect("Unable to parse number")));

    info!("Sorting the position");
    positions.sort();

    info!("Building the crab fleet"); // A vec with all position and number of crabs at that position
    let mut crabs  = Vec::new();
    let mut iter_position = positions.iter();

    let first_crab = iter_position.next();
    let mut current_crab_fleet = CrabFleet {
        position : *first_crab.expect("At least one crab in fleet"),
        nb_vessel : 1,
    };
    while let Some(&crab_position) = iter_position.next() {
        if current_crab_fleet.position == crab_position {
            current_crab_fleet.nb_vessel += 1;
        } else {
            crabs.push(current_crab_fleet);
            current_crab_fleet = CrabFleet {
                position: crab_position,
                nb_vessel: 1,
            };
        }
    }
    crabs.push(current_crab_fleet);


    let mut min_iter = crabs.iter();
    let mut max_iter = crabs.iter().rev();
    let mut min_fleet = CrabFleet {
        ..*min_iter.next().expect("At least one fleet")
    };
    let mut max_fleet = CrabFleet {
        ..*max_iter.next().expect("At least one fleet")
    };

    let mut consumption = 0;

    while min_fleet.position < max_fleet.position {
        // We moving the smallest fleet
        if min_fleet.nb_vessel < max_fleet.nb_vessel {
            // Moving the min fleet to the 'next' fleet
            let next_fleet = min_iter.next().unwrap();
            consumption += min_fleet.nb_vessel * (next_fleet.position - min_fleet.position);
            // The min fleet is now bigger and at a new position
            min_fleet.position = next_fleet.position;
            min_fleet.nb_vessel += next_fleet.nb_vessel;

        } else {
            // Moving the max fleet to the 'previous' fleet
            let previous_fleet = max_iter.next().unwrap();
            consumption += max_fleet.nb_vessel * (max_fleet.position - previous_fleet.position);
            // The max fleet is now bigger and at a new position
            max_fleet.position = previous_fleet.position;
            max_fleet.nb_vessel += previous_fleet.nb_vessel;
        }
    }

    println!("Min fleet of {} at {}" , min_fleet.nb_vessel, min_fleet.position);
    println!("Max fleet of {} at {}" , max_fleet.nb_vessel, max_fleet.position);

    println!("Consumption : {}", consumption);

}


struct CrabFleet {
    position: u32,
    nb_vessel: u32,
}

