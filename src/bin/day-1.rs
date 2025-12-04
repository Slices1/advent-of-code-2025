use std::fs;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("../../inputs/1.txt");

    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    let mut zero_tally_part_1: u64 = 0;
    let mut zero_tally_part_2: u64 = 0;
    
    let mut dial_position: u64 = 50;
    
    for line in lines {

        let value: u64 = line[1..].parse()?;
        
        let sign = match &line[0..1] {
            "L" => -1,
            "R" => 1,
            _ => 0, 
        };

        // Part 1
        // Just tracks if we LAND on 0 at the end of the turn
        let next_position = (dial_position as i64 + sign * (value as i64)).rem_euclid(100) as u64;
        zero_tally_part_1 += (next_position == 0) as u64;


        // Part 2
        // 1. Add full revolutions (every 100 steps crosses 0)
        zero_tally_part_2 += value / 100;

        // 2. Check the remaining partial turn
        let remainder = value % 100;
        
        if remainder > 0 {
            if sign == 1 {
                if dial_position + remainder >= 100 {
                    zero_tally_part_2 += 1;
                }
            } else {
                if dial_position > 0 && remainder >= dial_position {
                    zero_tally_part_2 += 1;
                }
            }
        }

        dial_position = next_position;
    }

    println!("Result for part 1: {}", zero_tally_part_1);
    println!("Result for part 2: {}", zero_tally_part_2);

    Ok(())
}