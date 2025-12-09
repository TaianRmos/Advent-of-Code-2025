// use std::fmt::format;
use std::fs::{create_dir_all, File};
use std::io::Write;

mod challenge_1;
mod challenge_2;
mod challenge_3;
mod challenge_4;
mod challenge_5;
mod challenge_6;
mod challenge_7;
mod challenge_8;
mod challenge_9;

// Content of rust files when creating a new challenge
const MOD_FILE_CONTENT: &str = "#[allow(dead_code)]\npub mod run;";
const RUN_FILE_CONTENT: &str = r#"use std::fs;


pub fn part_1() -> std::io::Result<()> {
    let _content: String = fs::read_to_string("src/challenge_{{}}/test.txt")?;
    let result: i32 = 0;
    println!("Result part 1: {result}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    let _content: String = fs::read_to_string("src/challenge_{{}}/test.txt")?;
    let result: i32 = 0;
    println!("Result part 2: {result}");
    Ok(())
}"#;


#[allow(dead_code)]
fn create_challenge_folder(number: usize) -> std::io::Result<()> {
    // Creates the folder
    let folder_name: String = format!("src/challenge_{}", number);
    create_dir_all(&folder_name)?;

    // Creates all the files inside it
    let _md_file: File = File::create(format!("{}/challenge_{}.md", folder_name, number))?;
    let _input_file: File = File::create(format!("{}/input.txt", folder_name))?;
    let _test_file: File = File::create(format!("{}/test.txt", folder_name))?;
    let mut mod_file: File = File::create(format!("{}/mod.rs", folder_name))?;
    let mut run_file: File = File::create(format!("{}/run.rs", folder_name))?;

    mod_file.write_all(MOD_FILE_CONTENT.as_bytes())?;
    run_file.write_all(RUN_FILE_CONTENT.replace("{{}}", &number.to_string()).as_bytes())?;

    Ok(())
}


#[allow(dead_code)]
fn run_all_challenges() -> std::io::Result<()> {
    println!("\n==========DAY 1==========\n");
    challenge_1::run::part_1()?;
    challenge_1::run::part_2()?;
    
    println!("\n==========DAY 2==========\n");
    challenge_2::run::part_1()?;
    challenge_2::run::part_2()?;
    
    println!("\n==========DAY 3==========\n");
    challenge_3::run::part_1()?;
    challenge_3::run::part_2()?;
    
    println!("\n==========DAY 4==========\n");
    challenge_4::run::part_1()?;
    challenge_4::run::part_2()?;
    
    println!("\n==========DAY 5==========\n");
    challenge_5::run::part_1()?;
    challenge_5::run::part_2()?;
    
    println!("\n==========DAY 6==========\n");
    challenge_6::run::part_1()?;
    challenge_6::run::part_2()?;
    
    println!("\n==========DAY 7==========\n");
    challenge_7::run::part_1()?;
    challenge_7::run::part_2()?;
    
    println!("\n==========DAY 8==========\n");
    challenge_8::run::part_1()?;
    challenge_8::run::part_2()?;
    
    println!("\n==========DAY 9==========\n");
    challenge_9::run::part_1()?;
    challenge_9::run::part_2()?;
    
    Ok(())
}


fn main() -> std::io::Result<()> {
    // create_challenge_folder(9)?;
    challenge_9::run::part_2()?;
    // run_all_challenges()?;
    Ok(())
}