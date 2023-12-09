
const INPUT_DATA: &str = include_str!("../../../data/09_oasis_readings.txt");
// const INPUT_DATA: &str = "\
// 0 3 6 9 12 15
// 1 3 6 10 15 21
// 10 13 16 21 30 45
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
        let mut first_values: Vec<i64> = vec![value_history.first().unwrap().clone()];
        let mut current_values: Vec<i64> = value_history.clone();

        while !is_all_zero(&current_values) {
            current_values = current_values
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();

            first_values.push(current_values.first().unwrap().clone());
        }



        println!("first_values: {:?}", first_values);

        
        let mut prediction = 0;
        let n_val = first_values.len();
        for ix in 1..n_val + 1 {
            prediction = first_values[n_val - ix] - prediction;
        }

        predictions.push(prediction);
    }

    let answer = predictions.iter().sum::<i64>();
    println!("answer: {}", answer);
}

fn is_all_zero(values: &Vec<i64>) -> bool {
    values.iter().all(|value| *value == 0)
}

