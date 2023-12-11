const INPUT_DATA: &str = include_str!("../../../data/11_galaxy_map.txt");
// const INPUT_DATA: &str = "\
// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....
// ";


fn parse_map(input: &str) -> Vec<Vec<usize>> {
    let mut next_number = 1;
    input
        .lines()
        .map(|line| {
            let row: Vec<usize> = line
                .chars()
                .map(|ch| {
                    let value = if ch == '.' {
                        0
                    } else {
                        let number = next_number;
                        next_number += 1;
                        number
                    };
                    value
                })
                .collect();
            row
        })
        .collect()
}


fn expand_vertically(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut expanded: Vec<Vec<usize>> = Vec::new();
    for row in 0..map.len() {
        let sum_of_row: usize = map[row].iter().sum();
        if sum_of_row == 0 {
            expanded.push(map[row].to_vec());
            expanded.push(map[row].to_vec());
        } else {
            expanded.push(map[row].to_vec());
        }
    }
    expanded
}


fn transpose_map(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    if map.is_empty() {
        return Vec::new();
    }

    let num_rows = map.len();
    let num_cols = map[0].len();

    (0..num_cols)
        .map(|col| (0..num_rows).map(|row| map[row][col]).collect())
        .collect()
}


fn expand_universe(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let expanded_vertically = expand_vertically(map);
    let expanded_horizontally = expand_vertically(&transpose_map(&expanded_vertically));
    transpose_map(&expanded_horizontally)
}


fn extract_galaxies(map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] != 0 {
                galaxies.push((row, col));
            }
        }
    }
    galaxies
}


fn manhattan_distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    let dx = if p1.0 > p2.0 { p1.0 - p2.0 } else { p2.0 - p1.0 };
    let dy = if p1.1 > p2.1 { p1.1 - p2.1 } else { p2.1 - p1.1 };

    dx + dy
}


fn calculate_distances(points: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let len = points.len();
    let mut distances = vec![vec![0; len]; len];

    for i in 0..len {
        for j in (i + 1)..len {
            let dist = manhattan_distance(&points[i], &points[j]);
            distances[i][j] = dist;
        }
    }

    distances
}


fn main() {
    // remove empty from string
    let input_data = INPUT_DATA.trim();

    let universe_map = parse_map(input_data);
    let universe = expand_universe(&universe_map);

    // // print universe
    // for row in 0..universe.len() {
    //     println!("{:?}", universe[row]);
    // }

    // get all galaxies
    let galaxies = extract_galaxies(&universe);

    // // print galaxies
    // for galaxy in galaxies.iter() {
    //     println!("{:?}", galaxy);
    // }

    // calculate distances
    let distances = calculate_distances(&galaxies);
    for row in 0..distances.len() {
        println!("{:?}", distances[row]);
    }

    // sum of all distances
    let sum_of_distances: usize = distances
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum::<usize>();

    println!("sum of all distances: {}", sum_of_distances);

}
