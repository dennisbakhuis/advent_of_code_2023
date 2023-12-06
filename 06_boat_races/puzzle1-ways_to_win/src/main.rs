use regex::Regex;


// const INPUT_DATA: &str = "\
// Time:      7  15   30
// Distance:  9  40  200
// ";
const INPUT_DATA: &str = include_str!("../../../data/06_boat_race_times.txt");
const ACCELEATION: u32 = 1;


fn main() {
    // extract time and distance from data
    let first_line = INPUT_DATA.lines().next().unwrap();
    let second_line = INPUT_DATA.lines().nth(1).unwrap();
    let re = Regex::new(r"(\d+)").unwrap();
    let (times, distances) = (
        re.captures_iter(first_line).map(|c| c[1].parse::<u32>().unwrap()).collect::<Vec<u32>>(),
        re.captures_iter(second_line).map(|c| c[1].parse::<u32>().unwrap()).collect::<Vec<u32>>()
    );
    let pairs = times.into_iter().zip(distances.into_iter()).collect::<Vec<(u32, u32)>>();

    println!("{:?}", pairs);

    
    // calculate possibilities
    let mut possibilities: Vec<u32> = Vec::new();

    for (time, distance) in pairs {
        let distances: Vec<u32> = (0..time).map(|t| ACCELEATION * t * (time - t))
            .filter(|x| x > &distance)
            .collect();
        possibilities.push(distances.len() as u32);
    }

    println!("{:?}", possibilities);

    // calculate answer
    let product = possibilities.into_iter().product::<u32>();

    println!("Answer: {}", product);

}

