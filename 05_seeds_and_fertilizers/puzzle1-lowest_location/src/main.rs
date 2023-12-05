use regex::Regex;

// test data
// const INPUT_DATA: &str = include_str!("../../../data/05_almanak_testdata.txt");
// real data
const INPUT_DATA: &str = include_str!("../../../data/05_almanak.txt");

// Format
// seeds: <number>, ....  # starting seed numbers
//
// <source>-to-<destination> map:
// <destination_start> <source_start> <range>
//

fn main() {
    // split input data by empty lines
    let mut input_data = INPUT_DATA.split("\n\n");

    // extract seeds
    let first_line = input_data
        .next()
        .unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let seeds: Vec<i64> = re.find_iter(first_line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();

    // extract all maps
    let mut tuple_sets: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    while let Some(map_data) = input_data.next() {
        if map_data.is_empty() {
            continue;
        }

        // convert map_data to lines and only select lines starting with numbers
        let range_lines: Vec<&str> = map_data.lines()
            .filter(|line| line.starts_with(char::is_numeric))
            .collect();

        // iterate over map_lines and extract the map data
        let mut tuples: Vec<(i64, i64, i64)> = Vec::new();

        for range_line in range_lines {
            let map_line: Vec<i64> = range_line.split_whitespace()
                .filter_map(|digits| digits.parse().ok())
                .collect();


            let destination_start = map_line[0];
            let source_start = map_line[1];
            let range = map_line[2];

            tuples.push((source_start, destination_start, range));
        }

        // store map
        tuple_sets.push(tuples);
    }

    // Move seeds through all ranges
    let mut source = seeds.clone();

    for tuples in tuple_sets {
        let mut destination: Vec<i64> = Vec::new();

        for src in source.iter() {
            let mut destination_value = *src;

            // check if src is in ranges
            for (source_start, destination_start, range) in tuples.iter() {
                if *source_start <= *src && *src < *source_start + *range {
                    destination_value = *destination_start + (*src - *source_start);
                    break;
                }
            }

            destination.push(destination_value);
        }
        source = destination;
    }

    println!("Minimum seed: {}", source.iter().min().unwrap());
}
