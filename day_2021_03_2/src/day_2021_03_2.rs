use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> std::io::Result<()> {

    let lines = load("day_3.txt")?;


    let mut oxygen = lines.to_vec();
    let mut idx = 0;

    while oxygen.len() > 1 {
        let most_used = most_bit_used(&oxygen, idx, '1');
        oxygen.retain(|value| value.chars().nth(idx).unwrap() == most_used);
        idx += 1;
    }



    let mut co2 = lines.to_vec();
    let mut idx = 0;

    while co2.len() > 1 {
        let most_used = most_bit_used(&co2, idx, '1');
        let less_used = match most_used {
            '1' => '0',
            '0' => '1',
            _ => panic!("Char most used not in allowed values {}", most_used),

        };
        co2.retain(|value| value.chars().nth(idx).unwrap() == less_used);
        idx += 1;
    }



    let oxygen = i32::from_str_radix(oxygen.get(0).unwrap(), 2).expect("Unable to parse oxygen");
    let co2 = i32::from_str_radix(co2.get(0).unwrap(), 2).expect("Unable to parse co2");

    println!("Oxygen : {}", oxygen);
    println!("Co2 : {}", co2);



    println!("Result : {}", oxygen * co2);

    Ok(())

}


fn most_bit_used(values: &Vec<String>, idx: usize, equals_char : char) -> char {
    let mut count_1 = 0;
    for value in values {
        if (*value).chars().nth(idx).unwrap() == '1' {
            count_1 += 1;
        }
    }
    let count_0 = values.len() - count_1;
    if count_0 > count_1 { '0' } else if count_1 > count_0 { '1' } else { equals_char }
}


fn load(file: &str) -> Result<Vec<String>, io::Error> {

    let file = File::open(file)?;
    let reader = io::BufReader::new(file).lines();

    let mut lines = Vec::new();
    for line in reader {
        let line = line?;
        lines.push(line);
    }
    Ok(lines)
}