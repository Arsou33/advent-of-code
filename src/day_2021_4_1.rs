use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
struct Square {
    value : u32,
    drawn: bool,
}

struct Card {
    values : [[Square;5];5],
    match_by_line : [u32;5],
    match_by_column : [u32;5],
}

impl Card {
    fn set_value(&mut self, i: usize, j:usize, value: u32) {
        self.values[i][j].value = value;
    }
}

impl Card {
    fn from_lines(lines: &Vec<Vec<u32>>) -> Card {
        let mut card = Card {
            values : [[Square { value: 0, drawn: false };5];5],
            match_by_line: [0;5],
            match_by_column: [0;5],
        };
        for (i, line) in lines.iter().enumerate() {
            for (j, value) in  line.iter().enumerate() {
                card.set_value(i, j, *value);
            }
        }
        card
    }

    fn play(&mut self, value: u32) -> Option<u32> {
        for (no_line, line) in self.values.iter_mut().enumerate() {
            for (no_col, square) in line.iter_mut().enumerate() {
                if square.value == value {
                    if square.drawn { panic!("Draw already played"); }
                    square.drawn = true;
                    self.match_by_line[no_line] += 1;
                    self.match_by_column[no_col] += 1;

                    // If card is winning return the sum of all drawn values
                    if self.match_by_line[no_line] == 5 || self.match_by_column[no_col] == 5 {
                        return Some(self.sum_not_drawn());
                    }
                }
            }
        }
        None
    }

    fn sum_not_drawn(&self) -> u32 {
        let mut sum = 0u32;
        for line in self.values {
            for square in line {
               if !square.drawn {
                   sum += square.value;
               }
            }

        }
        return sum;
    }
}



pub fn run() -> std::io::Result<()> {

    let file = File::open("day_4.txt")?;
    let reader = io::BufReader::new(file).lines();

    let mut iter = reader.into_iter();
    let draw_line = iter.next().unwrap()?;
    let draws = Vec::from_iter(draw_line.split(',').map(|d| d.parse::<u32>().unwrap()));

    // Reading the cards from the file
    let mut cards = Vec::new();
    while let Some(line) = iter.next() {

        // At the beginning of a card, line is empty
        let line = line?;
        if !line.is_empty() { panic!("Line should be empty {}", line)};

        // We read the five line of the card
        let mut lines = Vec::new();
        for _ in 0..5 {
            let line = iter.next().unwrap()?;
            let line = Vec::from_iter(line.split_whitespace().map(|v| v.parse::<u32>().unwrap()));
            lines.push(line);

        }
        let card = Card::from_lines(&lines);
        cards.push(card);
    }


    match play(&draws,&mut cards) {
        Some(x) => {
            println!("Result : {:?}", x);
            return Ok(());

        }
        _ => {}
    }


    panic!("No one wins");


}



fn play(draws: &Vec<u32>, cards: &mut Vec<Card>) -> Option<u32> {
    for draw in draws.iter() {
        for card in cards.iter_mut() {
            if let Some(result) = card.play(*draw) {
               return Some(result * *draw);
            }
        }
    }
    // No one wins
    None
}
