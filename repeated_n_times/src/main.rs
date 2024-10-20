use std::collections::HashMap;

// use rayon::prelude::*;

pub fn repeated_n_times_hash_seq(nums: Vec<i32>) -> i32 {
    let mut seq: HashMap<i32, i32> = HashMap::new();
    
    for &x in nums.iter() {
        if let Some(_) = seq.get(&x) {
            return x;
        } else {
            seq.insert(x, 1);
        }
    }
    0
}

// Best approach
pub fn repeated_n_times_best(nums: Vec<i32>) -> i32 {
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            return nums[i];
        }
    }
    
    if (nums[0] ^ nums[2] == 0) || (nums[0] ^ nums[3] == 0) {
        return nums[0];
    } else if nums[1] ^ nums[3] == 0 {
        return nums[1];
    } else { // if nums[2] ^ nums[3] == 0
        return nums[2];
    }
}

pub fn repeated_n_times_near_best(nums: Vec<i32>) -> i32 {
    let mut iter = nums.windows(2);

    while let Some(x) = iter.next() {
        if x[0] ^ x[1] == 0 {
            return x[0];
        }
    }
    
    if (nums[0] ^ nums[2] == 0) || (nums[0] ^ nums[3] == 0) {
        return nums[0];
    } else if nums[1] ^ nums[3] == 0 {
        return nums[1];
    } else { // if nums[2] ^ nums[3] == 0
        return nums[2];
    }
}

pub fn recursive_xor(nums: Vec<i32>) -> i32 {
    if nums.len() > 2 {
        let mut iter = nums.into_iter();
        let rx = iter.next().unwrap();
        let ry = iter.next().unwrap(); 
        let (ra, rb) = rayon::join(
            || {
                if rx ^ ry == 0 { rx } else { 0 }
            },
            || {
                recursive_xor(iter.collect())
            }
        );
        return ra | rb;
    }
    return 0;
}

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let r = recursive_xor(nums.clone());
    if r == 0 {
        if (nums[0] ^ nums[2] == 0) || (nums[0] ^ nums[3] == 0) {
            return nums[0];
        } else if nums[1] ^ nums[3] == 0 {
            return nums[1];
        } else { // if nums[2] ^ nums[3] == 0
            return nums[2];
        }
    } else {
        return r;
    }
}

fn main() {
    // Test cases
    let prices0 = vec![1,2,3,3];
    let result0 = repeated_n_times(prices0);
    println!("{}", result0);  // Expected output: 3
    
    let prices1 = vec![1,3,2,3];
    let result1 = repeated_n_times(prices1);
    println!("{}", result1);  // Expected output: 3

    let prices2 = vec![2,1,2,5,3,2];
    let result2 = repeated_n_times(prices2);
    println!("{}", result2);  // Expected output: 2

    let prices3 = vec![5,1,5,2,5,3,5,4];
    let result3 = repeated_n_times(prices3);
    println!("{}", result3);  // Expected output: 5

    let prices4 = vec![1,2,3,5,5,5,5,4];
    let result4 = repeated_n_times(prices4);
    println!("{}", result4);  // Expected output: 5
}
