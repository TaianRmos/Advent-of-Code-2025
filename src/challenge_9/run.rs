use std::fs;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Segment {
    start: Point,
    end: Point,
    is_vertical: bool,
}


impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point {x, y}
    }
}


impl Segment {
    fn new(p1: &Point, p2: &Point) -> Self {
        if p1.x == p2.x {
            if p1.y < p2.y {
                Segment { start: p1.to_owned(), end: p2.to_owned(), is_vertical: true }
            }
            else {
                Segment { start: p2.to_owned(), end: p1.to_owned(), is_vertical: true }
            }
        }
        else {
            if p1.x < p2.x {
                Segment { start: p1.to_owned(), end: p2.to_owned(), is_vertical: false }
            }
            else {
                Segment { start: p2.to_owned(), end: p1.to_owned(), is_vertical: false }
            }
        }
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
    let mut points: Vec<Point> = content.lines()
        .map(|s| {
            let mut splitted = s.split(",");
            let x: i64 = splitted.next().unwrap().parse().unwrap();
            let y: i64 = splitted.next().unwrap().parse().unwrap();
            Point { x, y }
        })
        .collect();
    let last_point: Point = points[0].clone();
    points.push(last_point);

    let all_segments: Vec<Segment> = points.windows(2)
        .map(|p| Segment::new(&p[0], &p[1]))
        .collect();

    for segment in all_segments {
        println!("{:?}", segment);
    }

    let max_area: i64 = 0;
    println!("Result part 2: {max_area}");
    Ok(())
}