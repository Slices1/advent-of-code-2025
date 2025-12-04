use std::fs;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("inputs/3.txt");

    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // find the largest combination of 2 digits. you cannot rearrange
    // perhaps find the 2 largest numbers, then order them in the same way as the original
    // nvm.

    // when the largest digit is at the end, that is always the unit digit.
    // and the 2nd largest digit is the tens digit

    // when the largest digit isnt at the end, it is the tens digit
    // and the 2nd largest digit is the largest digit to its right

    let mut total_joltage: u64 = 0;
    for bank in &lines {
        let bank_length:usize = bank.len();
        let mut largest_digit = '0';
        let mut largest_index = 0;
        let mut second_largest_digit = '0';
        let tens_digit;
        let units_digit;
        for (i, digit) in bank.chars().enumerate() {
            if digit > largest_digit {
                largest_digit = digit;
                largest_index = i;
            }
        }
        // if largest digit is at the end
        // find 2nd largest to the right
        if largest_index == bank_length - 1 {
            units_digit = largest_digit;
            for digit in bank.chars() {
                if digit > second_largest_digit && digit != largest_digit {
                    second_largest_digit = digit;
                }
            }
            tens_digit = second_largest_digit;
        } else {
            // largest digit is not at the end
            tens_digit = largest_digit;
            for digit in bank.chars().skip(largest_index + 1) {
                if digit > second_largest_digit {
                    second_largest_digit = digit;
                }
            }
            units_digit = second_largest_digit;
        }
        let joltage: u32 = (tens_digit.to_digit(10).unwrap() * 10) + units_digit.to_digit(10).unwrap();
        // println!("Bank: {}, Tens: {}, Units: {}, Joltage: {}", bank, tens_digit, units_digit, joltage);
        total_joltage += joltage as u64;
    }

    println!("Total joltage: {}", total_joltage);


    // for part 2, we will step through the lines again
    // we have to find the 12 digits in the bank that give the highest total joltage
    
    // we can instead build the joltage number digit by digit
    // // starting from the left, select the largest digit that still leaves enough digits to complete the number
    // then move to the next digit and repeat until we have 12 digits

    total_joltage = 0;
    for bank in lines {
        let bank_length:usize = bank.len();
        let mut search_width = bank_length - 11;
        let mut best_joltage: u64 = 0;
        let mut current_index = 0;
        while search_width > 0 && current_index + search_width - 1 < bank_length{
            // select max digit within the search window
            let mut largest_digit = '0';
            let mut largest_index: usize = 0;
            // start from current_index and search search_width chars
            for (i, digit) in bank.chars().enumerate().skip(current_index).take(search_width) {
                if digit > largest_digit {
                    largest_digit = digit;
                    largest_index = i;
                }
            }
            // println!("Current index: {}, Search width: {}, Largest digit: {}, Largest index: {}", current_index, search_width, largest_digit, largest_index);
            search_width -= largest_index - current_index ;
            current_index = largest_index + 1;
            // add the digit to joltage string
            best_joltage = best_joltage * 10 + largest_digit.to_digit(10).unwrap() as u64;
        }
        // println!("Bank: {}, Best joltage: {}", bank, best_joltage);
        total_joltage += best_joltage;
    }
    println!("Total joltage (part 2): {}", total_joltage);
    Ok(())
}