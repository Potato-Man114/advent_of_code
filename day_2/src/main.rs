use std::env;
use std::fs;
use std::process::exit;
use std::str::Split;

const MAX_RED_COUNT: u32 = 12;
const MAX_GREEN_COUNT: u32 = 13;
const MAX_BLUE_COUNT: u32 = 14;

macro_rules! ternary {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test { $true_expr } else { $false_expr }
    };
}

#[derive(Debug)]
struct Round {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl Round {
    fn is_possible(&self, maximums: &Round) -> bool {
        self.red_count <= maximums.red_count && self.blue_count <= maximums.blue_count && self.green_count <= maximums.green_count
    }

    fn power(&self) -> u32 {
        self.red_count * self.blue_count * self.green_count
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>
}

impl Game {
    fn is_possible(&self, maximums: &Round) -> bool {
        for round in &self.rounds {
            if !round.is_possible(maximums) {
                return false;
            }
        }
        true
    }

    fn fewest_possible(&self) -> Round {
        let mut least = Round {
            red_count: 0,
            green_count: 0,
            blue_count: 0
        };

        for round in &self.rounds {
            least.red_count = ternary!(round.red_count > least.red_count => round.red_count; least.red_count);
            least.blue_count = ternary!(round.blue_count > least.blue_count => round.blue_count; least.blue_count);
            least.green_count = ternary!(round.green_count > least.green_count => round.green_count; least.green_count);

        }


        least
    }
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

fn find_possible_games(games: Vec<Game>, max_counts: Round) -> Vec<Game> {
    let mut possible_games: Vec<Game> = Vec::new();

    for game in games {
        if game.is_possible(&max_counts) {
            possible_games.push(game);
        }
    }
    possible_games
}

fn get_all_fewest(games: Vec<Game>) -> Vec<Round> {
    let mut fewest: Vec<Round> = Vec::new();
    for game in games {
        fewest.push(game.fewest_possible());
    }
    fewest
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit(0);
    }
    let input_filename = &args[1];
    let games: Vec<Game> = read_games_from_file(&input_filename);
    // let max_counts = Round {
    //     red_count: MAX_RED_COUNT,
    //     green_count: MAX_GREEN_COUNT,
    //     blue_count: MAX_BLUE_COUNT
    // };
    // let possible_games: Vec<Game> = find_possible_games(games, max_counts);
    // let mut sum = 0;
    // for game in possible_games {
    //     sum += game.id;
    // }
    // dbg!(sum);

    let fewest_possible_cubes: Vec<Round> = get_all_fewest(games);

    let mut sum = 0;
    for round in fewest_possible_cubes {
        sum += round.power();
    }

    dbg!(sum);


}
