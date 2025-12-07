use std::fs;
use std::error::Error;

fn solve(numbers_part_1: &Vec<u64>, operation: &str) -> u64 {
    match operation {
        "+" => numbers_part_1.iter().sum(),
        "*" => numbers_part_1.iter().product(),
        _ => panic!("Unknown"),
    }
}

fn solver(numbers: &Vec<Vec<u64>>, operations: &Vec<&str>) -> u64 {
    let mut grand_total: u64 = 0;
    for col in 0..numbers[0].len() {
        let mut col_numbers: Vec<u64> = Vec::new();
        for row in 0..numbers.len() {
            col_numbers.push(numbers[row][col]);
        }
        let operation = operations[col];
        grand_total += solve(&col_numbers, operation);
    }
    grand_total
}

fn solver_part_2(numbers: &Vec<Vec<u64>>, operations: &Vec<&str>) -> u64 {
    let mut grand_total: u64 = 0;

    // Zip numbers with operations to process them in pairs
    for (problem, op) in numbers.iter().zip(operations.iter()) {
        grand_total += solve(problem, op);
    }

    grand_total
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/6.txt")
        .or_else(|_| fs::read_to_string("../../inputs/6.txt"))
        .expect("Could not find input file in either location");    

    // I added another newline just before the end of the input file to separate parts

    // firt part are the digits, second part are the operations
    // make 1d vec of operations, make 2d vec of numbers_part_1
    let split_lines = content.split("\n\n").collect::<Vec<&str>>();
    let (numbers_section, operations_section) = (split_lines[0], split_lines[1]);
    let mut operations: Vec<&str> = Vec::new();
    operations = operations_section.split_whitespace().collect();

    let mut numbers_part_1: Vec<Vec<u64>> = Vec::new();
    // numbers_part_1 are separated by 1 or more spaces
    // at the end of the numbers_part_1 line there are 0 or more spaces as well
    for line in numbers_section.lines() {
        // I also need to reverse the order of chars to get correct numbers
        let row: Vec<u64> = line
            .split_whitespace()
            .map(|num_str| num_str.parse::<u64>().unwrap())
            .collect();
        numbers_part_1.push(row);
    }


// part 2
// numbers_section: &str
// 123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// ->
// numbers_part_2: Vec<Vec<u64>>
// 1 369 32 623
// 24 248 587 431
// 356 8 175 4


    // plan:
    // iterate over columns of numbers_section
    // for each column, collect the digits into a number
    // repeat until all digits in column are ' ',
    // at which point we have found the first column of numbers for numbers_part_2
    // parse and push it to a temp variable
    // repeat
    // then push the columns of temp variable to numbers_part_2

    // check:
    // 1 4
    // 2 5
    // 3 6
    // stored in temp as
    // 1 2 3
    // 4 5 6
    // then we need to transpose to numbers_part_2

    let lines: Vec<Vec<char>> = numbers_section.lines().map(|l| l.chars().collect()).collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // numbers_part_2 will now store a list of problems:
    // [[1, 24, 356], [369, 248, 8], ...]
    let mut numbers_part_2: Vec<Vec<u64>> = Vec::new();
    let mut temp_col_group: Vec<u64> = Vec::new();

    for col in 0..=width {
        let mut num_str = String::new();

        // 1. Scan vertically to build a number string
        if col < width {
            for line in &lines {
                if let Some(&c) = line.get(col) {
                    if !c.is_whitespace() {
                        num_str.push(c);
                    }
                }
            }
        }

        // 2. Logic: Add number to current group OR flush group to result
        if !num_str.is_empty() {
            // We found a number part, add it to our current "problem" buffer
            temp_col_group.push(num_str.parse().unwrap());
        } else if !temp_col_group.is_empty() {
            // We hit a space column (or end of string) -> Parsing for this block is done.
            // Push the whole problem as a single row.
            numbers_part_2.push(temp_col_group.clone());
            temp_col_group.clear();
        }
    }

    // print
    // for row in &numbers_part_1 {
        // println!("{:?}", row);
    // }
    // println!("Operations: {:?}", operations);
    // print numbers_part_2
    // println!("\nNumbers Part 2:");
    // for row in &numbers_part_2 {
    //     println!("{:?}", row);
    // }

    // apply solve() to each column of numbers_part_1 with corresponding operation
    let grand_total_part_1: u64 = solver(&numbers_part_1, &operations);
    let grand_total_part_2: u64 = solver_part_2(&numbers_part_2, &operations);



    println!("Grand Total: {}", grand_total_part_1);
    println!("Part 2 Grand Total: {}", grand_total_part_2);


    // for part 2, index differently
    // each numbers_part_1 start is vertically aligned with its operation
    // each column is a number
    // we can likely transpose the number_section when still in string form
    // then split and parse each transposed line to get number vec
    
    Ok(())
}

// Part 2 Grand Total: 10227753257799