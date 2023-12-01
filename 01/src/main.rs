use std::fs;

const INPUT_FILE: &str = "input.txt";

fn main() {
    println!("File {INPUT_FILE}");
    let lines = file_path_to_lines(INPUT_FILE.to_string());
    let mut total_sum = 0;
    for line in lines {
        total_sum = total_sum + get_first_and_last_digit_pair(line).combine_ints();
    }
    println!("TOTAL: {total_sum}")
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
        println!("{}",  self.i1 * 10 + self.i2);
        self.i1 * 10 + self.i2
    }
}

// me regex idea ^(one|two|three|four|five|six|seven|eight|nine)

fn get_first_and_last_digit_pair(input_string: String) -> IntPair {
    let mut new_pair = IntPair{
        i1: 0,
        i2: 0,
    };
    let mut first_found = false;

    for (i, c) in input_string.chars().enumerate(){
        println!("{input_string}");
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