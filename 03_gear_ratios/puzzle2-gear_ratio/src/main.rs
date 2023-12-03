use std::fmt;
use std::fs;

const TEST_SCHEMATIC: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";



const DATA_PATH: &str = "../../data/03_engine_schematic.txt";

struct Gear {
    id: u32,
    location: (usize, usize),
    neighbors: Vec<u32>,
}


fn main() {
    // read data from file
    let input_text = fs::read_to_string(DATA_PATH)
        .expect("Error reading file");

    // let schematic = Schematic::new(TEST_SCHEMATIC);
    let schematic = Schematic::new(&input_text);


    // find all '*' locations in string
    let mut gears: Vec<Gear> = Vec::new();
    let mut gear_id: u32 = 0;

    for line_number in 0..schematic.lines.len() {
        let line = schematic.get_line(line_number);
        for (index, character) in line.line.chars().enumerate() {
            if character == '*' {
                let mut neighbors: Vec<u32> = Vec::new();

                // collect digits to the left
                if index > 0 {
                    if let Some(left_number) = extract_number(&line.line, index - 1) {
                        neighbors.push(left_number);
                    }
                }

                // collect digits to the right
                if index < line.line.len() {
                    if let Some(right_number) = extract_number(&line.line, index + 1) {
                        neighbors.push(right_number);
                    }
                }

                // collect digits above (this could be two)
                if line.previous_line.chars().nth(index).unwrap().is_digit(10) {
                    // single digit above
                    if let Some(above_center_number) = extract_number(&line.previous_line, index) {
                        neighbors.push(above_center_number);
                    }
                } else {
                    // possible split digits above left/right
                    if let Some(above_left_number) = extract_number(&line.previous_line, index - 1) {
                        neighbors.push(above_left_number);
                    }
                    if let Some(above_right_number) = extract_number(&line.previous_line, index + 1) {
                        neighbors.push(above_right_number);
                    }

                }

                // collect digits below (this could be two)
                if line.next_line.chars().nth(index).unwrap().is_digit(10) {
                    // single digit above
                    if let Some(above_center_number) = extract_number(&line.next_line, index) {
                        neighbors.push(above_center_number);
                    }
                } else {
                    // possible split digits above left/right
                    if let Some(above_left_number) = extract_number(&line.next_line, index - 1) {
                        neighbors.push(above_left_number);
                    }
                    if let Some(above_right_number) = extract_number(&line.next_line, index + 1) {
                        neighbors.push(above_right_number);
                    }

                }

                // create gear object
                gears.push(Gear {
                    id: gear_id,
                    location: (line_number, index),
                    neighbors,
                });

                // increment gear id
                gear_id += 1;
            }
        }
    }

    // print gears
    for gear in gears.iter() {
        println!("Gear {} at {:?} with neighbors {:?}", gear.id, gear.location, gear.neighbors);
    }
    println!("Number of gears: {}", gears.len());

    // select gears with exactly 2 neighbors
    let selected_gears: Vec<&Gear> = gears.iter().filter(|x| x.neighbors.len() == 2).collect();

    // print selected gears
    for gear in selected_gears.iter() {
        println!("Selected gear {} at {:?} with neighbors {:?}", gear.id, gear.location, gear.neighbors);
    }
    println!("Number of selected gears: {}", selected_gears.len());

    // calculate sum of gear ratios
    let sum_gear_ratios: u32 = selected_gears.iter().map(|x| x.neighbors[0] * x.neighbors[1] ).sum();

    println!("Sum of gear ratios: {}", sum_gear_ratios);

}

struct ExtractNumberError;

fn extract_number(text: &str, index: usize) -> Option<u32> {
    // return error if index is not a digit
    if !text.chars().nth(index).unwrap().is_digit(10) {
        return None;
    }

    // get starting index
    let mut start_index = index;
    while start_index > 0 && text.chars().nth(start_index - 1).unwrap().is_digit(10) {
        start_index -= 1;
    }

    // get ending index
    let mut end_index = index;
    while end_index < text.len() && text.chars().nth(end_index).unwrap().is_digit(10) {
        end_index += 1;
    }

    // extract number
    let number_string = &text[start_index..end_index];

    // return number
    Some(number_string.parse::<u32>().unwrap())
}


struct SchematicLine {
    previous_line: String,
    line: String,
    next_line: String,
}

impl fmt::Display for SchematicLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "prev: {}\nline: {}\nnext: {}\n", self.previous_line, self.line, self.next_line)
    }
}

struct Schematic<'a> {
    lines: Vec<&'a str>,
}

impl Schematic<'_> {
    pub fn new(input_text: &str) -> Schematic {
        let lines: Vec<&str> = input_text.split('\n').filter(|x| x.len() > 0).collect();

        Schematic { lines: lines.clone() }
    }

    pub fn get_line(&self, line_number: usize) -> SchematicLine {
        if line_number >= self.lines.len() {
            panic!("Line number {} out of range", line_number);
        }

        let space_line = String::from_utf8(vec![b' '; self.lines[0].len()]).unwrap();

        let previous_line = if line_number == 0 {
            space_line.clone()
        } else {
            self.lines[line_number - 1].to_string()
        };

        let next_line = if line_number == self.lines.len() - 1 {
            space_line.clone()
        } else {
            self.lines[line_number + 1].to_string()
        };

        SchematicLine {
            previous_line,
            line: self.lines[line_number].to_string(),
            next_line,
        }
    }

}

