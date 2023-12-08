use std::collections::HashMap;
use num::Integer;


const INPUT_DATA: &str = include_str!("../../../data/08_haunted_desert_map.txt");
// const INPUT_DATA: &str = "\
// RL
//
// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG) DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
// ";
//
// const INPUT_DATA: &str = "\
// LLR
//
// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
// ";

// const INPUT_DATA: &str = "\
// LR
//
// 11A = (11B, XXX)
// 11B = (XXX, 11Z)
// 11Z = (11B, XXX)
// 22A = (22B, XXX)
// 22B = (22C, 22C)
// 22C = (22Z, 22Z)
// 22Z = (22B, 22B)
// XXX = (XXX, XXX)
// ";
//

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

    // Starting data structure, all nodes that end with Z
    let current_location: Vec<&str> = mapping
        .iter()
        .filter(|(key, _)| key.ends_with("A"))
        .map(|(key, _)| *key)
        .collect();

    // get shortest path for each node individually
    let shortest_paths: Vec<usize> = current_location
        .iter()
        .filter_map(|node| get_shortest_path(node, left_right, &mapping, 1))
        .collect();

    println!("Shortest paths do not align with each other but according to the puzzle eventually the must:");
    println!("Shortest paths: {:?}\n", shortest_paths);

    // check for periodicity
    let mut periods: Vec<Vec<usize>> = Vec::new();
    for cycles in 1..4 {
        let periods_with_cycles: Vec<usize> = current_location
            .iter()
            .filter_map(|node| get_shortest_path(node, left_right, &mapping, cycles))
            .collect();

        periods.push(periods_with_cycles);
    }

    println!("The pattern repeats itself during 3 cycles:");
    for (ix, _) in current_location.iter().enumerate() {
        let path_steps = periods.iter().map(|x| x[ix]).collect::<Vec<usize>>();
        let step_size1 = path_steps[1] - path_steps[0];
        let step_size2 = path_steps[2] - path_steps[1];

        println!("Path steps: {:?} -> step sizes: {}, {}", path_steps, step_size1, step_size2);
    }

    // get least common multiple
    let least_common_multiple = shortest_paths.iter().fold(1, |acc, x| Integer::lcm(&acc, x));
    println!("Least common multiple: {}", least_common_multiple);

}

fn get_shortest_path(
    starting_node: &str,
    direction: &str,
    mapping: &HashMap<&str, (&str, &str)>,
    cycles_until_found: usize,
) -> Option<usize> {
    let mut current_location = starting_node;
    let mut current_cycle = 1;

    for (ix, current_direction) in direction.chars().cycle().enumerate() {
        if current_location.ends_with("Z") {  // aparently this is also enough for puzzle 1
            if current_cycle == cycles_until_found {
                return Some(ix);
            }
            current_cycle += 1;
        }

        match current_direction {
            'L' => current_location = mapping.get(current_location).unwrap().0,
            'R' => current_location = mapping.get(current_location).unwrap().1,
            _ => panic!("Unexpected direction: {}", current_direction),
        }

    }

    None // do to cycle() this will never be reached
}

