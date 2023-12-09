use std::env;
use std::fs;
use std::process::exit;
use std::str::Split;

const MAX_RED_COUNT: u32 = 12;
const MAX_GREEN_COUNT: u32 = 13;
const MAX_BLUE_COUNT: u32 = 14;

#[derive(Debug)]
struct Round {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>
}

fn read_one_game(line: &str) -> Game {
    let id = match line.split_once(":") {
        None => 0,
        Some(i) => match i.0.split_once(" ") {
            None => 0,
            Some(j) => j.1.trim().parse().unwrap()
        }
    };

    let mut rounds: Vec<Round> = Vec::new();
    let round_strings: Split<'_, &str>  = match line.split_once(":") {
        None => "".split(";"),
        Some(i) => i.1.split(";")
    };

    for portion in round_strings {
        let mut round = Round {
            red_count: 0,
            green_count: 0,
            blue_count: 0
        };
        for count in portion.trim().split(",") {
            dbg!(count);
            match count.trim().split_once(" ") {
                None => (),
                Some(i) => match &(i.1.trim().to_lowercase()[..]) {
                    "red" => round.red_count += i.0.parse::<u32>().unwrap(),
                    "green" => round.green_count += i.0.parse::<u32>().unwrap(),
                    "blue" => round.blue_count += i.0.parse::<u32>().unwrap(),
                    _ => ()
                }
            };
        }
        rounds.push(round);
    }

    let game = Game {
        id,
        rounds
    };
    game
}

fn read_games_from_file(filename: &str) -> Vec<Game> {
    let file_contents = fs::read_to_string(filename).expect("Could not read provided filepath");
    let mut games: Vec<Game> = Vec::new();
    for line in file_contents.lines() {
        games.push(read_one_game(line));
    }

    games
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit(0);
    }
    let input_filename = &args[1];
    let games: Vec<Game> = read_games_from_file(&input_filename);

    dbg!(games);
}
