
const INPUT_DATA: &str = include_str!("../../../data/09_oasis_readings.txt");
// // const INPUT_DATA: &str = "\
// // 0 3 6 9 12 15
// // 1 3 6 10 15 21
// // 10 13 16 21 30 45
// // ";
// const INPUT_DATA: &str = "\
// 12 28 47 77 148 320 690 1410 2740 5170 9648 17942 33138 60228 106667 182671 300884 474856 715541 1024739 1384064
// ";

fn main() {
    // Non-empty lines
    let lines = INPUT_DATA.lines().filter(|line| !line.is_empty());

    // Parse numbers
    let value_histories: Vec<Vec<i64>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // Process each value_history
    let mut predictions: Vec<i64> = Vec::new();
    for value_history in value_histories {
        let mut last_values: Vec<i64> = vec![value_history.last().unwrap().clone()];
        let mut current_values: Vec<i64> = value_history.clone();

        while !is_all_zero(&current_values) {
            current_values = current_values
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();

            last_values.push(current_values.last().unwrap().clone());
        }

        let prediction = last_values.iter().sum::<i64>();
        predictions.push(prediction);

    }

    let answer = predictions.iter().sum::<i64>();
    println!("answer: {}", answer);
}

fn is_all_zero(values: &Vec<i64>) -> bool {
    values.iter().all(|value| *value == 0)
}

