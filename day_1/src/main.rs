use std::env;
use std::fs;
use std::process::exit;
use std::str::Split;


fn get_calibration_value(values: Split<'_, &str>) -> u32 {
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


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() == 1 {
        exit(0);
    }
    let filename = &args[1];

    let file_contents = fs::read_to_string(filename).expect("Could not read provided filepath.");
    
    let calibration_value = get_calibration_value(file_contents.split("\n"));

    println!("{}", calibration_value);

}


