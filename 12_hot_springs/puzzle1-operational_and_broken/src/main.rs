const INPUT_DATA: &str = include_str!("../../../data/12_springs_map.txt");
// const INPUT_DATA: &str = "\
// ???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1
// ";


#[derive(Debug)]
struct Row {
    spring: String,
    groups: String,
}

impl Row {
    pub fn new(raw_input_row: &str) -> Self {
        let (spring, groups) = if let Some((p1, p2)) = raw_input_row.split_once(" ") {
            (p1.to_string(), p2.to_string())
        } else {
            panic!("Invalid input row: {}", raw_input_row);
        };

        Self {
            spring,
            groups,
        }
    }
}


fn generate_combinations(input: &str) -> Vec<String> {
    let mut combinations = Vec::new();
    generate_combinations_helper(input, 0, String::new(), &mut combinations);
    combinations
}

fn generate_combinations_helper(input: &str, index: usize, current: String, combinations: &mut Vec<String>) {
    if index == input.len() {
        combinations.push(current);
        return;
    }

    let current_char = input.chars().nth(index).unwrap();
    if current_char == '?' {
        let mut option1 = current.clone();
        option1.push('.');
        generate_combinations_helper(input, index + 1, option1, combinations);

        let mut option2 = current.clone();
        option2.push('#');
        generate_combinations_helper(input, index + 1, option2, combinations);
    } else {
        let mut unchanged = current.clone();
        unchanged.push(current_char);
        generate_combinations_helper(input, index + 1, unchanged, combinations);
    }
}

fn count_and_sort_groups(possibility: &str) -> String {
    let mut group_counts = Vec::new();
    let mut current_count = 0;

    for c in possibility.chars() {
        if c == '#' {
            current_count += 1;
        } else if current_count > 0 {
            group_counts.push(current_count);
            current_count = 0;
        }
    }

    if current_count > 0 {
        group_counts.push(current_count);
    }

    group_counts
        .into_iter()
        .map(|count| count.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn count_occurrences(input_vec: Vec<String>, target_string: &str) -> usize {
    input_vec.iter().filter(|&s| s == target_string).count()
}

fn main() {
    // import data
    let springs: Vec<Row> = INPUT_DATA.trim().lines().map(|l| Row::new(l)).collect();

    for spring in &springs {
        println!("{:?}", spring);
    }

    // get first spring
    let mut possible_combinations: Vec<usize> = Vec::new();
    for spring in &springs {

        // generate all possibilities and return combination string
        let combinations = generate_combinations(&spring.spring).iter()
            .map(|c| count_and_sort_groups(&c))
            .collect::<Vec<String>>();

        // count occurrences of required string
        let occurrences = count_occurrences(combinations, &spring.groups);

        possible_combinations.push(occurrences);
    }

    // print answer
    let sum_of_possibilities: usize = possible_combinations.into_iter().sum();
    println!("Answer: {}", sum_of_possibilities);

}
