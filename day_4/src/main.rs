use std::env;
use std::fs;
use std::process::exit;
use std::str::Lines;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {

    fn match_count(&self) -> u32 {
        let mut match_count: u32 = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                match_count += 1;
            }
        }
        match_count
    }

    fn score(&self) -> u32 {
        let match_count = self.match_count();
        match match_count {
            0 => 0,
            1 => 1,
            _ => 2_u32.pow(match_count - 1)
        }
    }
}

fn extract_numbers(string: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for i in string.split(" ") {
        match i.trim().parse::<u32>() {
            Err(_) => continue,
            Ok(j) => numbers.push(j)
        }
    }

    numbers
}

fn get_cards(file_lines: Lines) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in file_lines {
        let id;
        let winning_numbers: Vec<u32>;
        let numbers: Vec<u32>;

        match line.split_once(":") {
            None => continue,
            Some(i) => {
                id = i.0.trim().rsplit_once(" ").unwrap().1.parse().unwrap();
                match i.1.trim().split_once("|") {
                    None => continue,
                    Some(j) => {
                        winning_numbers = extract_numbers(j.0.trim());
                        numbers = extract_numbers(j.1.trim());
                    }
                }
            }
        }
        cards.push(Card { id, winning_numbers, numbers })
    }
    cards
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        dbg!("No input file provided");
        exit(0);
    }
    let input_filename = &args[1];
    let file_contents = fs::read_to_string(input_filename).expect("Could not read provided filepath");

    let cards: Vec<Card> = get_cards(file_contents.lines());

    let mut sum = 0;
    for card in cards {
        sum += card.score();
    }
    println!("{}", &sum);
}
