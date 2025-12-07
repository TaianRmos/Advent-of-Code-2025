use std::fs;


pub fn part_1() -> std::io::Result<()> {
    // Reads the file
    let content: String = fs::read_to_string("src/challenge_6/input.txt")?;
    let math_lines: Vec<&str> = content.lines().collect();
    let mut number_grid: Vec<Vec<i64>> = Vec::new();

    // Creates the grid of numbers
    for line in &math_lines[..math_lines.len() - 1] {
        let number_line: Vec<i64> = line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
        number_grid.push(number_line);
    }
    // Get all the operations
    let operations: Vec<&str> = math_lines[math_lines.len() - 1].split_ascii_whitespace().collect();

    let mut final_result: i64 = 0;
    let mut result: i64;

    // For each operation, go through the whole column and evaluate
    for col in 0..operations.len() {
        if operations[col] == "+" {
            result = 0;
            for row in 0..number_grid.len() {
                result += number_grid[row][col];
            }
        }
        else {
            result = 1;
            for row in 0..number_grid.len() {
                result *= number_grid[row][col];
            }
        }
        final_result += result;
    }

    println!("Result part 1: {final_result}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    // Reads the file
    let content: String = fs::read_to_string("src/challenge_6/input.txt")?;
    let math_lines: Vec<&str> = content.lines().collect();

    // Get all the operations
    let operations: Vec<&str> = math_lines[math_lines.len() - 1].split_ascii_whitespace().collect();

    // Creates a grid with all chars inside the file, except for the last line (it's the operations)
    let mut file_grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    file_grid.pop();

    // Initialize the final result and current result
    let mut final_result: i64 = 0;
    let mut current_index: usize = 0;
    let mut result: i64 = 0;
    if operations[0] == "*" {
        result = 1;
    }

    // We read the whole grid column by column
    for col in 0..file_grid[0].len() {
        // Initialize an empty string and get the current operator
        let mut col_number: String = String::from("");
        let current_operator: &str = operations[current_index];

        // Concatenates all chars of the column into the string
        for row in 0..file_grid.len() {
            col_number.push(file_grid[row][col]);
        }

        // Get rid of all whitespaces
        col_number = col_number.replace(' ', "");

        // If the line is not empty, we found a number
        if col_number != "" {
            // Parse the number and add or multiply
            // depending on the current operator
            let number: i64 = col_number.parse().unwrap();
            if current_operator == "+" {
                result += number;
            }
            else {
                result *= number;
            }
        }
        // If the line is empty, it's the end of the column
        else {
            // We add the current result to the final one
            final_result += result;
            result = 0;

            // We go to the next operator
            current_index += 1;
            if operations[current_index] == "*" {
                result = 1;
            }
        }
    }
    final_result += result;
    println!("Result part 2: {final_result}");
    Ok(())
}