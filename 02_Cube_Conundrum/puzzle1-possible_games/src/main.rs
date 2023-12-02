use std::fs;
use std::num::ParseIntError;
use std::fmt;


const INPUT_FILE: &str = "../../data/02_cube_game_results.txt";
const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;


fn main() {
    // Read input data
    let input_data = fs::read_to_string(INPUT_FILE)
        .expect("Something went wrong reading the file");

    // Get IDs when possible
    let sum_of_ids = input_data.lines()
        .map(|s| parse_game(s).id_when_possible(MAX_RED, MAX_GREEN, MAX_BLUE))
        .sum::<u32>();

    println!("Sum of possible IDs: {}", sum_of_ids);

}


fn parse_int(input: &str) -> Result<u32, ParseIntError> {
    // Parse first integer in input string
    input.chars()
        .filter(|a| a.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
}


fn parse_game_result(input_value: &str) -> GameResult {
    // input_value has format:
    // <n> green, <n> red, <n> blue
    let mut n_blue: u32 = 0;
    let mut n_red: u32 = 0;
    let mut n_green: u32 = 0;

    for part in input_value.split(",") {
        let number = parse_int(part).unwrap();
        if part.contains("blue") {
            n_blue = number;
        } else if part.contains("red") {
            n_red = number;
        } else if part.contains("green") {
            n_green = number;
        }
    }
    GameResult {
        n_blue,
        n_red,
        n_green,
    }
}


fn parse_game(input_value: &str) -> Game {
    // input_value has format:
    // <game_number>: <game_result>;<game_result>;...
    let game_number = parse_int(input_value.split(":").next().unwrap()).unwrap();

    let game_data = input_value[input_value.find(':').unwrap() + 1..].trim();
    let game_strings:Vec<&str> = game_data.split(";").collect();
    let game_result: Vec<GameResult> = game_strings.iter().map(|s| parse_game_result(s)).collect();
    Game {
        game_number,
        game_result,
    }
}

struct GameResult {
    n_blue: u32,
    n_red: u32,
    n_green: u32,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameResult: {} green, {} red, {} blue",
            self.n_green, self.n_red, self.n_blue)
    }
}

struct Game {
    game_number: u32,
    game_result: Vec<GameResult>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game: {}\n", self.game_number)?;
        for result in &self.game_result {
            write!(f, "{}\n", result)?;
        }
        Ok(())
    }
}

impl Game {
    fn id_when_possible(&self,  n_red: u32, n_green: u32, n_blue: u32) -> u32 {
        // return the Game.id if none of the GameResults have more cubes
        // than the input values.
        for result in &self.game_result {
            if result.n_blue > n_blue || result.n_red > n_red || result.n_green > n_green {
                return 0;
            }
        }
        return self.game_number;
    }
}
