use std::usize;
use std::collections::HashMap;


const INPUT_DATA: &str = include_str!("../../../data/10_pipe_maze.txt");
// const INPUT_DATA: &str = "\
// FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L
// ";

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    StartingPoint,
    None,
}

struct Map {
    map: String,
    line_width: usize,
    start: (usize, Direction),
}

impl Map {
    pub fn new(raw_map: &str) -> Map {
        let nx = raw_map.lines().next().unwrap().len();
        let ny = raw_map.lines().count();
        let map = raw_map.lines().collect::<String>();

        let current_index = map.find('S').unwrap();
        let current_direction = Map::get_initial_direction(raw_map, current_index);

        println!("Map dimensions: ({}, {})", nx, ny);
        println!("Map number of characters: {}", map.len());
        println!("Start: ({}, {})", current_index % nx, current_index / nx);
        println!("Start direction: {:?}", current_direction);

        Map {
            map,
            line_width: nx,
            start: (current_index, current_direction),
        }
    }

    fn get_initial_direction(raw_map: &str, current_index: usize) -> Direction {
        let nx = raw_map.lines().next().unwrap().len();
        let ny = raw_map.lines().count();
        let map = raw_map.lines().collect::<String>();

        let connect_above = current_index / nx > 0 && "|7F".contains(map.chars().nth(current_index - nx).unwrap());
        let connect_below = current_index / nx < ny && "|JL".contains(map.chars().nth(current_index + nx).unwrap());
        let connect_left = current_index % nx > 0 && "-LF".contains(map.chars().nth(current_index - 1).unwrap());
        let connect_right = current_index % nx < nx  - 1 && "-7J".contains(map.chars().nth(current_index + 1).unwrap());

        if connect_above {
            Direction::Up
        } else if connect_right {
            Direction::Right
        } else if connect_below && connect_left {
            Direction::Down
        } else {
            panic!("No initial direction found");
        }
    }

    pub fn find_next(
        &self,
        current_index: &usize,
        current_direction: &Direction,
    ) -> (usize, Direction) {
        let new_index = match current_direction {
            Direction::Up => current_index - self.line_width,
            Direction::Down => current_index + self.line_width,
            Direction::Left => current_index - 1,
            Direction::Right => current_index + 1,
            Direction::StartingPoint => current_index.clone(),
            Direction::None => panic!("No next found"),
        };

        let current_char: &str = self.map.get(new_index..new_index + 1).unwrap();

        let new_direction = match current_char {
            "-" => if current_direction == &Direction::Right { Direction::Right} else { Direction::Left },
            "|" => if current_direction == &Direction::Up { Direction::Up } else { Direction::Down },
            "F" => if current_direction == &Direction::Up { Direction::Right } else { Direction::Down },
            "7" => if current_direction == &Direction::Right { Direction::Down } else { Direction::Left },
            "L" => if current_direction == &Direction::Left { Direction::Up } else { Direction::Right },
            "J" => if current_direction == &Direction::Right { Direction::Up } else { Direction::Left },
            "S" => Direction::StartingPoint,
            _ => panic!("No next found"),
        };
        (new_index, new_direction)
    }

    pub fn get_vertical_stroke_direction(
        &self,
        current_index: &usize,
        current_direction: &Direction,
    ) -> Direction {
        let ci = current_index.clone();
        let current_char: &str = self.map.get(ci..ci + 1).unwrap();

        match current_char {
            "-" => Direction::None,
            "|" => if current_direction == &Direction::Up { Direction::Up } else { Direction::Down },
            "F" => if current_direction == &Direction::Right { Direction::Up } else { Direction::Down },
            "7" => if current_direction == &Direction::Down { Direction::Down } else { Direction::Up },
            "L" => if current_direction == &Direction::Up { Direction::Up } else { Direction::Down },
            "J" => if current_direction == &Direction::Left { Direction::Down } else { Direction::Up },
            "S" => current_direction.clone(),
            _ => panic!("No next found"),
        }
    }

}

fn main() {
    let map = Map::new(INPUT_DATA);
    let mut direction_map: HashMap<usize, Direction> = HashMap::new(); // for part b

    // part a
    let mut current_index = map.start.0;
    let mut current_direction = map.start.1;
    let mut steps = 0;
    direction_map.insert(current_index, current_direction); // for part b

    while current_direction != Direction::StartingPoint {
        let (new_index, new_direction) = map.find_next(&current_index, &current_direction);
        current_index = new_index;
        current_direction = new_direction;
        steps += 1;
        // println!("Coords: ({}, {}) -> {:?}", current_index % map.nx, current_index / map.nx, current_direction);
        direction_map.insert(current_index, current_direction); // for part b
    }

    println!("Answer part a: {}", steps / 2);

    // part b using the non-zero unwinding rule
    let mut stroke_direction = Direction::StartingPoint;
    let mut nodes_inside = 0;
    let mut updown: Vec<char> = Vec::new();

    for ix in 0..map.map.len() {
        if direction_map.contains_key(&ix) {
            let map_direction = direction_map.get(&ix).unwrap();

            let current_stroke_direction = map.get_vertical_stroke_direction(&ix, &map_direction);

            if current_stroke_direction == Direction::Up {
                stroke_direction = Direction::Up;
                updown.push('↑');
            } else if current_stroke_direction == Direction::Down {
                stroke_direction = Direction::Down;
                updown.push('↓');
            } else {
                updown.push('.');
            }
        } else {
            if stroke_direction == Direction::Down {
                nodes_inside += 1;
                updown.push('o');
            } else {
                updown.push('.');
            }
        }
    }

    // create string from updown as map with dimensions nx, ny
    let mut map_string = String::new();
    for (ix, character) in updown.iter().enumerate() {
        map_string.push(*character);
        if ix % map.line_width == map.line_width - 1 {
            map_string.push('\n');
        }
    }
    println!("{}", map_string);

    println!("Answer part b: {}", nodes_inside);
}

