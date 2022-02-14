extern crate core;

mod day_2021_1_1;
mod day_2021_1_2;
mod day_2021_2_1;
mod day_2021_3_1;
mod day_2021_3_2;
mod day_2021_4_1;



fn main() {
    pretty_env_logger::init();
    let day = "day_2021_4_1";
    println!("Hello, {}", day);
    match day {
        "day_2021_1_1" => { day_2021_1_1::run() }
        "day_2021_1_2" => { day_2021_1_2::run() }
        "day_2021_2_1" => { day_2021_2_1::run() }
        "day_2021_3_1" => { day_2021_3_1::run() }
        "day_2021_3_2" => { day_2021_3_2::run() }
        "day_2021_4_1" => { day_2021_4_1::run() }
        _ => panic!("Not a good day!!")
    }.expect("Error in processing");
}
