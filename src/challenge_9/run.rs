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
    let mut points_on_column: Vec<Vec<i64>> = vec![Vec::new(); (max_x + 1) as usize];

    // Fill the points into the rows and columns
    for point in &points {
        let (x, y) = point.clone();
        points_on_row[y as usize].push(x);
        points_on_column[x as usize].push(y);
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

    for i in 0..points_on_column.len() {
        if points_on_column[i].len() == 0 {
            points_on_column[i].push(0);
            points_on_column[i].push(0);
        }
        else {
            points_on_column[i].sort();
        }
    }

    let max_area = 0;
    println!("Result part 2: {max_area}");
    Ok(())
}