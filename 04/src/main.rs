use std::{fs, collections::HashMap};
// use regex::Regex;
// use once_cell::sync::Lazy;

// static SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^0-9\.]").unwrap());

fn file_path_to_lines(file_path: String) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    for line in  fs::read_to_string(file_path).unwrap().lines() {
        lines.push(line.to_string())
    }
    return lines;
}

#[derive(Debug, Clone)]
struct Card {
    Id: i32,
    WinNums: Vec<u32>,
    Nums: Vec<u32>,
}

impl Card {
    fn from_string(s: String) -> Card {
        return Card {
            Id: 
                s.split(":").collect::<Vec<_>>()
                [0].split_whitespace().collect::<Vec<_>>()
                [1].parse::<i32>().unwrap(),
            WinNums: 
                s.split(":").collect::<Vec<_>>()
                [1].split("|").collect::<Vec<_>>()
                [0].trim().split_whitespace().map(|n| n.trim().parse::<u32>().unwrap()).collect(),
            Nums:
                s.split(":").collect::<Vec<_>>()
                [1].split("|").collect::<Vec<_>>()
                [1].trim().split_whitespace().map(|n| {n.trim().parse::<u32>().unwrap()}).collect(),
        }
    }

    fn get_score(self) -> i32 {
        let mut score = 0;
        for num in self.Nums {
            if self.WinNums.contains(&num) {
                if score == 0 {
                    score += 1;
                } else {
                    score *= 2;
                }
            }
        }
        return score
    }

    fn get_wins(self) -> i32 {
        let mut wins = 0;
        for num in self.Nums {
            if self.WinNums.contains(&num) {
                wins += 1;
            }
        }
        return wins
    }
}

fn main() {
    let lines = file_path_to_lines("input.txt".to_string());
    let cards: Vec<Card> = lines.iter().map(|s| Card::from_string(s.to_string())).collect();
    
    let mut total_score = 0;
    for card in cards.clone() {
        total_score += card.get_score();
    }

    println!("TOTAL Part 1: {}", total_score);

    let mut card_catalog: HashMap<i32, i32> = HashMap::new();

    for card in cards.clone() {
        // init the card
        if !card_catalog.contains_key(&card.Id) {
            card_catalog.insert(card.Id, 1);
            let c_wins = card.clone().get_wins();
            for i in card.Id+1..card.Id+1+c_wins {
                if !card_catalog.contains_key(&i) {
                    card_catalog.insert(i, 1);
                } else {
                    card_catalog.insert(i, card_catalog.get(&i).unwrap().to_owned() + 1);
                }
            }
        } else {
            let cum_card_count = card_catalog.insert(card.Id, card_catalog.get(&card.Id).unwrap().to_owned() + 1);
            match cum_card_count {
                Some(n) => {
                    for x in 0..n+1 {
                        let c_wins = card.clone().get_wins();
                        for i in card.Id+1..card.Id+1+c_wins {
                            if !card_catalog.contains_key(&i) {
                                card_catalog.insert(i, 1);
                            } else {
                                card_catalog.insert(i, card_catalog.get(&i).unwrap().to_owned() + 1);
                            }
                        }
                    }
                }
                None => {}
            }
        }

    }

    let mut part_two_total = 0;
    for card in cards.clone() {
        part_two_total += card_catalog.get(&card.Id).unwrap()
    }

    println!("TOTAL Part 2: {}", part_two_total);
}