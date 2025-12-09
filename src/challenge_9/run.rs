use std::fs;


fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
}


pub fn part_1() -> std::io::Result<()> {
    let content: String = fs::read_to_string("src/challenge_9/input.txt")?;
    let points: Vec<(i64, i64)> = content.lines()
        .map(|s| {
            let mut splitted = s.split(",");
            let x: i64 = splitted.next().unwrap().parse().unwrap();
            let y: i64 = splitted.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        let point_a: (i64, i64) = points[i];
        for j in i+1..points.len() {
            let point_b: (i64, i64) = points[j];
            let area: i64 = ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1);

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Result part 1: {max_area}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    let content: String = fs::read_to_string("src/challenge_9/test.txt")?;
    let points: Vec<(i64, i64)> = content.lines()
        .map(|s| {
            let mut splitted = s.split(",");
            let x: i64 = splitted.next().unwrap().parse().unwrap();
            let y: i64 = splitted.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    // Get the max values for x and y to construct the grid
    let (max_x, max_y) = points.iter()
        .fold((i64::MIN, i64::MIN), |(mx, my), (x, y)| {
            (mx.max(*x), my.max(*y))
        });
        
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; (max_x + 3) as usize]; (max_y + 3) as usize];
    for point in &points {
        grid[point.1 as usize][point.0 as usize] = '#';
    }
    print_grid(&grid);
    println!();

    // Complete the shape horizontally
    let mut is_filling: bool = false;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // If it's a #, we start filling if we weren't
            // and we stop if it's the second #
            if grid[i][j] == '#' {
                if is_filling {
                    is_filling = false;
                }
                else {
                    is_filling = true;
                }
            }
            // Fill the grid if needed
            else if is_filling {
                grid[i][j] = 'X';
            }
        }
    }


    // Complete the shape vertically
    is_filling = false;
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            // If it's a #, we start filling if we weren't
            // and we stop if it's the second #
            if grid[i][j] == '#' {
                if is_filling {
                    is_filling = false;
                }
                else {
                    is_filling = true;
                }
            }
            // Fill the grid if needed
            else if is_filling {
                grid[i][j] = 'X';
            }
        }
    }

    // Change path by #
    for row in &mut grid {
        for cell in row {
            if *cell == 'X' {
                *cell = '#';
            }
        }
    }

    print_grid(&grid);
    println!();

    // Fill the grid
    let mut is_filling: bool = false;
    for i in 0..grid.len() {
        let mut nb_hashtags: usize = grid[i].iter().filter(|&&c| c == '#').count();

        for j in 0..grid[0].len() {
            // If it's a #, we start filling if we weren't
            // and we stop if it's the second #
            if grid[i][j] == '#' {
                nb_hashtags -= 1;
                if nb_hashtags > 0 {
                    is_filling = true;
                }
                else {
                    is_filling = false;
                }
            }
            // Fill the grid if needed
            else if is_filling {
                grid[i][j] = '#';
            }
        }
    }
    

    print_grid(&grid);


    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        let point_a: (i64, i64) = points[i];
        for j in i+1..points.len() {
            let point_b: (i64, i64) = points[j];

            if grid[point_b.1 as usize][point_a.0 as usize] == '.' || grid[point_a.1 as usize][point_b.0 as usize] == '.' {
                continue;
            }    

            let area: i64 = ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1);

            if area > max_area {
                max_area = area;
            }
        }
    }

    let max_area = 0;
    println!("Result part 2: {max_area}");
    Ok(())
}