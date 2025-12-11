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

    fn rectangle_area(&self, p: &Point) -> i64 {
        ((self.x - p.x).abs() + 1) * ((self.y - p.y).abs() + 1)
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


    fn is_on_segment(&self, p: &Point) -> bool {
        if self.is_vertical {
            if p.x == self.start.x && p.y >= self.start.y && p.y <= self.end.y {
                return true;
            }
            else {
                return false;
            }
        }
        else {
            if p.y == self.start.y && p.x >= self.start.x && p.x <= self.end.x {
                return true;
            }
            else {
                return false;
            }
        }
    }

    fn is_ray_colliding_with(&self, p: &Point) -> bool {
        // The raycast is horizontal, so the segment 
        // has to be vertical
        if !self.is_vertical {
            return false;
        }
        // If the point is after the segment
        // (the raycast goes to the right)
        else if p.x >= self.start.x {
            return false;
        }
        // If the ray is at a height between the edge of the
        // segment, we're colliding with it
        else if p.y >= self.start.y && p.y < self.end.y {
            return true;
        }
        // We're not colliding
        else {
            return false;
        }
    }

    fn is_intersecting_with(&self, seg: &Segment) -> bool {
        if self.is_vertical == seg.is_vertical {
            return false;
        }
        if self.is_vertical {
            if self.start.x > seg.start.x && self.start.x < seg.end.x && seg.start.y > self.start.y && seg.start.y < self.end.y {
                return true;
            }
            else {
                return false;
            }
        }
        else {
            if self.start.y > seg.start.y && self.start.y < seg.end.y && seg.start.x > self.start.x && seg.start.x < self.end.x {
                return true;
            }
            else {
                return false;
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
    let content: String = fs::read_to_string("src/challenge_9/input.txt")?;
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

    let nb_segments: usize = all_segments.len();

    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        // First point we choose
        let a: Point = points[i];

        for j in i+1..points.len() {
            // Diagonal point
            let c: Point = points[j];

            // We get the area of the rectangle
            let area: i64 = a.rectangle_area(&c);
            if area <= max_area {
                continue;
            }

            // Define the two other points of the rectangle
            let b: Point = Point { x: a.x, y: c.y };
            let d: Point = Point { x: c.x, y: a.y };

            // We already know that A and C are inside
            let rectangle_points_to_test: Vec<&Point> = vec![&b, &d];

            // Creates the rectangle itself with segments
            let rectangle: Vec<Segment> = vec![
                Segment::new(&a, &b),
                Segment::new(&b, &c),
                Segment::new(&c, &d),
                Segment::new(&d, &a)
            ];
            let mut valid_rectangle: bool = true;

            for point in rectangle_points_to_test {
                let mut nb_collisions: i32 = 0;
                let mut is_on_segment: bool = false;

                // Look every single segments of the shape
                for i in 0..nb_segments {
                    // Check if the point is on any segment
                    if all_segments[i].is_on_segment(point) {
                        is_on_segment = true;
                        break;
                    }
                    // Raycast to count the amount of collisions
                    if all_segments[i].is_ray_colliding_with(point) {
                        nb_collisions += 1;
                    }
                }
                // If not on a segment, and an even amount of collisions
                // it means the points is outside the shape
                if !is_on_segment && nb_collisions % 2 == 0 {
                    valid_rectangle = false;
                    break;
                }
            }

            // Skip if it's already invalid
            if !valid_rectangle {
                continue;
            }

            // Look at each segment of the rectangle
            for side in rectangle {
                for i in 0..nb_segments {
                    // Checks if it intersect with any segment of the shape
                    if all_segments[i].is_intersecting_with(&side) {
                        valid_rectangle = false;
                        break;
                    }
                }
                // We already found a side that intersects
                if !valid_rectangle {
                    break;
                }
            }

            // Skip if it's already invalid
            if !valid_rectangle {
                continue
            }
                
            // We found a better rectangle
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Result part 2: {max_area}");
    Ok(())
}