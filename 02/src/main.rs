use std::fs;

const INPUT_FILE: &str = "input.txt";

fn main() {
    let lines = file_path_to_lines(INPUT_FILE.to_string());
    let mut games: Vec<Game> = vec![];
    for line in lines {
        let new_game = Game::from_string(line);
        games.push(new_game)
    }
    let mut total_valid_games = 0;
    for game in games.clone() {
        if game.clone().get_color_max_count(ColorCube::Red) < 13
        && game.clone().get_color_max_count(ColorCube::Green) < 14
        && game.clone().get_color_max_count(ColorCube::Blue) < 15
        {
            total_valid_games += game.id;
        }
    }
    println!("TOTAL VALID GAME ID SUM PART ONE: {}", total_valid_games);
    let mut total_power_games = 0;
    for game in games {
        total_power_games +=
            game.clone().get_color_max_count(ColorCube::Red) *
            game.clone().get_color_max_count(ColorCube::Green) *
            game.clone().get_color_max_count(ColorCube::Blue);
    }
    println!("TOTAL GAME POWER SUM PART TWO: {}",  total_power_games);
}

fn file_path_to_lines(file_path: String) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    for line in  fs::read_to_string(file_path).unwrap().lines() {
        lines.push(line.to_string())
    }
    return lines;
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum ColorCube {
    Red,
    Green,
    Blue,
    None,
}

#[derive(Debug, Clone, Copy)]
struct ColorCubeSet {
    count: u32,
    color: ColorCube,
}

#[derive(Debug, Clone)]
struct GameRound {
    cube_sets: Vec<ColorCubeSet>
}

impl GameRound {
    fn get_color_count(self, color: ColorCube) -> u32 {
        let mut color_count: u32 = 0;
        for color_set in self.cube_sets {
            if color == color_set.color {
                color_count += color_set.count
            }
        }
        return color_count
    }

    fn from_string(round_string: String) -> GameRound {
        let mut round = GameRound{
            cube_sets: vec![],
        };
        for cube_set in round_string.split(",") {
            let set_split: Vec<&str> = cube_set.trim().split(" ").collect();
            round.cube_sets.push(
                ColorCubeSet {
                    count: set_split[0].parse::<u32>().unwrap(),
                    color: match  set_split[1] {
                        "red"   => ColorCube::Red,
                        "green" => ColorCube::Green,
                        "blue"  => ColorCube::Blue,
                        _       => ColorCube::None,
                    }
                }
            )
        }
        return round
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<GameRound>,
}

impl Game {
    fn get_color_max_count(self, color: ColorCube) -> u32 {
        let mut color_count: u32 = 0;
        for round in self.rounds {
            let round_color_count = round.get_color_count(color);
            if round_color_count > color_count {
                color_count = round_color_count;
            }
        }
        return color_count
    }

    fn from_string(game_string: String) -> Game {
        let mut rounds: Vec<GameRound> = vec![];
        let game_id = game_string.clone()
            .split(":").collect::<Vec<_>>()[0]
            .split(" ").collect::<Vec<_>>()[1]
            .parse::<u32>().unwrap();
        for round_split in game_string.split(":").collect::<Vec<&str>>()[1].split(";") {
            rounds.push(
                GameRound::from_string(round_split.to_string())
            )
        }
        return Game{
            id: game_id,
            rounds: rounds,
        }
    }
}