use std::collections::HashMap;
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
    let seeds: Vec<u32> = re.find_iter(first_line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();

    println!("seeds: {:?}", seeds);

    // extract all maps
    let mut maps: Vec<HashMap<u32, u32>> = Vec::new();
    while let Some(map_data) = input_data.next() {
        if map_data.is_empty() {
            continue;
        }

        // convert map_data to lines and only select lines starting with numbers
        let map_lines: Vec<&str> = map_data.lines()
            .filter(|line| line.starts_with(char::is_numeric))
            .collect();

        // iterate over map_lines and extract the map data
        let mut map: HashMap<u32, u32> = HashMap::new();

        for map_line in map_lines {
            let map_line: Vec<u32> = map_line.split_whitespace()
                .filter_map(|digits| digits.parse().ok())
                .collect();


            let destination_start = map_line[0];
            let source_start = map_line[1];
            let range = map_line[2];

            // create map
            for i in 0..range {
                map.insert(source_start + i, destination_start + i);
            }
        }

        // store map
        maps.push(map);
    }

    // apply maps to seeds
    let mut source = seeds.clone();

    for map in maps {
        println!("step...");

        let mut destination: Vec<u32> = Vec::new();
        for src in source.iter() {
            let mut destination_value = *src;
            if let Some(new_seed) = map.get(&(*src)) {
                destination_value = *new_seed as u32;
            }
            destination.push(destination_value);
        }
        source = destination;
    }

    println!("Final seeds: {:?}", source);
    println!("Minimum seed: {}", source.iter().min().unwrap());
}
