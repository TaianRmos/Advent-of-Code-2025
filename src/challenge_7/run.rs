use std::fs;


#[allow(dead_code)]
fn display_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c: char = grid[i][j];
            print!("{c}");
        }
        println!();
    }
}

#[allow(dead_code)]
fn display_count_grid(grid: &Vec<Vec<i64>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c: i64 = grid[i][j];
            if c == 0 {
                print!(".")
            }
            else {
                print!("{c}");
            }
        }
        println!();
    }
}


fn propagate_the_beam(mut grid: Vec<Vec<char>>) -> (i32, i64) {
    // Get the size of the grid and start the Beam
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    // grid_count[i][j] is the amount of beams in the cell i, j
    let mut grid_count: Vec<Vec<i64>> = vec![vec![0; cols]; rows];
    grid[1][cols/2] = '|';
    grid_count[1][cols/2] = 1;
    
    // Parse the grid
    let mut splits: i32 = 0;
    for i in 2..rows {
        for j in 0..cols {
            // If the cell above is not a beam, we skip
            if grid[i-1][j] != '|' {
                continue
            }

            // If it's a dot, we continue the beam
            if grid[i][j] == '.' || grid[i][j] == '|' {
                grid[i][j] = '|';

                // Adds the beams above
                grid_count[i][j] += grid_count[i-1][j];
            }
            // If it's a ^ we split
            else if grid[i][j] == '^' {
                splits += 1;
                grid[i][j-1] = '|';
                grid[i][j+1] = '|';

                // Adds the beam above to the sides
                grid_count[i][j-1] += grid_count[i-1][j];
                grid_count[i][j+1] += grid_count[i-1][j];
            }
        }
    }

    let total_paths: i64 = grid_count[rows - 1].iter().sum();
    (splits, total_paths)
}


pub fn part_1() -> std::io::Result<()> {
    // Reads the file in a grid
    let content: String = fs::read_to_string("src/challenge_7/input.txt")?;
    let grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    // Propagates and prints the result
    let (splits, _) = propagate_the_beam(grid);
    println!("Result part 1: {splits}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    // Reads the file in a grid
    let content: String = fs::read_to_string("src/challenge_7/input.txt")?;
    let grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    // Propagates and prints the result
    let (_, total_paths) = propagate_the_beam(grid);
    println!("Result part 2: {total_paths}");
    Ok(())
}