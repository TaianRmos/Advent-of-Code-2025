use std::{fs, vec};

fn is_inside(grid: &Vec<Vec<u8>>, y: usize, x: usize) -> bool {
    if grid[y][x] == 1 {
        return true
    }

    let width = grid[0].len();
    let mut crossings = 0;

    if x > width/2 {
        // Cast a ray to the right
        for j in x+1 .. width {
            if grid[y][j] == 1 && grid[y][j-1] == 1 &&  grid[y][j-2] == 1{
                continue
            }
            if grid[y][j] == 1 {
                crossings += 1;
            }
        }
    }
    else {
        // Cast a ray to the left
        for j in (0..x).rev() {
            // skip thick horizontal lines of 3 cells
            if grid[y][j] == 1 && grid[y][j+1] == 1 && grid[y][j+2] == 1 {
                continue
            }
            if grid[y][j] == 1 {
                crossings += 1;
            }
        }
    }


    // odd = inside, even = outside
    crossings % 2 == 1
}


fn print_grid(grid: &Vec<Vec<u8>>) {
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
    // Reading the file and extracting the points
    let content: String = fs::read_to_string("src/challenge_9/input.txt")?;
    let mut points: Vec<(i64, i64)> = content.lines()
        .map(|s| {
            let mut splitted = s.split(",");
            let x: i64 = splitted.next().unwrap().parse().unwrap();
            let y: i64 = splitted.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let last_point: (i64, i64) = points[0].clone();
    points.push(last_point);

    // Get the max values for x and y to construct the grid
    let (max_x, max_y) = points.iter()
        .fold((i64::MIN, i64::MIN), |(mx, my), (x, y)| {
            (mx.max(*x), my.max(*y))
        });
        
    let mut grid: Vec<Vec<u8>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1 ) as usize];
    let mut current_position = points[0].clone();
    grid[current_position.1 as usize][current_position.0 as usize] = 1;

    // Trace the outline of the path
    for point in points.clone() {
        // If points are aligned on the x axis
        if point.0 == current_position.0 {
            // We need to check which y is larger for the 'for' loop
            if current_position.1 < point.1 {
                for i in current_position.1..=point.1 {
                    grid[i as usize][point.0 as usize] = 1;
                }
            }
            else {
                for i in point.1..=current_position.1 {
                    grid[i as usize][point.0 as usize] = 1;
                }
            }
        }
        // If points are aligned on the y axis
        else {
            // We need to check which y is larger for the 'for' loop
            if current_position.0 < point.0 {
                for i in current_position.0..=point.0 {
                    grid[point.1 as usize][i as usize] = 1;
                }
            }
            else {
                for i in point.0..=current_position.0 {
                    grid[point.1 as usize][i as usize] = 1;
                }
            }
        }
        current_position = point.clone();
    }

    // Fill the inside of the shape
    // let mut is_filling: bool = false;
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         if is_filling && grid[i][j] == 0 {
    //             grid[i][j] = 2;
    //         }
    //         else if grid[i][j] == 1 {
    //             if !is_filling {
    //                 is_filling = true;
    //             }
    //             else if grid[i][j-1] != 1 {
    //                 is_filling = false;
    //             }
    //         }
    //     }
    //     is_filling = false;
    // }

    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        let point_a: (i64, i64) = points[i];
        for j in i+1..points.len() {
            let point_b: (i64, i64) = points[j];

            // If it's not inside the shape, we skip it
            if !is_inside(&grid, point_a.1 as usize, point_b.0 as usize) || !is_inside(&grid, point_b.1 as usize, point_a.0 as usize) {
                continue
            }

            let area: i64 = ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1);

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Result part 2: {max_area}");
    Ok(())
}