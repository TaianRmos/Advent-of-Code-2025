use std::{fs, vec};
use std::fs::File;
use std::io::Write;


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
    // Reading the file and extracting the points
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
    
    // points_on_row[x] has the list of y coordinates where we have points on row x
    // Since we should have only 2 (verified), it's either [0, 0], or [ya, yb]
    let mut points_on_row: Vec<Vec<i64>> = vec![Vec::new(); (max_y + 1) as usize];

    // Fill the points into the rows and columns
    for point in &points {
        let (x, y) = point.clone();
        points_on_row[y as usize].push(x);
    }

    // Fill the empty vectors with 2 zeros, and sort to simplify the use case
    for i in 0..points_on_row.len() {
        if points_on_row[i].len() == 0 {
            points_on_row[i].push(0);
            points_on_row[i].push(0);
        }
        else {
            points_on_row[i].sort();
        }
    }

    let mut file = File::create("output_raw.txt")?;

    for i in 0..points_on_row.len() {
        writeln!(file, "{i} - {:?}", points_on_row[i])?;
    }
    writeln!(file)?;

    let mut start: i64 = 0;
    let mut end: i64 = 0;
    for i in 0..points_on_row.len() {
        // Bot points are 0 (no # on the row)
        if points_on_row[i][0] == 0 && points_on_row[i][1] == 0 {
            points_on_row[i][0] = start;
            points_on_row[i][1] = end;
        }
        // Start and end are not initialize yet
        else if start == 0 && end == 0 {
            start = points_on_row[i][0];
            end = points_on_row[i][1];
        }
        // Enlarge on the left
        else if points_on_row[i][0] < start && points_on_row[i][1] == start {
            points_on_row[i][1] = end;
            start = points_on_row[i][0];
        }
        // Enlarge on the right
        else if points_on_row[i][0] == end && points_on_row[i][1] > end {
            points_on_row[i][0] = start;
            end = points_on_row[i][1];
        }
        // Shrink on the left
        else if points_on_row[i][0] == start && points_on_row[i][1] < end {
            start = points_on_row[i][1];
            points_on_row[i][1] = end;
        }
        // Shrink on the right
        else if points_on_row[i][0] > start && points_on_row[i][1] == end {
            end = points_on_row[i][0];
            points_on_row[i][0] = start;
        }
    }

    let mut file = File::create("output.txt")?;

    for i in 0..points_on_row.len() {
        writeln!(file, "{i} - {:?}", points_on_row[i])?;
    }
    writeln!(file)?;

    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        let point_a: (i64, i64) = points[i];
        for j in i+1..points.len() {
            let point_b: (i64, i64) = points[j];

            if point_a.0 < points_on_row[point_b.1 as usize][0] || point_a.0 > points_on_row[point_b.1 as usize][1] {
                continue;
            }

            if point_b.0 < points_on_row[point_a.1 as usize][0] || point_b.0 > points_on_row[point_a.1 as usize][1] {
                continue;
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