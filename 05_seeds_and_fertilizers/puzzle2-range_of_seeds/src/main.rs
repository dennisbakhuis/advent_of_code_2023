use regex::Regex;
use std::cmp;

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

    // extract initial seeds
    let first_line = input_data
        .next()
        .unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let initial_seeds: Vec<i64> = re.find_iter(first_line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();

    // println!("input: {:?}", initial_seeds);

    // group values in pairs as (<start>, <end>)
    let seed_ranges: Vec<(i64, i64)> = initial_seeds.chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] ))
        .collect();

    // println!("input_ranges: {:?}", seed_ranges);

    // extract all mappings
    let mut range_mappings: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    while let Some(map_data) = input_data.next() {
        if map_data.is_empty() {
            continue;
        }

        // convert map_data to lines and only select lines starting with numbers
        let range_lines: Vec<&str> = map_data.lines()
            .filter(|line| line.starts_with(char::is_numeric))
            .collect();

        let mut tuples: Vec<(i64, i64, i64)> = Vec::new();
        for range_line in range_lines {
            let map_line: Vec<i64> = range_line.split_whitespace()
                .filter_map(|digits| digits.parse().ok())
                .collect();

            tuples.push((map_line[0], map_line[1], map_line[2]));
        }

        // store tuple
        range_mappings.push(tuples);
    }

    let mut minimum: Vec<i64> = Vec::new();

    for seed_range in seed_ranges.iter() {
        let mut current_seed_ranges: Vec<(i64, i64)> = Vec::new();
        current_seed_ranges.push(*seed_range);

        for range_map in range_mappings.iter() {
            current_seed_ranges = process_range_map(&current_seed_ranges, range_map);
        }

        minimum.push(current_seed_ranges.iter().min().unwrap().0);

    }

    println!("Minimum: {:?}", minimum.iter().min().unwrap());
    
}


fn process_range_map(
    seed_ranges: &Vec<(i64, i64)>,
    range_mapping: &Vec<(i64, i64, i64)>,
) -> Vec<(i64, i64)> {
    let mut current_seed_ranges: Vec<(i64, i64)> = seed_ranges.clone();
    let mut seeds_transform: Vec<(i64, i64)> = Vec::new();
    let mut new_seed_ranges: Vec<(i64, i64)> = Vec::new();

    for (target, source_start, range) in range_mapping.iter() {
        let source_end = source_start + range;
        while !current_seed_ranges.is_empty() {
            let (seed_start, seed_end): (i64, i64) = current_seed_ranges.pop().unwrap();

            let seed_left = (seed_start, cmp::min(seed_end, *source_start));
            if seed_left.1 > seed_left.0 {
                new_seed_ranges.push(seed_left);
            }

            let seed_center = (
                cmp::max(seed_start, *source_start),
                cmp::min(source_end, seed_end),
            );
            if seed_center.1 > seed_center.0 {
                let new_seed = (
                    seed_center.0 + target - source_start,
                    seed_center.1 + target - source_start,
                );
                seeds_transform.push(new_seed);
            }

            let seed_right = (cmp::max(source_end, seed_start), seed_end);
            if seed_right.1 > seed_right.0 {
                new_seed_ranges.push(seed_right);
            }
        }

        // move items to new_seed_ranges
        while !new_seed_ranges.is_empty() {
            current_seed_ranges.push(new_seed_ranges.pop().unwrap());
        }

    }
    // combine seeds_overlap with current_seed_ranges
    for item in seeds_transform {
        current_seed_ranges.push(item);
    }

    // current_seed_ranges
    current_seed_ranges
}

