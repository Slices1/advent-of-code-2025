use std::{fs};
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("../../inputs/2.txt");
    let content = fs::read_to_string(file_path)?;
    // let lines: Vec<&str> = content.split(',').collect();

    // Idea 1: check every id

    // find number of IDs which meet this criteria
    // 1. has even num of digits
    // 2. first half of digit string == second half
    // Note: for 2.,

    // let mut part_1_sum: u64 = 0;
    // for line in lines {
    //     let ranges: Vec<&str> = line.split('-').collect();
    //     let range_start: u64 = ranges[0].parse().unwrap();
    //     let range_end: u64 = ranges[1].parse().unwrap();
    //     for id in range_start..=range_end {
    //         // print!("{id}");
    //         // condition 1:
    //         if id.len() % 2 == 1 { continue; }

    //     }
    // }


    // Idea 2: check every repeating pattern
    // let mut pattern = 0;
    // let _max_id: Vec<&str> = lines.last().unwrap().split('-').collect();
    // let max_id = _max_id[1].parse().unwrap();

    // println!("Max id: {max_id}");
    let mut part_1_sum: u64 = 0;
    let mut part_2_sum: u64 = 0;
// Split by comma, but verify the line isn't empty before processing
    for raw_line in content.split(',') {
        // 1. Remove whitespace (spaces, newlines) from both ends
        let line = raw_line.trim();
        
        // 2. Skip empty lines (handles trailing commas or newlines at EOF)
        if line.is_empty() {
            continue;
        }

        let ranges: Vec<&str> = line.split('-').collect();
        
        // 3. Parse safely (trimming individual numbers just in case)
        let mut range_start: u64 = ranges[0].trim().parse().unwrap();
        let mut range_end: u64 = ranges[1].trim().parse().unwrap();


        // let mut id: u64 = range_start;
        // if number of digits of start and end are same and odd then continue;
        if ranges[0].len() == ranges[1].len() && ranges[1].len() % 2 == 1 {
            continue;
        }
        if ranges[0].len() % 2 == 1 {
            // if it was 123, it should become 1000
            // println!("why is {range_start} panicking");
            range_start = 10_u64.pow(range_start.ilog10() + 1);
        }
        if ranges[1].len() % 2 == 1 {
            // if it was 123, it should become 99
            // println!("why is {} panicking? {} uhh {}", range_end, range_end.ilog10(), range_end.ilog10() - 1);
            range_end = (10_u64.pow(range_end.ilog10()) - 1) as u64;
            // println!("has it panicked yet? no...");
        }

        // range_start-range_end
        // for 1234-4567 I'd need 12 and 45
        // for 4321-8765, Id need 43 and 86

        // 1. Calculate how many digits there are (e.g., 4)
        let num_digits = range_start.ilog10() + 1;

        // 2. Calculate the divisor to chop off the bottom half
        //    For 4 digits, we divide by 10^2 (100)
        let divisor: u64 = 10_u32.pow(num_digits / 2) as u64;

        // 3. Perform the split
        let mut half_id_start = range_start / divisor; // 1234 -> 12
        let mut half_id_end = range_end / divisor;     // 4567 -> 45
        // println!("half_id_end={half_id_end}");
        if half_id_end > range_end % divisor {
            half_id_end -= 1;
            // println!("half_id_end={half_id_end}");
        }
        if half_id_start < range_start % divisor {
            half_id_start += 1;
        }

        // for half_id = half_id_start..=half_id_end {
        //     part_1_sum += id;
        // }
        let sum_of_halves = (half_id_end)*(half_id_end+1)/2 - (half_id_start-1)*(half_id_start)/2;
        let amount_to_add = sum_of_halves * (divisor + 1);
        part_1_sum += amount_to_add;
        // println!("For range {range_start}-{range_end}, got half id range as {half_id_start}-{half_id_end}. adding {amount_to_add}. part_1_sum={part_1_sum}");
    
    
    
    
    
        
        // Part 2:
        let start: u64 = ranges[0].parse()?;
        let end: u64 = ranges[1].parse()?;
        part_2_sum += solve(start, end);
    }

    println!("Result for part 1: {part_1_sum}");
    println!("Result for part 2: {part_2_sum}");
    Ok(())
}




fn solve(start: u64, end: u64) -> u64 {
    let start_len = start.ilog10() + 1;
    let end_len = end.ilog10() + 1;

    // If range crosses digit boundaries (e.g. 90-105), split it up (e.g. 90-99 and 100-105)
    if start_len != end_len {
        let boundary = 10_u64.pow(start_len);
        return solve(start, boundary - 1) + solve(boundary, end);
    }

    let len = start_len;
    let mut range_sum = 0;

    // find all divisors of the length (excluding the length itself)
    // for len 6: divisors are 1, 2, 3.
    let mut divisors = Vec::new();
    for i in 1..=len / 2 {
        if len % i == 0 {
            divisors.push(i);
        }
    }

    // For each pattern length, generate the numbers
    for divisor in divisors {
        // Calculate the Multiplier.
        let num = 10_u64.pow(len) - 1;
        let den = 10_u64.pow(divisor) - 1;
        let multiplier = num / den;

        // determine range for the base x
        let min_x_digits = 10_u64.pow(divisor - 1);
        let max_x_digits = 10_u64.pow(divisor) - 1;

        // Calculate raw bounds
        let start_base = (start + multiplier - 1) / multiplier; 
        let end_base = end / multiplier;               

        // Intersect with valid digit counts
        let actual_start = start_base.max(min_x_digits);
        let actual_end = end_base.min(max_x_digits);

        for x in actual_start..=actual_end {
            // 3. Avoid double counting!
            // If we are looking at divisor=2 (e.g. 121212),
            // we must ensure '12' isn't just '1' repeated (11).
            if !is_repetitive(x) {
                range_sum += x * multiplier;
            }
        }
    }

    range_sum
}

// Checks if a number is composed of a smaller sub-pattern repeated
// 1212 -> true
// 123 -> false
fn is_repetitive(n: u64) -> bool {
    let s: String = n.to_string();
    let len = s.len();
    
    // Check all divisors of the length
    for i in 1..=len / 2 {
        if len % i == 0 {
            let sub = &s[0..i];
            // split s into chunks using chunks(), each has a size the same as divisor
            // check if all chunks equals the substring using all()
            if s
                .as_bytes()
                .chunks(i)
                .all(|c| 
                            c == sub.as_bytes()
                    ) {
                return true;
            }
        }
    }
    false
}