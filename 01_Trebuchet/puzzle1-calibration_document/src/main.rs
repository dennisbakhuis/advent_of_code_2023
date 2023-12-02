use std::fs;

// Path to input file
const DATA_PATH: &str = "../../data/01_calibration_document.txt";


// Function to extract first and last digit from string
fn extract_numbers(item: &str) -> u32 {
    // find first digit in item
    let first_char: Option<char> = item.chars()
        .find(|c| c.is_digit(10));

    // find last digit in item
    let last_char: Option<char> = item.chars().rev()
        .find(|c| c.is_digit(10));

    // combine the first and last digit items as string if found
     match (first_char, last_char) {
        (Some(f), Some(l)) => (f.to_string() + &l.to_string()).parse().unwrap(),
        _ => 0,
    }
}


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
