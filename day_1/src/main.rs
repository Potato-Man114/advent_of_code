use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
use std::str::Split;


fn get_calibration_value(values: Split<'_, &str>, matches: HashMap<String, u32>) -> u32 {
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

fn get_matches() -> HashMap<String, u32> {
    let mut matches: HashMap<String, u32> = HashMap::new();

    matches.insert("0".to_string(), 0);
    matches.insert("1".to_string(), 1);
    matches.insert("2".to_string(), 2);
    matches.insert("3".to_string(), 3);
    matches.insert("4".to_string(), 4);
    matches.insert("5".to_string(), 5);
    matches.insert("6".to_string(), 6);
    matches.insert("7".to_string(), 7);
    matches.insert("8".to_string(), 8);
    matches.insert("9".to_string(), 9);
    // matches.insert("1", 1);
    // matches.insert("1", 1);
    // matches.insert("1", 1);

    matches
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() == 1 {
        exit(0);
    }
    let filename = &args[1];
    
    let file_contents = fs::read_to_string(filename).expect("Could not read provided filepath.");
    
    let calibration_value = get_calibration_value(file_contents.split("\n"), get_matches());

    println!("{}", calibration_value);

}


