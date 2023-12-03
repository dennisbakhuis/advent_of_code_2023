use std::fmt;
use std::fs;


const DATA_PATH: &str = "../../data/03_engine_schematic.txt";


fn main() {
    // read data from file
    let input_text = fs::read_to_string(DATA_PATH)
        .expect("Error reading file");

    // let schematic = Schematic::new(TEST_SCHEMATIC2);
    let schematic = Schematic::new(&input_text);

    // get part numbers and sum
    let part_numbers = schematic.get_part_numbers();
    let sum_part_numbers: u32 = part_numbers.iter().sum();

    println!("Symbols: {}", schematic.symbols);
    println!("Sum of part numbers: {}", sum_part_numbers);
}


fn is_surrounded_by_symbol(line: &SchematicLine, number: &NumberPart, selected_characters: &str) -> bool {
    // Get the symbols around a string of numbers

    // string indices
    let string_start = number.start_index;
    let string_end = number.end_index;

    // check symbol left
    if string_start > 0 {
        if selected_characters.contains(line.line.chars().nth(string_start - 1).unwrap()) {
            return true;
        }
    }

    // symbol right
    if string_end < line.line.len() {
        if selected_characters.contains(line.line.chars().nth(string_end).unwrap()) {
            return true;
        }
    }

    // symbols above/below
    let index_start = if string_start > 0 { string_start - 1 } else { 0 };
    let index_end = if string_end < line.line.len() { string_end + 1 } else { line.line.len() };
    for c in line.previous_line[index_start..index_end].chars() {
        if selected_characters.contains(c) {
            return true;
        }
    }
    for c in line.next_line[index_start..index_end].chars() {
        if selected_characters.contains(c) {
            return true;
        }
    }

    return false;
}


#[derive(Debug)]
struct NumberPart {
    number: String,
    start_index: usize,
    end_index: usize,
}

impl fmt::Display for NumberPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}, {})", self.number, self.start_index, self.end_index)
    }
}

fn find_numbers(input_text: &str) -> Vec<NumberPart> {
    // find numbers in line, separated by periods or symbols
    let mut numbers: Vec<NumberPart> = Vec::new();
    let mut current_number: Vec<char> = Vec::new();

    for (index, character) in input_text.chars().enumerate() {
        if character.is_digit(10) {
            current_number.push(character);
        } else {
            if current_number.len() > 0 {
                let number_string: String = current_number.iter().collect();
                let number_part = NumberPart {
                    number: number_string,
                    start_index: index - current_number.len(),
                    end_index: index,
                };
                numbers.push(number_part);
                current_number.clear();
            }
        }
    }
    if current_number.len() > 0 {
        let number_string: String = current_number.iter().collect();
        let number_part = NumberPart {
            number: number_string,
            start_index: input_text.len() - current_number.len(),
            end_index: input_text.len(),
        };
        numbers.push(number_part);
    }
    numbers
}


fn get_unique_symbols(input_text: &str) -> String {
    // Find all unique characters in the input text
    let mut unique_chars: Vec<char> = Vec::new();
    for c in input_text.chars() {
        // add character if it's not already in the vector, not is a digit or period
        if !c.is_digit(10) && c != '.' && c != '\n' {
            if !unique_chars.contains(&c) {
                unique_chars.push(c);
            }
        }
    }
    unique_chars.iter().collect()
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
    symbols: String,
    _space_line: String,
}

impl Schematic<'_> {
    pub fn new(input_text: &str) -> Schematic {
        let lines: Vec<&str> = input_text.split('\n').filter(|x| x.len() > 0).collect();
        let symbols = get_unique_symbols(input_text);
        let space_line = String::from_utf8(vec![b' '; lines[0].len()]).unwrap();

        Schematic { lines: lines.clone(), symbols , _space_line: space_line}
    }

    pub fn get_line(&self, line_number: usize) -> SchematicLine {
        if line_number >= self.lines.len() {
            panic!("Line number {} out of range", line_number);
        }

        let previous_line = if line_number == 0 {
            self._space_line.clone()
        } else {
            self.lines[line_number - 1].to_string()
        };

        let next_line = if line_number == self.lines.len() - 1 {
            self._space_line.clone()
        } else {
            self.lines[line_number + 1].to_string()
        };

        SchematicLine {
            previous_line,
            line: self.lines[line_number].to_string(),
            next_line,
        }
    }

    pub fn get_part_numbers(&self) -> Vec<u32> {
        let mut part_numbers: Vec<u32> = Vec::new();
        let number_of_lines = self.lines.len();

        for i in 0..number_of_lines {
            let line = self.get_line(i);
            let numbers = find_numbers(&line.line);
            for number in numbers {
                if is_surrounded_by_symbol(&line, &number, &self.symbols) {
                    part_numbers.push(number.number.parse::<u32>().unwrap());
                }
            }
        }

        part_numbers
    }
}

