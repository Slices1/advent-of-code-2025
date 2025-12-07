use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/5.txt")
        .or_else(|_| fs::read_to_string("../../inputs/5.txt"))
        .expect("Could not find input file in either location");    


    // split at \n\n, first section is ingredient ID ranges, second is ingredient ids
    let split_lines = content.split("\n\n").collect::<Vec<&str>>();

    let (id_ranges, ids) = (split_lines[0], split_lines[1]);

    // iterate over id_ranges, parse into vector(min, max)
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for line in id_ranges.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let min: usize = parts[0].parse()?;
        let max: usize = parts[1].parse()?;
        ranges.push((min, max));
    }

    // check each id 

    let mut valid_count = 0;
    for id in ids.lines() {
        let id_num: usize = id.parse().unwrap();
        for (min, max) in &ranges {
            if id_num >= *min && id_num <= *max {
                valid_count += 1;
                break;
            }
        }
    }

    println!("Valid ingredient IDs: {}", valid_count);



    

    // part 2
    // find vec of ranges (stored in ranges rn)
    // sort ranges by start
    // merge overlapping ranges
    // count total covered numbers in merged ranges by doing sum of end - start

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // print 
    // for range in &ranges {
    //     println!("Range: {}-{}", range.0, range.1);
    // }

    // merge
    let mut merged_ranges: Vec<(usize, usize)> = Vec::new();
    merged_ranges.push(ranges[0]);

    for range in ranges {
        let last = merged_ranges.last_mut().unwrap();
        if range.0 <= last.1 {
            last.1 = last.1.max(range.1);
        } else {
            merged_ranges.push(range);
        }

    }
    // print
    // for range in &merged_ranges {
        // println!("Merged Range: {}-{}", range.0, range.1);
    // }

    // sum
    let mut ids_in_ranges = 0;
    for range in &merged_ranges {
        ids_in_ranges += range.1 - range.0 + 1;
    }

    println!("Total ingredient IDs covered by ranges: {}", ids_in_ranges);

    Ok(())
}