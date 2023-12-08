use std::collections::HashMap;


const INPUT_DATA: &str = include_str!("../../../data/08_haunted_desert_map.txt");
// const INPUT_DATA: &str = "\
// RL
//
// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
// ";

// const INPUT_DATA: &str = "\
// LLR
//
// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
// ";


fn main() {
    // get left_right instruction from first line
    let left_right = INPUT_DATA.lines().nth(0).unwrap();

    // Mapping features:
    // mapping.0 is unique in whole column

    // get mapping data from 2nd line to end discarding empty lines
    let mapping_data = INPUT_DATA.lines().skip(1).filter(|x| !x.is_empty());
    let mut mapping: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in mapping_data {
        mapping.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    // Starting data structure
    let mut current_location: Vec<Vec<&str>> = Vec::new();
    current_location.push(vec!["AAA"]);

    // Main loop
    let mut found = false;
    while !found {
        for current_direction in left_right.chars() {
            for current_path in current_location.iter_mut() {
                let current_location = current_path.last().unwrap();
                let (left, right) = mapping.get(current_location).unwrap();
                match current_direction {
                    'L' => current_path.push(left),
                    'R' => current_path.push(right),
                    _ => panic!("Unexpected direction: {}", current_direction),
                }

                if current_path.last().unwrap() == &"ZZZ" {
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }
    }

    // Get shortest path
    let shortest_path = current_location.iter().map(|x| x.len()).min().unwrap();

    println!("Shortest path: {}", shortest_path - 1);
}
