// const INPUT_DATA: &str = include_str!("../../../data/10_pipe_maze.txt");
const INPUT_DATA: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

struct Map {
    map: String,
    nx: usize,
    ny: usize,
}

impl Map {
    pub fn new(raw_map: &str) -> Map {
        let nx = raw_map.lines().next().unwrap().len();
        let ny = raw_map.lines().count();
        let map = raw_map.lines().collect::<String>();

        println!("Map dimensions: ({}, {})", nx, ny);
        println!("Map number of characters: {}", map.len());

        Map {
            map,
            nx,
            ny,
        }
    }

    pub fn new_empty(nx: usize, ny: usize) -> Map {
        let map = (0..nx*ny).map(|_| '.').collect::<String>();
        Map {
            map,
            nx,
            ny,
        }
    }

    pub fn plot_indices(&mut self, indices: &Vec<usize>) {
        for i in indices {
            self.map.replace_range(*i..=*i, "*");
        }
    }

    pub fn print_map(&self) {
        for i in 0..self.ny {
            println!("{}", &self.map[i*self.nx..(i+1)*self.nx]);
        }
    }

    pub fn find_start(&self) -> usize {
        self.map.find('S').unwrap()
    }

    pub fn find_connected(&self, current: usize) -> Vec<usize> {
        let mut connected = Vec::new();
        // check if row abow
        if current > self.nx {
            let above = current - self.nx;
            // if char is '|' or '7' or 'F' or 'S' add to connected
            if "|7FS".contains(self.map.chars().nth(above).unwrap()) {
                connected.push(above);
            }
        }
        // check if row below
        if current < self.nx * (self.ny - 1) {
            let below = current + self.nx;
            // if char is '|' or 'J' or 'L' or 'S' add to connected
            if "|JLS".contains(self.map.chars().nth(below).unwrap()) {
                connected.push(below);
            }
        }
        // check if column left
        if current % self.nx != 0 {
            let left = current - 1;
            // if char is '-' or 'F' or 'L' or 'S' add to connected
            if "-FLS".contains(self.map.chars().nth(left).unwrap()) {
                connected.push(left);
            }
        }
        // check if column right
        if current % self.nx != self.nx - 1 {
            let right = current + 1;
            // if char is '-' or 'J' or '7' or 'S' add to connected
            if "-J7S".contains(self.map.chars().nth(right).unwrap()) {
                connected.push(right);
            }
        }
        connected
    }

    pub fn find_next(&self, previous: usize, current: usize) -> usize {
        // combine characters of previous and current as string
        let pc = self.map.chars().nth(previous).unwrap();
        let cc = self.map.chars().nth(current).unwrap();
        let pc_cc = format!("{}{}", pc, cc);

        let next: usize = match &pc_cc[..] {
            // Symbols: | - L J 7 F S
            "||" => {if previous > current {current - self.nx} else {current + self.nx}},
            "|J" => current - 1,
            "|L" => current + 1,
            "|F" => current + 1,
            "|7" => current - 1,

            "--" => {if previous < current {current + 1} else {current - 1}},
            "-J" => current - self.nx,
            "-7" => current + self.nx,
            "-F" => current + self.nx,
            "-L" => current - self.nx,

            "L-" => current + 1,
            "L|" => current - self.nx,
            "LJ" => current - self.nx,
            "L7" => {if previous < current {current + self.nx} else {current - 1}},
            "LF" => current + 1,

            "J-" => current - 1,
            "J|" => current - self.nx,
            "J7" => current - 1,
            "JF" => {if previous - 1 == current {current + self.nx} else {current + 1}},
            "JL" => current - self.nx,

            "7-" => current - 1,
            "7|" => current + self.nx,
            "7L" => {if previous > current {current - self.nx} else {current + 1}},
            "7J" => current - 1,
            "7F" => current + self.nx,

            "F-" => current + 1,
            "F|" => current + self.nx,
            "F7" => current + self.nx,
            "FJ" => {if previous + 1 == current {current - self.nx} else {current - 1}},
            "FL" => current + 1,

            "S-" => {if previous < current {current + 1} else {current - 1}},
            "S|" => {if previous < current {current + self.nx} else {current - self.nx}},
            "SL" => {if previous < current {current + 1} else {current - self.nx}},
            "SJ" => {if current - 1 == previous {current - self.nx} else {current - 1}},
            "S7" => {if previous < current {current + self.nx} else {current - 1}},
            "SF" => {if current + 1 == previous {current + self.nx} else {current + 1}},

            _ => panic!("No match found"),
        };

        next
    }
}

fn main() {
    let map = Map::new(INPUT_DATA);

    // indices
    let mut indices = Vec::new();

    // find start
    let start = map.find_start();
    indices.push(start);

    let around = map.find_connected(start)[0];
    indices.push(around);

    // find cycle
    loop {
        let next = map.find_next(indices[indices.len() - 2], indices[indices.len() - 1]);
        if indices.contains(&next) {
            break;
        }
        indices.push(next);
    }

    println!("Cycle length: {}", indices.len());

    // answer
    let answer = indices.len() / 2;
    println!("Answer part a: {}", answer);

    // find points inside
    // let coordinates = indices.iter().map(|i| {
    //     let x = i % map.nx;
    //     let y = i / map.nx;
    //     (x as i32, y as i32)
    // }).collect::<Vec<(i32, i32)>>();

    // scale indices to factor
    let factor = 3;
    let mut new_map = Map::new_empty(map.nx * factor, map.ny * factor);
    let scaled_indices = indices.iter().map(|i| {
        let x = i % map.nx;
        let y = i / map.nx;
        let scaled_x = x * factor;
        let scaled_y = y * factor;
        let scaled_i = scaled_y * new_map.nx + scaled_x;
        scaled_i
    }).collect::<Vec<usize>>();

    new_map.plot_indices(&scaled_indices);

    new_map.print_map();

    let surface = vec![(0, 0), (0, 4), (4, 4), (4, 0)];
    let result = enclosed_coordinates(&surface);

    println!("Enclosed coordinates: {:?}", result);

}

