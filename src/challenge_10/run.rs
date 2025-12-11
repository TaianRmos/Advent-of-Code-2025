use std::fs;


pub fn part_1() -> std::io::Result<()> {
    let content: String = fs::read_to_string("src/challenge_10/test.txt")?;
    let mut outputs: Vec<String> = Vec::new();
    let mut toggles: Vec<Vec<String>> = Vec::new();
    let mut voltages: Vec<Vec<i32>> = Vec::new();

    for line in content.lines() {
        let elements: Vec<String> = line.split(" ").map(|s| String::from(s)).collect();
        let output: String = elements[0]
            .replace(['[', ']'],"")
            .replace(".", "0")
            .replace("#", "1");
        
        let toggle: String = "0".repeat(output.len());
        



    }


    let result: i32 = 0;
    println!("Result part 1: {result}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    let _content: String = fs::read_to_string("src/challenge_10/test.txt")?;
    let result: i32 = 0;
    println!("Result part 2: {result}");
    Ok(())
}