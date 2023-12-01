use core::num;
use std::fs;
use regex::Regex;

const INPUT_FILE: &str = "input.txt";

fn main() {
    let lines = file_path_to_lines(INPUT_FILE.to_string());
    let mut total_sum = 0;
    for line in lines.clone() {
        total_sum = total_sum + get_first_and_last_digit_pair_part_one(line).combine_ints();
    }
    println!("TOTAL ONE: {total_sum}");
    let mut total_sum_two = 0;
    for line in lines {
        total_sum_two = total_sum_two + get_first_and_last_digit_pair_part_two(line).combine_ints();
    }
    println!("TOTAL TWO: {total_sum_two}");
}

fn file_path_to_lines(file_path: String) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    for line in  fs::read_to_string(file_path).unwrap().lines() {
        lines.push(line.to_string())
    }
    return lines;
}

#[derive(Debug)]
struct IntPair  {
    i1: u32,
    i2: u32,
}

impl IntPair {
    fn combine_ints(self) -> u32 {
        self.i1 * 10 + self.i2
    }
}

// me regex idea ^(one|two|three|four|five|six|seven|eight|nine)

fn get_first_and_last_digit_pair_part_one(input_string: String) -> IntPair {
    let mut new_pair = IntPair{
        i1: 0,
        i2: 0,
    };
    let mut first_found = false;

    for (i, c) in input_string.chars().enumerate(){
        if c.is_numeric(){
            if !first_found {
                new_pair.i1 = c.to_digit(10).unwrap();
                new_pair.i2 = c.to_digit(10).unwrap();
                first_found = true;
            }
            else {
                new_pair.i2 = c.to_digit(10).unwrap();
            }
        }
    }
    new_pair
}

fn i_from_alpha(s: String) -> Option<u32> {
    let num_regex = Regex::new(r"^(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    match num_regex.captures(&s) {
        Some(caps) => {
            Some(
                match &caps[0] {
                "one"   => {1}
                "two"   => {2}
                "three" => {3}
                "four"  => {4}
                "five"  => {5}
                "six"   => {6}
                "seven" => {7}
                "eight" => {8}
                "nine"  => {9}
                _       => {return None}
            })
        }
        None => {return None}
    }
}

fn get_first_and_last_digit_pair_part_two(input_string: String) -> IntPair {
    let mut new_pair = IntPair{
        i1: 0,
        i2: 0,
    };
    let mut first_found = false;

    for (i, c) in input_string.chars().enumerate(){
        if c.is_numeric(){
            if !first_found {
                new_pair.i1 = c.to_digit(10).unwrap();
                new_pair.i2 = c.to_digit(10).unwrap();
                first_found = true;
            }
            else {
                new_pair.i2 = c.to_digit(10).unwrap();
            }
        }
        match i_from_alpha(input_string[i..].to_string()) {
            Some(num) => {
                if !first_found {
                    new_pair.i1 = num;
                    new_pair.i2 = num;
                    first_found = true;
                }
                else {
                    new_pair.i2 = num;
                }
            }
            None => {}
        }
    }
    new_pair
}