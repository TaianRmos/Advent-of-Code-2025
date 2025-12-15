use std::fs;
use itertools::Itertools;


fn parse_content(content: String) -> (Vec<String>, Vec<Vec<String>>, Vec<Vec<i32>>, Vec<(usize, usize)>) {
    let mut outputs: Vec<String> = Vec::new();
    let mut toggles: Vec<Vec<String>> = Vec::new();
    let mut voltages: Vec<Vec<i32>> = Vec::new();
    let mut min_max_combinations: Vec<(usize, usize)> = Vec::new();

    for line in content.lines() {
        // Get the elements of the String
        let elements: Vec<String> = line.split(" ").map(|s| String::from(s)).collect();
        let nb_elements = elements.len();

        // Take the first part as 0s and 1s
        let output: String = elements[0]
            .replace(['[', ']'],"")
            .replace(".", "0")
            .replace("#", "1");
        let output_length: usize = output.len().to_owned();
        
        // Get all the toggles
        let mut line_toggles: Vec<String> = Vec::new();
        let mut line_toggle_sizes: Vec<i32> = Vec::new();
        for i in 1..nb_elements-1 {
            // Creates a sequence of 0s
            let mut toggle: Vec<char> = "0".repeat(output_length).chars().collect();

            // Get the lights toggled by the element
            let lights: Vec<i32> = elements[i].replace(['(', ')'], "").split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            line_toggle_sizes.push(lights.len() as i32);

            // Writes the toggle as 0s and 1s
            for light in lights {
                toggle[light as usize] = '1';
            }
            line_toggles.push(toggle.into_iter().collect());
        }

        let line_voltages: Vec<i32> = elements[nb_elements-1].replace(['{', '}'], "")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        let max_combination: i32 = line_voltages.iter().sum();
        let min_combination: i32 = max_combination / line_toggle_sizes.iter().max().unwrap();

        outputs.push(output);
        toggles.push(line_toggles);
        voltages.push(line_voltages);
        min_max_combinations.push((min_combination as usize, max_combination as usize));
    }

    (outputs, toggles, voltages, min_max_combinations)
}


fn xor_binary_strings(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars()).map(|(a, b)| if a == b {'0'} else {'1'}).collect()
}


fn add_voltages(volts: &Vec<i32>, toggle: &str) -> Vec<i32> {
    volts.iter().zip(toggle.chars()).map(|(&v, c)| if c == '0' {v} else {v + 1}).collect()
}


pub fn part_1() -> std::io::Result<()> {
    let content: String = fs::read_to_string("src/challenge_10/input.txt")?;
    let (outputs, toggles, _, _) = parse_content(content);
    let mut result: i32 = 0;

    // Search for every output
    for i in 0..outputs.len() {
        // Get the information for this line
        let output: String = outputs[i].clone();
        let output_length: usize = output.len();
        let line_toggles: Vec<String> = toggles[i].clone();
        let mut found_combination: bool = false;

        // Tries every possible size of combination
        for size in 1..=line_toggles.len() {
            // Gets a list of all combinations of the given size
            for toggle_combo in line_toggles.iter().combinations(size) {
                // Initialize the current state as 0s
                let mut current_state: String = "0".repeat(output_length);

                // Xor the state with the combination of toggles
                for toggle in toggle_combo {
                    current_state = xor_binary_strings(&current_state, toggle);
                }

                // Check if we found a working combination
                if current_state == output {
                    found_combination = true;
                    result += size as i32;
                    break;
                }
            }
            if found_combination {
                break;
            }
        }

    }

    println!("Result part 1: {result}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    let content: String = fs::read_to_string("src/challenge_10/input.txt")?;
    let (_, toggles, voltages, min_max_combinations) = parse_content(content);
    let mut result: i32 = 0;

    // Search for every voltage
    for i in 0..voltages.len() {
        println!("{i}/{:?}", voltages.len());
        // Get the information for this line
        let voltage = voltages[i].clone();        
        let voltage_length: usize = voltage.len();
        let (min_size, max_size) = min_max_combinations[i];
        let line_toggles: Vec<String> = toggles[i].clone();
        let mut found_combination: bool = false;
        
        // Tries every possible size of combination
        for size in min_size..=max_size {
            // Gets a list of all combinations of the given size (counting duplicates)
            for toggle_combo in line_toggles.iter().combinations_with_replacement(size) {
                // Initialize the current state as 0s
                let mut current_state: Vec<i32> = vec![0; voltage_length];

                // Adds the voltages to the current state
                for toggle in toggle_combo {
                    current_state = add_voltages(&current_state, toggle)
                }

                // Check if we found a working combination
                if current_state == voltage {
                    found_combination = true;
                    result += size as i32;
                    break;
                }
            }
            if found_combination {
                break;
            }
        }
    }

    println!("Result part 2: {result}");
    Ok(())
}