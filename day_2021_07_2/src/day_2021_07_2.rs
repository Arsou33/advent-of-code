extern crate log;

use std::fs;

use log::{info};

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


    info!("Moving crabs");
    let mut min_iter = crabs.iter();
    let mut max_iter = crabs.iter().rev();
    let first_fleet = min_iter.next().expect("At least one fleet");
    let last_fleet = max_iter.next().expect("At least one fleet");
    let mut min_fleet = MovingFleet {
        position: first_fleet.position,
        fleets: vec!(first_fleet),
    };
    let mut max_fleet = MovingFleet {
        position: last_fleet.position,
        fleets: vec!(last_fleet),
    };

    let mut next_min_fleet = min_iter.next();
    let mut previous_max_fleet = max_iter.next();

    let mut consumption = 0;

    while min_fleet.position < max_fleet.position {
        // We moving the fleet the less expensive to move of 1
        let min_fleet_consumption = min_fleet.fleets.iter()
            .fold(0,  |acc, f| acc + f.nb_vessel * (min_fleet.position + 1 - f.position) );
        let max_fleet_consumption = max_fleet.fleets.iter()
            .fold(0,  |acc, f| acc + f.nb_vessel * (f.position - (max_fleet.position -1)) );

        if min_fleet_consumption < max_fleet_consumption {
            // Moving the min fleet to the previous position
            consumption += min_fleet_consumption;
            min_fleet.position += 1;
            if let Some(current_next_min_fleet) = next_min_fleet {
                if current_next_min_fleet.position == min_fleet.position {
                    min_fleet.fleets.push(current_next_min_fleet);
                    next_min_fleet = min_iter.next();
                }
            }

        } else {
            // Moving the max fleet to the 'previous' fleet
            consumption += max_fleet_consumption;
            max_fleet.position -= 1;
            if let Some(current_previous_max_fleet) = previous_max_fleet {
                if current_previous_max_fleet.position == max_fleet.position {
                    max_fleet.fleets.push(current_previous_max_fleet);
                    previous_max_fleet = max_iter.next();
                }
            }
        }
    }

    let min_fleet_nb = min_fleet.fleets.iter().fold(0, |acc, f| acc + f.nb_vessel);
    let max_fleet_nb = max_fleet.fleets.iter().fold(0, |acc, f| acc + f.nb_vessel);

    println!("Min fleet of {} at {}" , min_fleet_nb, min_fleet.position);
    println!("Max fleet of {} at {}" , max_fleet_nb, max_fleet.position);

    println!("Consumption : {}", consumption);

}


struct CrabFleet {
    position: u32,
    nb_vessel: u32,
}

// struct MovingFleet {
//     position: u32,
//     fleets: Vec<CrabFleet>,
// }

struct MovingFleet<'a> {
    position: u32,
    fleets: Vec<&'a CrabFleet>,
}


