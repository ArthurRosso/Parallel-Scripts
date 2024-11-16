use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Create a file to write to
    let mut file = File::create("input1.txt")?;

    // Create a vector from 0 to 10_000
    let nums: Vec<i32> = (0..1_000_000).collect();

    // Start writing the opening bracket
    write!(file, "[")?;

    // Write each number to the file, separated by commas
    for (_, num) in nums.iter().enumerate() {
        write!(file, "{},", num)?;
    }

    let nums: Vec<i32> = (0..1_000_000).collect();

    for _ in nums.iter().enumerate() {
        write!(file, "{},", 1_000_000)?;
    }

    // Write the closing bracket
    write!(file, "1000000]")?;

    // Create a file to write to
    let mut file = File::create("input2.txt")?;

    // Create a vector from 0 to 4999
    let nums: Vec<i32> = (0..999999).collect();

    // Start writing the opening bracket
    write!(file, "[")?;

    // Write each number to the file, separated by commas
    for (_, num) in nums.iter().enumerate() {
        write!(file, "{},{},", num, 1_000_000)?;
    }

    // Write the closing bracket
    write!(file, "999999,1000000]")?;

    println!("Data written to file successfully in the desired format.");

    Ok(())
}
