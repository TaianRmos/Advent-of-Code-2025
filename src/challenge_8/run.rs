use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}


impl Point {
    // Creates a new point
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point {x, y, z}
    }

    // Euclidian distance between two points
    fn distance_between(a: &Self, b: &Self) -> f64 {
        let dx: f64 = a.x - b.x;
        let dy: f64 = a.y - b.y;
        let dz: f64 = a.z - b.z;

        (dx * dx + dy *dy + dz * dz).sqrt()
    }
}


fn are_in_circuit(circuits: &Vec<Vec<Point>>, point_a: &Point, point_b: &Point) -> (i32, i32) {
    let (mut found_a, mut found_b) = (-1, -1);

    for (i, circuit) in circuits.iter().enumerate() {
        if circuit.contains(point_a) {
            found_a = i as i32;
        }
        if circuit.contains(point_b) {
            found_b = i as i32;
        }
    }

    (found_a, found_b)
}


fn create_circuits(mut iterations: usize) -> (i64, i64) {
    let content: String = fs::read_to_string("src/challenge_8/input.txt").unwrap();
    let points_str: Vec<&str> = content.lines().collect();
    let mut all_points: Vec<Point> = Vec::new();

    // Creates all the points from the file
    for point_str in points_str {
        let coordinates: Vec<f64> = point_str.split(",").map(|c| c.parse().unwrap()).collect();
        all_points.push(Point::new(coordinates[0], coordinates[1], coordinates[2]))
    }

    // Calculates distances between every point and sort them
    // We only need to go through the up side of the diagonal to avoid duplicates
    let mut distances: Vec<(f64, Point, Point)> = Vec::new();
    for i in 0..all_points.len() - 1 {
        let point_a = &all_points[i];
        for j in i+1..all_points.len() {
            let point_b = &all_points[j];
            if point_a == point_b {
                continue;
            }
            let distance: f64 = Point::distance_between(point_a, point_b);
            distances.push((distance, point_a.clone(), point_b.clone()));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Creating the circuits
    let mut circuits: Vec<Vec<Point>> = Vec::new();
    let mut result_2: i64 = 0;

    // For difference between part 1 and 2
    if iterations == 0 {
        iterations = distances.len();
    }

    for i in 0..iterations {
        let (_, point_a, point_b) = distances[i];

        // found_a is the index of the circuit containing a, -1 if not found
        let (found_a, found_b) = are_in_circuit(&circuits.clone(), &point_a, &point_b);

        // None of the points were found in existing circuits
        if found_a == -1 && found_b == -1 {
            circuits.push(vec![point_a, point_b]);
        }
        // Point A is already in a circuit, we add point B
        else if found_a != -1 && found_b == -1 {
            circuits[found_a as usize].push(point_b);
        }
        // Point B is already in a circuit, we add point 1
        else if found_a == -1 && found_b != -1 {
            circuits[found_b as usize].push(point_a);
        }
        // Both points were found in a circuit
        else if found_a != -1 && found_b != -1 {
            // If A and B are already in the same circuit, nothing to do
            // If A and B are in different circuits, we need to merge them
            if found_a != found_b {
                let mut circuit_b: Vec<Point> = circuits[found_b as usize].clone();
                circuits[found_a as usize].append(&mut circuit_b);
                circuits.remove(found_b as usize);
            }
        }

        // If we have only one circuit containing all points, we stop
        if circuits.len() == 1 && circuits[0].len() == all_points.len() {
            result_2 = point_a.x as i64 * point_b.x as i64;
            break
        }
    }

    // Gets the length of the circuits, sort them and multiply the 3 largest
    let mut circuit_lengths: Vec<i64> = circuits.iter().map(|c| c.len() as i64).collect();
    circuit_lengths.sort_unstable_by(|a, b| b.cmp(a));
    let result_1: i64 = circuit_lengths.iter().take(3).product();
    (result_1, result_2)
}


pub fn part_1() -> std::io::Result<()> {
    // Creates the circuits and get the results
    let (result, _) = create_circuits(1000);
    println!("Result part 1: {result}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    // Creates the circuits and get the results
    let (_, result) = create_circuits(0);
    println!("Result part 2: {result}");
    Ok(())
}