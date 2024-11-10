use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Create a file to write to
    let mut file = File::create("input1.txt")?;

    // Create a vector from 0 to 10_000
    let nums: Vec<i32> = (0..100).collect();

    // Start writing the opening bracket
    write!(file, "[")?;

    // Write each number to the file, separated by commas
    for (_, num) in nums.iter().enumerate() {
        write!(file, "{},", num)?;
    }

    let nums: Vec<i32> = (0..99).collect();

    for _ in nums.iter().enumerate() {
        write!(file, "{},", 100)?;
    }

    // Write the closing bracket
    write!(file, "100]")?;

    // Create a file to write to
    let mut file = File::create("input2.txt")?;

    // Create a vector from 0 to 4999
    let nums: Vec<i32> = (0..99).collect();

    // Start writing the opening bracket
    write!(file, "[")?;

    // Write each number to the file, separated by commas
    for (_, num) in nums.iter().enumerate() {
        write!(file, "{},{},", num, 100)?;
    }

    // Write the closing bracket
    write!(file, "99,100]")?;

    println!("Data written to file successfully in the desired format.");
    
    Ok(())
}

