use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/5.txt")
        .or_else(|_| fs::read_to_string("../../inputs/5.txt"))
        .expect("Could not find input file in either location");    

    let lines: Vec<&str> = content.lines().collect();


    
    Ok(())
}