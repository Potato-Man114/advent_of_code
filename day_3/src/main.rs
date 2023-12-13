use std::env;
use std::fs;
use std::process::exit;

fn split_contents_2d(file_contents: String) -> Vec<String> {
    let mut characters: Vec<String> = Vec::new();

    for line in file_contents.lines() {
        characters.push(line.to_string());
        // let mut this_line_characters: Vec<char> = Vec::new();
        // for character in line.chars() {
        //     this_line_characters.push(character);
        // }
        // characters.push(this_line_characters);
    }

    characters
}

fn get_numbers_adjacent_symbols(characters: Vec<String>) -> Vec<u32> {
    const NOT_SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut numbers: Vec<u32> = Vec::new();

    for line in 0..characters.len() {
        let mut numbers_on_line: Vec<(String, usize, usize)> = Vec::new();
        let mut m = characters[line].match_indices(|x: char| char::is_ascii_digit(&x)).peekable();
        let mut working: Option<(String, usize, usize)> = Option::None;
        while m.peek().is_some() {
            let digit = m.next().unwrap();
            if working.is_some() {
                let unwrapped = working.unwrap();
                if unwrapped.2 > digit.0 + 1 {
                    numbers_on_line.push(unwrapped.clone());
                    working = Option::Some((digit.1.to_string(), digit.0, digit.0));
                }
                working = Option::Some((unwrapped.0.to_string() + digit.1, unwrapped.1, digit.0));
            }
            else {
                working = Option::Some((digit.1.to_string(), digit.0, digit.0));
            }

            // let digit = m.next().unwrap();
            // if m.peek().is_some() && m.peek().unwrap().0 == digit.0 + 1 { //next digit is part of the same number
            //     if working.is_some() {
            //         working = Option::some((working.unwrap().0 + ))
            //     }
            //     else {
            //         working = Option::Some((digit.1.to_string() + m.peek().unwrap().1, digit.0, digit.0 + 1));
            //     }
            // }
            // else {

            // }
        }
        if working.is_some() {
            numbers_on_line.push(working.unwrap());
        }
    }

    numbers
}

fn get_part_numbers(file_contents: String) -> Vec<u32> {
    let characters = split_contents_2d(file_contents);
    let part_numbers = get_numbers_adjacent_symbols(characters);
    part_numbers
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit(0);
    }
    let input_filename = &args[1];
    let file_contents = fs::read_to_string(input_filename).expect("Could not read provided filepath");

    let part_numbers: Vec<u32> = get_part_numbers(file_contents);


    let mut sum = 0;
    for part_number in part_numbers {
        sum += part_number;
    }

    dbg!("sum", sum);

}