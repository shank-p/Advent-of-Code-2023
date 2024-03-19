/*
    --- Day 1: Trebuchet?! ---
    https://adventofcode.com/2023/day/1
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const DECIMAL_RADIX: u32 = 10;

fn parse_calibration_file_part_1(file_path: &str) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut raw_calibration_values: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (mut first_digit, mut last_digit) = (None, None);
                for ch in line.chars() {
                    if ch.is_digit(DECIMAL_RADIX) {
                        let digit = ch.to_digit(DECIMAL_RADIX);
                        if last_digit.is_none() {
                            first_digit = digit;
                            last_digit = digit;
                        } else {
                            last_digit = digit;
                        }
                    }
                }
                raw_calibration_values.push(vec![first_digit.unwrap(), last_digit.unwrap()]);
            },
            Err(..) => {},
        }
    }
    Ok(raw_calibration_values)
}

fn parse_calibration_file_part_2(file_path: &str) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let digits: HashMap<String,u32> = HashMap::from([
        (("one").to_string(), 1), 
        (("two").to_string(), 2),
        (("three").to_string(), 3), 
        (("four").to_string(), 4),
        (("five").to_string(), 5), 
        (("six").to_string(), 6),
        (("seven").to_string(), 7), 
        (("eight").to_string(), 8),
        (("nine").to_string(), 9), 
        (("zero").to_string(), 0),  
    ]);

    let mut raw_calibration_values: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (mut first_digit, mut last_digit): (Option<u32>, Option<u32>) = (None, None);
                let (mut first_found, mut last_found): (bool, bool) = (false, false);
                let mut temp_str = String::new(); 
                
                /* first digit */
                for ch in line.chars() {
                    if first_found {break;} 
                    if ch.is_digit(DECIMAL_RADIX) {
                        first_digit = ch.to_digit(DECIMAL_RADIX);
                        break;
                    }
                    temp_str.push(ch);
                    for (key, &value) in &digits {
                        if temp_str.contains(&*key) {
                            first_digit = Some(value);
                            first_found = true;
                            break;
                        }
                    }
                }

                temp_str = String::new();
                /* last digit */
                for ch in line.chars().rev() {
                    if last_found {break}
                    if ch.is_digit(DECIMAL_RADIX) {
                        last_digit = ch.to_digit(DECIMAL_RADIX);
                        break;
                    }
                    temp_str.insert(0, ch);
                    for (key, &value) in &digits {
                        if temp_str.contains(&*key) {
                            last_digit = Some(value);
                            last_found = true;
                            break;
                        }
                    }
                }
                raw_calibration_values.push(vec![first_digit.unwrap(), last_digit.unwrap()]);
            },
            Err(..) => {},
        }
    }
    Ok(raw_calibration_values)
}

fn aggregate_calibration_values(raw_calibration_values: Vec<Vec<u32>>) -> u32 {
    let mut sum_aggregate: u32 = 0;
    for values in raw_calibration_values {
        let calibration_value = values[0]*10 + values[1];
        sum_aggregate += calibration_value;
    }
    sum_aggregate
}

fn main() {
    let input_file = "day_1_Trbuchet.txt";
    let raw_calibration_values_part_1 = parse_calibration_file_part_1(input_file).unwrap();
    let raw_calibration_values_part_2 = parse_calibration_file_part_2(input_file).unwrap();
    let res_part_1 = aggregate_calibration_values(raw_calibration_values_part_1);
    let res_part_2 = aggregate_calibration_values(raw_calibration_values_part_2);
    println!(" # --- Day 1: Trebuchet?! --- # ");
    println!(" -->> Result Part 1 : {res_part_1}");
    println!(" -->> Result Part 2 : {res_part_2}");
}