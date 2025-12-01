use std::fs;

pub fn challenge_1() -> std::io::Result<()> {
    let input_file: String = fs::read_to_string("src/challenge_1/input.txt")?;

    let code_instructions: Vec<&str> = input_file.lines().collect();

    for code in code_instructions {
        println!("{code}")
    }

    Ok(())
}