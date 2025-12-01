use std::fs;

pub fn _part_1() -> std::io::Result<()> {
    // Read the file and extract the lines (the instructions)
    let input_file: String = fs::read_to_string("src/challenge_1/input.txt")?;
    let code_instructions: Vec<&str> = input_file.lines().collect();

    // Initialize the variables
    let mut pointing_to: i32 = 50;
    let mut amount_of_zeros: i32 = 0;

    for code in code_instructions {
        // Seperate the direction from the amount of clicks the dial will rotate
        let direction: char = code.chars().next().unwrap();
        let nb_clicks: i32 = code[1..].parse().unwrap();
        
        // If Left, we substract, if Right we add 
        if direction == 'L' {
            pointing_to = (pointing_to - nb_clicks) % 100;
        }
        else {
            pointing_to = (pointing_to + nb_clicks) % 100;
        }

        // Adding again if the dial ends up on 0
        if pointing_to == 0 {
            amount_of_zeros += 1;
        }
    }

    println!("Secret code: {amount_of_zeros}");

    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    // Read the file and extract the lines (the instructions)
    let input_file: String = fs::read_to_string("src/challenge_1/input.txt")?;
    let code_instructions: Vec<&str> = input_file.lines().collect();

    // Initialize the variables
    let mut pointing_to: i32 = 50;
    let mut amount_of_zeros: i32 = 0;

    for code in code_instructions {
        // Seperate the direction from the amount of clicks the dial will rotate
        let direction: char = code.chars().next().unwrap();
        let nb_clicks: i32 = code[1..].parse().unwrap();
        
        // If Left, we substract, if Right we add 
        if direction == 'L' {
            pointing_to = pointing_to - nb_clicks;
            while pointing_to < 0 {
                // Handles the case where it was pointing at 0
                if pointing_to + nb_clicks != 0 {
                    amount_of_zeros += 1;
                }
                pointing_to += 100;
            }
        }
        else {
            pointing_to = pointing_to + nb_clicks;
            while pointing_to > 99 {
                // Handles the case where it was a multiple of 100
                if pointing_to != 100 {
                    amount_of_zeros += 1;
                }
                pointing_to -= 100;
            }
        }

        // Adding again if the dial ends up on 0
        if pointing_to == 0 {
            amount_of_zeros += 1;
        }        
    }

    println!("Secret code: {amount_of_zeros}");

    Ok(())
}