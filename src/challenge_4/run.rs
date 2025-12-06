use std::fs;

const _NEIGHBOURS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn count_roll_papers(i: usize, j: usize, grid: &Vec<Vec<char>>) -> i32 {
    let mut neighbour_roll_papers: i32 = 0;
    let rows: i32 = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;

    // Tries each of the 8 neighbours and look if it's an @
    for (x, y) in _NEIGHBOURS {
        let nx = i as i32 + x;
        let ny = j as i32 + y;

        if ny >= 0 && ny < rows && nx >= 0 && nx < cols && grid[nx as usize][ny as usize] == '@' {
            neighbour_roll_papers += 1;
        }
    }

    neighbour_roll_papers
}

fn get_accessible_roll_papers(initial_grid: &Vec<Vec<char>>, replace: bool) -> (Vec<Vec<char>>, i32) {
    let mut grid: Vec<Vec<char>> = initial_grid.clone();
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let mut accessible_roll_papers = 0;

    // Go through the whole grid 
    for i in 0..rows {
        for j in 0..cols {
            // Skip if it's not a @
            if grid[i][j] == '.' {
                continue
            }
            // Count the amount of roll papers in the neighbours
            let neighbour_roll_papers = count_roll_papers(i, j, &grid);

            // if it's lower than 4, it's accessible
            if neighbour_roll_papers < 4 {
                accessible_roll_papers += 1;

                // Replace if needed to
                if replace {
                    grid[i][j] = '.'; 
                }
            }
        }
    }

    (grid, accessible_roll_papers)
}


pub fn part_1() -> std::io::Result<()> {
    // Loads the file
    let input_file: String = fs::read_to_string("src/challenge_4/input.txt")?;
    let grid: Vec<Vec<char>> = input_file.lines().map(|line| line.chars().collect()).collect();

    // Counts the amount of accessible roll papers
    let accessible_roll_papers: i32;
    (_, accessible_roll_papers) = get_accessible_roll_papers(&grid, false);
    println!("Result part 1: {accessible_roll_papers}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    // Loads the file
    let input_file: String = fs::read_to_string("src/challenge_4/input.txt")?;
    let mut grid: Vec<Vec<char>> = input_file.lines().map(|line| line.chars().collect()).collect();

    let mut total_accessible_roll_papers = 0;
    let mut accessible_roll_papers = -1;

    // While we find accessible roll papers, we count them and remove them
    while accessible_roll_papers != 0 {
        (grid, accessible_roll_papers) = get_accessible_roll_papers(&grid, true);
        total_accessible_roll_papers += accessible_roll_papers;
    }
    println!("Result part 2: {total_accessible_roll_papers}");
    Ok(())
}