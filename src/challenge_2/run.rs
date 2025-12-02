use std::fs;


pub fn _part_1() -> std::io::Result<()> {
    // Reads the first (and only) line, then split on the ','
    let content: String = fs::read_to_string("src/challenge_2/input.txt")?;
    let ranges: Vec<&str> = content.trim_end().split(",").collect();
    let mut total_incorrect_ids: i64 = 0;

    // Loop on all the ranges
    for range in ranges {
        // Split the range to extract the start and end of the range
        let mut splitted: std::str::Split<'_, &str> = range.split("-");
        let start_range: i64 = splitted.next().unwrap().parse().unwrap();
        let end_range: i64 = splitted.next().unwrap().parse().unwrap();
        
        // For every number inside that range we check
        // if the number is repeating twice
        for id in start_range..=end_range {
            let id_str = id.to_string();
            if id_str.len() % 2 == 0 {
                let middle = id_str.len() / 2;
                if id_str[..middle] == id_str[middle..] {
                    total_incorrect_ids += id;
                }
            }
        }
    }

    println!("Answer: {total_incorrect_ids}");
    Ok(())
}


pub fn _part_2() -> std::io::Result<()> {
    // Reads the first (and only) line, then split on the ","
    let content: String = fs::read_to_string("src/challenge_2/input.txt")?;
    let ranges: Vec<&str> = content.trim_end().split(",").collect();
    let mut total_incorrect_ids: i64 = 0;

    // Loop on all the ranges
    for range in ranges {
        // Split the range to extract the start and end of the range
        let mut splitted: std::str::Split<'_, &str> = range.split("-");
        let start_range: i64 = splitted.next().unwrap().parse().unwrap();
        let end_range: i64 = splitted.next().unwrap().parse().unwrap();
        
        // Loops on all the numbers inside the range
        for id in start_range..=end_range {
            let id_str = id.to_string();
            let id_length = id_str.len();

            // We try every size of chunks between 1 and len / 2
            for l in 1..=id_length/2 {
                if id_length % l == 0 {
                    // Splits the id into chunks and storing it into a vector
                    let id_chunks: Vec<String> = id_str
                        .as_bytes()
                        .chunks(l)
                        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
                        .collect();

                    // Compares the elements 2 by 2 to see if they're all equal
                    let all_equal = id_chunks.windows(2).all(|w| w[0] == w[1]);
                    if all_equal {
                        // Adds the id and break (otherwise 12121212 would match two times)
                        total_incorrect_ids += id;
                        break;
                    }
                }
            }
        }
    }

    println!("Answer: {total_incorrect_ids}");
    Ok(())
}