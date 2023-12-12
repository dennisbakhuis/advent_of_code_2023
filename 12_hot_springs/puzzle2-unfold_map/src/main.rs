const INPUT_DATA: &str = include_str!("../../../data/12_springs_map.txt");
// const INPUT_DATA: &str = "\
// ???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1
// ";
// const INPUT_DATA: &str = "\
// ?. 1
// ?.# 1
// #?. 2
// ";


#[derive(Debug)]
struct SpringData {
    spring: String,
    groups: Vec<usize>,
}

impl SpringData {
    pub fn new(raw_input_row: &str, fold_factor: usize) -> Self {
        let (mut spring, groups_string) = if let Some((p1, p2)) = raw_input_row.split_once(" ") {
            (p1.to_string(), p2.to_string())
        } else {
            panic!("Invalid input row: {}", raw_input_row);
        };

        // parse groups to Vec<usize>
        let mut groups = groups_string.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        if fold_factor > 1 {
            spring = (1..fold_factor).fold(spring.clone(), |s, _| s + "?" + &spring );
            groups = groups.repeat(fold_factor);
        }

        Self {
            spring,
            groups,
        }
    }
}


fn calculate_possibilities(spring: &SpringData) -> usize {
    let characters = spring.spring.chars().collect::<Vec<char>>();
    let n_characters = characters.len();

    let groups = &spring.groups;
    let n_numbers = groups.len();

    // create a 3d structure to hold results of zeros
    // make one larger to let the algorithm with pos+1 etc. work
    let mut count_table = vec![vec![vec![0; n_characters + 1]; n_numbers + 1]; n_characters + 1];
    count_table[n_characters][n_numbers][0] = 1;
    count_table[n_characters][n_numbers - 1][groups[n_numbers - 1]] = 1;

    // Main loop
    for character_ix in (0..n_characters).rev() {  // iterate the characters backwards
        for (group_ix, &group_size) in groups.iter().enumerate() {
            for value in 0..=group_size {

                if characters[character_ix] == '.' || characters[character_ix] == '?' {
                    if value == 0 {
                        // add value from previous processed character to this one
                        count_table[character_ix][group_ix][value] += count_table[character_ix + 1][group_ix][0];
                    } else if group_ix < n_numbers && groups[group_ix] == value {
                        count_table[character_ix][group_ix][value] += count_table[character_ix + 1][group_ix + 1][0];
                    }
                }

                if characters[character_ix] == '#' || characters[character_ix] == '?' {
                    count_table[character_ix][group_ix][value] += count_table[character_ix + 1][group_ix][value + 1];
                }
            }
        }

        if characters[character_ix] == '.' || characters[character_ix] == '?' {
            count_table[character_ix][n_numbers][0] += count_table[character_ix + 1][n_numbers][0];
        }
    }

    count_table[0][0][0]
}

fn main() {
    // import data
    let fold_factor = 1;
    let springs: Vec<SpringData> = INPUT_DATA.trim().lines().map(|l| SpringData::new(l, fold_factor)).collect();

    // process spings
    let mut total_possibilities: Vec<usize> = Vec::new();

    for spring in &springs {
        let possibilities = calculate_possibilities(spring);
        total_possibilities.push(possibilities);
        println!("{} {:?} -> {}", spring.spring, spring.groups, possibilities);
    }

    // get sum
    let total_possibilities: usize = total_possibilities.into_iter().sum();
    println!("Answer: {}", total_possibilities);

}

