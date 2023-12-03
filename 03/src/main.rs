use std::fs;
use std::ops::{Add, Sub};
use regex::Regex;
use once_cell::sync::Lazy;

static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([^0-9\.])").unwrap());
static PART_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)").unwrap());

fn file_path_to_lines(file_path: String) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    for line in  fs::read_to_string(file_path).unwrap().lines() {
        lines.push(line.to_string())
    }
    return lines;
}

#[derive(Debug, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn off_grid(&self) -> bool {
        if self.x < 0 || self.y < 0{
            return true
        }
        return false
    } 
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone)]
struct PartNumber {
    number: u32,
    len: u32,
    start_pos: Vec2,
}

#[derive(Debug, Clone)]
struct SpecGrid {
    size: Vec2,
    grid: Vec<Vec<char>>,
}

impl SpecGrid {
    fn get_part_numbers(self) -> Vec<PartNumber> {
        let mut parts = vec![];
        for (y_pos, line) in self.grid.clone().into_iter().enumerate(){
            let mut matched_upto: u32 = 0;
            for (x_pos, c) in line.clone().into_iter().enumerate(){
                if x_pos as u32 >= matched_upto {
                    match PART_NUMBER_REGEX.captures(line[x_pos..].iter().collect::<String>().as_str()){
                        Some(caps) => {
                            parts.push(PartNumber { 
                                number: caps[0].parse().unwrap(),
                                len: caps[0].len() as u32,
                                start_pos: Vec2 { x: x_pos as i32, y: y_pos as i32},
                            });
                            matched_upto = (x_pos + caps[0].len()) as u32;
                        }
                        None => {}
                    }
                }
            }
        }
        return parts
    }

    fn is_part_valid(self, part: PartNumber) -> bool {
        let mut symbol_found = false;
        let check_vecs = vec![
            Vec2{x:-1,y:-1},
            Vec2{x:-1,y:0},
            Vec2{x:-1,y:1},
            Vec2{x:0,y:-1},
            Vec2{x:0,y:1},
            Vec2{x:1,y:-1},
            Vec2{x:1,y:0},
            Vec2{x:1,y:1},
        ];
        for x in part.start_pos.x..(part.start_pos.x + part.len as i32 ) {
            let current_origin = Vec2{x: x, y: part.start_pos.y};
            for check_offset in check_vecs.clone() {
                let check_point = current_origin.clone() + check_offset;
                if !check_point.clone().off_grid() && check_point.clone().x < self.clone().size.x && check_point.clone().y < self.clone().size.y{
                    // println!("cur check point: {:?} {:?}", check_point.x as usize, check_point.y as usize);
                    // println!("char: {:?}", self.grid.clone()[check_point.y as usize][check_point.x as usize].to_string().as_str());
                    if SYMBOL_REGEX.is_match(self.grid.clone()[check_point.y as usize][check_point.x as usize].to_string().as_str()) {
                        symbol_found = true;
                    }
                }
            }
        }
        return symbol_found
    }
}

fn main(){
    let lines = file_path_to_lines("input.txt".to_string());
    let mut main_grid = SpecGrid{
        size: Vec2 {
            x: lines[0].len() as i32,
            y: lines.len() as i32,
        },
        grid: vec![]
    };
    for line in lines.clone() {
        main_grid.grid.push(line.chars().collect())
    }
    let parts = main_grid.clone().get_part_numbers();
    let mut part_total = 0;
    for part in parts{
        if main_grid.clone().is_part_valid(part.clone()) {
            part_total += part.number;
        }
    }
    println!("TOTAL: {}", part_total);
}
