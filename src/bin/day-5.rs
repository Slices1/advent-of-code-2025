use std::fs;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("../../inputs/5.txt");

    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();


    
    Ok(())
}