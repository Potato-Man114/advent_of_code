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

fn single_adjacent_to_symbol(characters: &Vec<String>, position: (usize, usize)) -> bool {
    const NOT_SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    //basic sanity checks
    if position.0 >= characters.len() || position.1 >= characters[position.0].len() {
        return false;
    }

    //check left
    if position.1 != 0 && !NOT_SYMBOLS.contains(&characters[position.0].chars().nth(position.1 - 1).unwrap()) {
        return true;
    }

    //check right
    if position.1 != characters[position.0].len() - 1 && !NOT_SYMBOLS.contains(&characters[position.0].chars().nth(position.1 + 1).unwrap()) {
        return true;
    }

    //check above
    if position.0 != 0 {
        //check directly above
        if !NOT_SYMBOLS.contains(&characters[position.0 - 1].chars().nth(position.1).unwrap()) {
            return true;
        }
        //check left
        if position.1 != 0 && !NOT_SYMBOLS.contains(&characters[position.0 - 1].chars().nth(position.1 - 1).unwrap()) {
            return true;
        }

        //check right
        if position.1 != characters[position.0 - 1].len() - 1 && !NOT_SYMBOLS.contains(&characters[position.0 - 1].chars().nth(position.1 + 1).unwrap()) {
            return true;
        }

    }
    //check below
    if position.0 != characters.len() - 1 {
        //check directly below
        if !NOT_SYMBOLS.contains(&characters[position.0 + 1].chars().nth(position.1).unwrap()) {
            return true;
        }
        //check left
        if position.1 != 0 && !NOT_SYMBOLS.contains(&characters[position.0 + 1].chars().nth(position.1 - 1).unwrap()) {
            return true;
        }

        //check right
        if position.1 != characters[position.0 + 1].len() - 1 && !NOT_SYMBOLS.contains(&characters[position.0 + 1].chars().nth(position.1 + 1).unwrap()) {
            return true;
        }
    }

    false
}

fn adjacent_to_symbol(characters: &Vec<String>, number: &(String, usize, usize, usize)) -> bool {
    let mut adjacent_coordinates: Vec<(usize, usize)> = Vec::new();

    let line = number.1;
    let start_col = number.2;
    let end_col = number.3;

    for col in start_col..end_col + 1 {
        if single_adjacent_to_symbol(characters, (line, col)) {
            return true
        }
    }

    false
}

fn get_numbers_adjacent_symbols(characters: Vec<String>) -> Vec<u32> {
    let mut numbers: Vec<(String, usize, usize, usize)> = Vec::new();

    for line in 0..characters.len() {
        let mut numbers_on_line: Vec<(String, usize, usize, usize)> = Vec::new();
        let mut m = characters[line].match_indices(|x: char| char::is_ascii_digit(&x)).peekable();
        let mut working: Option<(String, usize, usize, usize)> = Option::None;
        while m.peek().is_some() {
            let digit = m.next().unwrap();
            if working.is_some() {
                let unwrapped = working.unwrap();
                if unwrapped.2 > digit.0 + 1 {
                    numbers_on_line.push(unwrapped.clone());
                    working = Option::Some((digit.1.to_string(), line, digit.0, digit.0));
                }
                working = Option::Some((unwrapped.0.to_string() + digit.1, line, unwrapped.1, digit.0));
            }
            else {
                working = Option::Some((digit.1.to_string(), line, digit.0, digit.0));
            }
        }
        if working.is_some() {
            numbers_on_line.push(working.unwrap());
        }
        numbers.extend(numbers_on_line);
    }

    let mut part_numbers: Vec<u32> = Vec::new();

    for number in numbers {
        if adjacent_to_symbol(&characters, &number) {
            part_numbers.push(number.0.parse().ok().unwrap());
        }
    }

    part_numbers
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