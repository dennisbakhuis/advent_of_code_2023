use std::fs;
use std::collections::VecDeque;


// Path to input file
const DATA_PATH: &str = "../../data/01_calibration_document.txt";

// Digit values
const DIGIT_VALUES: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];


// Main function
fn main() {
    // Read input data
    let input_data = fs::read_to_string(DATA_PATH)
        .expect("Failed to read input data");

    // Get separate lines and discard empty ones
    let lines: Vec<&str> = input_data.split("\n")
        .filter(|s| !s.is_empty())
        .collect();

    // Extract numbers from lines
    let value_vec: Vec<u32> = lines.iter()
        .map(|s| extract_numbers(s))
        .collect();

    // sum the values
    let sum: u32 = value_vec.iter().sum();

    println!("Sum of all values: {}", sum);

}


fn extract_numbers(input_values: &str) -> u32 {
    let first_digit = get_first_number(input_values, false);
    let last_digit = get_first_number(input_values, true);

    // combine the first and last digit items as string if found
     match (first_digit, last_digit) {
        (Some(f), Some(l)) => (f.to_string() + &l.to_string()).parse().unwrap(),
        _ => 0,
    }
}


fn get_first_number(
    input_values: &str,
    reverse: bool,
) -> Option<char> {
    let mut characters = VecDeque::from([' ', ' ', ' ', ' ', ' ']);

    let value: Box<dyn Iterator<Item=char>> = if reverse {
        Box::new(input_values.chars().rev())
    } else {
        Box::new(input_values.chars())
    };

    for character in value {
        // check if character is digit
        if character.is_digit(10) {
            return Some(character.clone());
        }

        if reverse {
            // Backward //

            // shift characters
            characters.pop_back();
            characters.push_front(character);

            // check if characters as string ends with one of the DIGIT_VALUES
            let characters_as_string: String = characters.iter().collect();

            for (digit_string, digit_char) in DIGIT_VALUES.iter() {
                if characters_as_string.starts_with(digit_string) {
                    return Some(*digit_char);
                }
            }
        } else {
            // Forward //

            // shift characters
            characters.pop_front();
            characters.push_back(character);

            // check if characters as string ends with one of the DIGIT_VALUES
            let characters_as_string: String = characters.iter().collect();

            for (digit_string, digit_char) in DIGIT_VALUES.iter() {
                if characters_as_string.ends_with(digit_string) {
                    return Some(*digit_char);
                }
            }
        }
    }
    return None;
}
