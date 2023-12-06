// const INPUT_DATA: &str = "\
// Time:      7  15   30
// Distance:  9  40  200
// ";
const INPUT_DATA: &str = include_str!("../../../data/06_boat_race_times.txt");
const ACCELEATION: u64 = 1;


fn main() {
    // extract time and distance from data
    let first_line = INPUT_DATA.lines().next().unwrap();
    let second_line = INPUT_DATA.lines().nth(1).unwrap();
    let numbers_fl = first_line.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let numbers_sl = second_line.chars().filter(|c| c.is_digit(10)).collect::<String>();

    let (time, distance) = (
        numbers_fl.parse::<u64>().unwrap(),
        numbers_sl.parse::<u64>().unwrap(),
    );

    println!("Time: {}\nDistance: {}", time, distance);

    // calculate possibilities
    let distances: Vec<u64> = (0..time).map(|t| ACCELEATION * t * (time - t))
        .filter(|x| x > &distance)
        .collect();

    let possibilities = distances.len() as u64;

    println!("Answer: {}", possibilities);

}

