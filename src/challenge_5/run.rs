use std::fs;


pub fn part_1() -> std::io::Result<()> {
    let content: String = fs::read_to_string("src/challenge_5/input.txt")?;

    // Find the index of the empty line
    let ranges_and_ids: Vec<&str> = content.lines().collect();
    let split_index: usize = ranges_and_ids.iter().position(|line| line.is_empty()).unwrap();

    // Just split ranges from ids
    let ranges: &[&str] = &ranges_and_ids[..split_index];
    let ids: &[&str] = &ranges_and_ids[split_index + 1..];
    let mut fresh_ids: i32 = 0;
    
    for id in ids {
        // Converts the id in integer
        let num_id: i64 = id.parse().unwrap();
        for range in ranges {
            // Get the start and end of the range
            let mut range_splitted: std::str::Split<'_, &str> = range.split("-");
            let range_start: i64 = range_splitted.next().unwrap().parse().unwrap();
            let range_end: i64 = range_splitted.next().unwrap().parse().unwrap();
            
            // If it's contained, it's fresh
            if range_start <= num_id && num_id <= range_end {
                fresh_ids += 1;
                break
            }
        }
    }

    println!("Result part 1: {fresh_ids}");
    Ok(())
}


pub fn part_2() -> std::io::Result<()> {
    let content: String = fs::read_to_string("src/challenge_5/input.txt")?;

    // Find the index of the empty line
    let ranges_and_ids: Vec<&str> = content.lines().collect();
    let split_index: usize = ranges_and_ids.iter().position(|line| line.is_empty()).unwrap();

    // Extract only the ranges, then map to split and get tuples
    let ranges_str: &[&str] = &ranges_and_ids[..split_index];
    let mut ranges: Vec<(i64, i64)> = ranges_str.iter()
        .map(|r| {
            let mut splitted: std::str::Split<'_, &str> = r.split("-");
            let start: i64 = splitted.next().unwrap().parse().unwrap();
            let end: i64 = splitted.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    // Sort the ranges by the start of the range to 
    // easily detect overlap
    ranges.sort_by_key(|r| r.0);
    let mut unique_ranges: Vec<(i64, i64)> = Vec::new();

    for range in ranges {
        // If we have an element inside the vector
        if let Some(last) = unique_ranges.last_mut() {
            // If the start of this range is bigger than the end
            // of the last, there is no overlapping
            if last.1 < range.0 {
                unique_ranges.push(range);
            }
            // Else, we know the start of the last is smaller, so
            // we just take the max of the end of the two ranges
            else {
                last.1 = last.1.max(range.1)
            }
        }
        // If the vector is empty
        else {
            unique_ranges.push(range);
        }
    }

    // We just add the length of all unique ranges
    let mut fresh_ids = 0;
    for (start, end) in unique_ranges {
        fresh_ids += end - start + 1;
    }

    println!("Result part 2: {fresh_ids}");
    Ok(())
}

