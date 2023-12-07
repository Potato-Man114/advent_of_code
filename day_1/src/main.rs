use std::env;
use std::fs;
use std::process::exit;
use std::str::Split;


fn get_calibration_value(values: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for value in values {

        let left = match value.find(|x: char| x.is_ascii_digit()) {
            None => "",
            Some(i) => &value[i..i+1]
        };
        let right = match value.rfind(|x: char| x.is_ascii_digit()) {
            None => "",
            Some(i) => &value[i..i+1]
        };
        
        sum += match (left.to_owned() + right).parse() {
            Err(_) => 0,
            Ok(i) => i
        }
    }
    sum
}

fn get_modifications() -> Vec<(&'static str, &'static str)> {
    let mut modifications: Vec<(&'static str, &'static str)> = Vec::new();

    // modifications.insert("zero", "0");
    modifications.push(("one", "o1e"));
    modifications.push(("two", "t2o"));
    modifications.push(("three", "t3e"));
    modifications.push(("four", "f4r"));
    modifications.push(("five", "f5e"));
    modifications.push(("six", "s6x"));
    modifications.push(("seven", "s7n"));
    modifications.push(("eight", "e8t"));
    modifications.push(("nine", "n9e"));
    modifications.push(("1", "1"));
    modifications.push(("2", "2"));
    modifications.push(("3", "3"));
    modifications.push(("4", "4"));
    modifications.push(("5", "5"));
    modifications.push(("6", "6"));
    modifications.push(("7", "7"));
    modifications.push(("8", "8"));
    modifications.push(("9", "9"));

    modifications
}

fn modify_puzzle_input(original_values: Split<'_, &str>, modifications: &Vec<(&'static str, &'static str)>) -> Vec<String> {
    let mut modified: Vec<String> = Vec::new();

    for value in original_values {
        let mut single_modified_value = value.clone().to_string();
        for modification in modifications.into_iter() {
            single_modified_value = single_modified_value.replace(modification.0, modification.1);
        }
        modified.push(single_modified_value.to_string());
    }

    modified
}


fn main() {
    
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() == 1 {
        exit(0);
    }
    let filename = &args[1];
    
    let file_contents = fs::read_to_string(filename).expect("Could not read provided filepath.");
    let modified_values = modify_puzzle_input(file_contents.split("\n"), &get_modifications());
    dbg!(&modified_values);
    let calibration_value = get_calibration_value(modified_values);

    println!("{}", calibration_value);

}


