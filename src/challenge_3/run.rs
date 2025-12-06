use std::fs;


pub fn part_1() -> std::io::Result<()> {
    // Read the file and extract the lines (the battery banks)
    let input_file: String = fs::read_to_string("src/challenge_3/input.txt")?;
    let battery_banks: Vec<&str> = input_file.lines().collect();
    let mut result: i32 = 0;

    for bank in battery_banks {
        // Creates a vector of batteries
        let batteries: Vec<i32> = bank.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

        // Gets the max and the index of the max on all batteries except the last
        let first_max = *batteries[..batteries.len() - 1].iter().max().unwrap();
        let i = batteries[..batteries.len() - 1].iter().position(|&x| x == first_max).unwrap();

        // gets the second max after the first
        let second_max = batteries[i+1..].iter().max().unwrap();

        result += 10 * first_max + second_max;
    }

    println!("Result part 1: {result}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    // Read the file and extract the lines (the battery banks)
    let input_file: String = fs::read_to_string("src/challenge_3/input.txt")?;
    let battery_banks: Vec<&str> = input_file.lines().collect();
    let mut result: i64 = 0;

    for bank in battery_banks {
        // Creates a vector of batteries
        let batteries: Vec<i32> = bank.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

        // Creates the vector which will store all the values we find
        let mut max_values: Vec<i32> = Vec::new();

        // Initialize the starting and ending indexes for the batteries we can look at
        // We start at 0, and end 11 digits before the end since we need at least 11 other digits to complete the whole number
        // Then, we start at the index of the last digit we selected, and end 1 digit later
        let mut index_start: usize = 0;
        let mut index_end: usize = batteries.len() - 11;

        // For each digit, find the max inside the slice we have and update the indexes
        for _ in 0..12 {
            let current_max_first = *batteries[index_start..index_end].iter().max().unwrap();

            // The starting index is increased by one plus the index inside of the max we found inside the slice
            index_start = index_start + 1 + batteries[index_start..index_end].iter().position(|&x| x == current_max_first).unwrap();
            index_end += 1;
            max_values.push(current_max_first);
        }

        // Construct the number from the digits we found
        let mut max_value: i64 = 0;
        for digit in max_values {
            max_value = max_value * 10 + digit as i64;
        }

        result += max_value
    }

    println!("Result part 2: {result}");
    Ok(())
}
