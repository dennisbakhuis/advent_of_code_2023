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


const EXPANSION_FACTOR: usize = 1_000_000;


fn find_galaxies(input_data: &str) -> Vec<(usize, usize)> {
    let mut locations = Vec::new();

    for (row_idx, line) in input_data.lines().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            if char == '#' {
                locations.push((row_idx, col_idx));
            }
        }
    }

    locations
}


fn rows_and_columns_without_galaxies(input_data: &str) -> (Vec<usize>, Vec<usize>) {
    let mut rows_without_galaxies = Vec::new();
    let mut cols_without_galaxies = Vec::new();

    let rows: Vec<&str> = input_data.lines().collect();
    let num_cols = rows[0].len();

    for (row_idx, row) in rows.iter().enumerate() {
        if !row.contains('#') {
            rows_without_galaxies.push(row_idx);
        }
    }

    for col_idx in 0..num_cols {
        let mut has_galaxy = false;
        for row in &rows {
            if row.chars().nth(col_idx).unwrap() == '#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            cols_without_galaxies.push(col_idx);
        }
    }

    (rows_without_galaxies, cols_without_galaxies)
}


fn calculate_distances(
    points: &Vec<(usize, usize)>,
    zero_rows: &Vec<usize>,
    zero_cols: &Vec<usize>,
) -> Vec<usize> {
    let len = points.len();
    let mut distances: Vec<usize> = Vec::new();

    for i in 0..len {
        for j in (i + 1)..len {
            let p1 = points[i];
            let p2 = points[j];

            let x1 = std::cmp::min(p1.0, p2.0);
            let x2 = std::cmp::max(p1.0, p2.0);
            let y1 = std::cmp::min(p1.1, p2.1);
            let y2 = std::cmp::max(p1.1, p2.1);

            // calculate Manhattan distance
            let mut dist = (x2 - x1) + (y2 - y1);

            // add expansions between points
            for zero_row in zero_rows {
                if *zero_row > x1 && *zero_row < x2 {
                    dist += EXPANSION_FACTOR - 1;
                }
            }

            for zero_col in zero_cols {
                if *zero_col > y1 && *zero_col < y2 {
                    dist += EXPANSION_FACTOR - 1;
                }
            }

            distances.push(dist);
        }
    }

    distances
}


fn main() {
    // remove empty from string
    let input_data = INPUT_DATA.trim();

    // get galaxies
    let galaxies = find_galaxies(input_data);

    // get rows and columns without galaxies
    let (rows_without_galaxies, cols_without_galaxies) = rows_and_columns_without_galaxies(input_data);

    // calculate distances
    let distances = calculate_distances(
        &galaxies,
        &rows_without_galaxies,
        &cols_without_galaxies,
    );

    // sum of all distances
    let sum_of_distances: usize = distances.iter().sum();

    println!("sum of all distances: {}", sum_of_distances);
}
