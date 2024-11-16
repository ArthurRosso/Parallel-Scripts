// use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

// use itertools::Itertools;
use rayon::join;
// use diam::join;
// use diam::prelude::*;
// use diam::svg;
use rayon::prelude::*;

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
// pub fn repeated_n_times_best(nums: Vec<i32>) -> i32 {
//     if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
//         return nums[0];
//     } else if nums[1] == nums[3] {
//         return nums[1];
//     }

//     for i in 1..nums.len() {
//         if nums[i] == nums[i - 1] {
//             return nums[i];
//         }
//     }
//     0
// }

pub fn repeated_n_times_best(nums: Vec<i32>) -> i32 {
    if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
        return nums[0];
    } else if nums[1] == nums[3] {
        return nums[1];
    }

    let mut res = 0;
    let mut iter = nums.windows(2);
    while let Some(x) = iter.next() {
        if x[0] == x[1] {
            res = x[0];
            break;
        }
    }

    res
}

// v1.0
// // Stack overflow:
// pub fn repeated_2_times(nums: Vec<i32>) -> i32 {
//     if nums.len() > 2 {
//         let mut iter = nums.into_iter();
//         let rx = iter.next().unwrap();
//         let ry = iter.next().unwrap();
//         let (ra, rb) = join(
//             || {
//                 if rx == ry { rx } else { 0 }
//             },
//             || {
//                 repeated_2_times(iter.collect())
//             }
//         );
//         return ra | rb;
//     }
//     return 0;
// }
// pub fn repeated_n_times_par(nums: Vec<i32>) -> i32 {
//     let r = repeated_2_times(nums.clone());
//     if r == 0 {
//         if (nums[0] ^ nums[2] == 0) || (nums[0] ^ nums[3] == 0) {
//             return nums[0];
//         } else if nums[1] ^ nums[3] == 0 {
//             return nums[1];
//         } else { // if nums[2] ^ nums[3] == 0
//             return nums[2];
//         }
//     }
//     return r;
// }

pub fn repeated_2_times_join(nums: &[i32]) -> Option<i32> {
    // At least >= 4
    let mut iter = nums.into_iter();
    let rx = *iter.next().unwrap_or(&0);
    let ry = *iter.next().unwrap_or(&0);
    let (ra, rb) = join(
        || {
            if rx == ry {
                Some(rx)
            } else {
                None
            }
        },
        || {
            if nums.len() > 2 {
                repeated_2_times_join(iter.as_slice())
            } else {
                None
            }
        },
    );
    ra.or(rb)
}

pub fn repeated_n_times_join(nums: Vec<i32>) -> i32 {
    if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
        return nums[0];
    } else if nums[1] == nums[3] {
        return nums[1];
    }

    let mut wanted_size = 4;
    let mut remaining_nums: &[i32] = nums.as_slice();
    let mut blocks = std::iter::from_fn(|| {
        if remaining_nums.is_empty() {
            None
        } else {
            let block_size = remaining_nums.len().min(wanted_size);
            let (block, remains) = remaining_nums.split_at(block_size);
            remaining_nums = remains;
            wanted_size *= 2;
            Some(block)
        }
    });
    while let Some(chunk) = blocks.next() {
        if let Some(r) = repeated_2_times_join(chunk) {
            return r;
        }
    }
    0
}

pub fn repeated_2_times_old(nums: &[i32]) -> Option<i32> {
    nums.par_windows(2)
        .fold(
            || None,
            |acc, w| {
                if w[0] == w[1] {
                    Some(w[0])
                } else {
                    acc
                }
            },
        )
        .reduce(|| None, |a, b| a.or(b))
}

pub fn repeated_2_times(nums: &[i32]) -> Option<i32> {
    nums.par_chunks(32)
        .fold(
            || None,
            |acc, w| {
                let res = w.windows(2).find(|win| win[0] == win[1]);
                if let Some(r) = res {
                    Some(r[0])
                } else {
                    acc
                }
            },
        )
        .reduce(|| None, |a, b| a.or(b))
}

pub fn repeated_n_times_block(nums: Vec<i32>) -> i32 {
    let mut wanted_size = 32;

    if nums.len() < wanted_size {
        return repeated_n_times_best(nums);
    }

    if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
        return nums[0];
    } else if nums[1] == nums[3] {
        return nums[1];
    }

    let mut remaining_nums: &[i32] = nums.as_slice();
    let mut blocks = std::iter::from_fn(|| {
        if remaining_nums.is_empty() {
            None
        } else {
            let block_size = remaining_nums.len().min(wanted_size);
            let (block, remains) = remaining_nums.split_at(block_size);
            remaining_nums = remains;
            wanted_size *= 2;
            Some(block)
        }
    });
    while let Some(chunk) = blocks.next() {
        if let Some(r) = repeated_2_times(chunk) {
            return r;
        }
    }

    0
}

fn main() -> io::Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    // Test cases
    println!("Prices 0");
    let prices0 = vec![1, 2, 3, 3];
    let start = Instant::now();
    let result00 = repeated_n_times_hash_seq(prices0.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result01 = repeated_n_times_best(prices0.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result02 = repeated_n_times_block(prices0.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result03 = repeated_n_times_join(prices0.clone());
    let duration = start.elapsed();
    println!("Function Parallel Joins, Time elapsed: {:?}", duration);

    // let start = Instant::now();
    // let result04 = repeated_n_times_blocks3(prices0.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel 3, Time elapsed: {:?}", duration);
    // println!("{}", result0);  // Expected output: 3
    assert_eq!(result00, 3);
    assert_eq!(result01, 3);
    assert_eq!(result02, 3);
    assert_eq!(result03, 3);

    println!("Prices 1");
    let prices1 = vec![1, 3, 2, 3];
    let start = Instant::now();
    let result10 = repeated_n_times_hash_seq(prices1.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result11 = repeated_n_times_best(prices1.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result12 = repeated_n_times_block(prices1.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result13 = repeated_n_times_join(prices1.clone());
    let duration = start.elapsed();
    println!("Function Parallel Joins, Time elapsed: {:?}", duration);

    // let start = Instant::now();
    // let result14 = repeated_n_times_blocks3(prices1.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel 3, Time elapsed: {:?}", duration);
    // println!("{}", result0);  // Expected output: 3
    assert_eq!(result10, 3);
    assert_eq!(result11, result12);
    assert_eq!(result13, 3);

    println!("Prices 2");
    let prices2 = vec![2, 1, 2, 5, 3, 2];
    let start = Instant::now();
    let result20 = repeated_n_times_hash_seq(prices2.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result21 = repeated_n_times_best(prices2.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result22 = repeated_n_times_block(prices2.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result23 = repeated_n_times_join(prices2.clone());
    let duration = start.elapsed();
    println!("Function Parallel Joins, Time elapsed: {:?}", duration);

    // let start = Instant::now();
    // let result24 = repeated_n_times_blocks3(prices2.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel 3, Time elapsed: {:?}", duration);
    assert_eq!(result20, 2);
    assert_eq!(result21, result22);
    assert_eq!(result23, 2);

    println!("Prices 3");
    let prices3 = vec![5, 1, 5, 2, 5, 3, 5, 4];
    let start = Instant::now();
    let result30 = repeated_n_times_hash_seq(prices3.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result31 = repeated_n_times_best(prices3.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result32 = repeated_n_times_block(prices3.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result33 = repeated_n_times_join(prices3.clone());
    let duration = start.elapsed();
    println!("Function Parallel Joins, Time elapsed: {:?}", duration);

    // let start = Instant::now();
    // let result34 = repeated_n_times_blocks3(prices3.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel 3, Time elapsed: {:?}", duration);
    assert_eq!(result30, 5);
    assert_eq!(result31, result32);
    assert_eq!(result33, 5);

    println!("Prices 4");
    let prices4 = vec![1, 2, 3, 5, 5, 5, 5, 4];
    let start = Instant::now();
    let result40 = repeated_n_times_hash_seq(prices4.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result41 = repeated_n_times_best(prices4.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result42 = repeated_n_times_block(prices4.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result43 = repeated_n_times_join(prices4.clone());
    let duration = start.elapsed();
    println!("Function Parallel Joins, Time elapsed: {:?}", duration);

    // let start = Instant::now();
    // let result44 = repeated_n_times_blocks3(prices4.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel 3, Time elapsed: {:?}", duration);
    assert_eq!(result40, 5);
    assert_eq!(result41, result42);
    assert_eq!(result43, 5);

    println!("Prices Nums 1");
    // Read the file content into a string
    let mut file = File::open("test/input1-20_000.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Remove the square brackets
    content = content
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_string();

    // Split the string by commas and collect into a Vec<i32>
    let nums1: Vec<i32> = content
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid number"))
        .collect();

    let start = Instant::now();
    let resultnums10 = repeated_n_times_hash_seq(nums1.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums11 = repeated_n_times_best(nums1.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums12 = repeated_n_times_block(nums1.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums13 = repeated_n_times_join(nums1.clone());
    let duration = start.elapsed();
    println!("Function Parallel Joins, Time elapsed: {:?}", duration);
    assert_eq!(resultnums10, 10_000);
    assert_eq!(resultnums11, resultnums12);
    assert_eq!(resultnums13, 10_000);

    println!("Prices Nums 2");
    // Read the file content into a string
    let mut file = File::open("test/input2-20_000.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Remove the square brackets
    content = content
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_string();

    // Split the string by commas and collect into a Vec<i32>
    let nums2: Vec<i32> = content
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid number"))
        .collect();

    let start = Instant::now();
    let resultnums20 = repeated_n_times_hash_seq(nums2.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums21 = repeated_n_times_best(nums2.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums22 = repeated_n_times_block(nums2.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums23 = repeated_n_times_join(nums2.clone());
    let duration = start.elapsed();
    println!("Function Parallel Joins, Time elapsed: {:?}", duration);

    assert_eq!(resultnums20, 10_000);
    assert_eq!(resultnums21, resultnums22);
    assert_eq!(resultnums23, 10_000);

    println!("Prices Nums 1_000_000");
    // Read the file content into a string
    let mut file = File::open("test/input1.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Remove the square brackets
    content = content
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_string();

    // Split the string by commas and collect into a Vec<i32>
    let nums1: Vec<i32> = content
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid number"))
        .collect();

    let start = Instant::now();
    let resultnums100 = repeated_n_times_hash_seq(nums1.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums110 = repeated_n_times_best(nums1.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let resultnums120 = repeated_n_times_block(nums1.clone());
    let duration = start.elapsed();
    println!("Function Parallel Block, Time elapsed: {:?}", duration);
    assert_eq!(resultnums100, 1_000_000);
    assert_eq!(resultnums110, resultnums120);

    // let mut file = File::open("test/input1-100.txt")?;
    // let mut content = String::new();
    // file.read_to_string(&mut content)?;

    // // Remove the square brackets
    // content = content
    //     .trim()
    //     .trim_start_matches('[')
    //     .trim_end_matches(']')
    //     .to_string();

    // // Split the string by commas and collect into a Vec<i32>
    // let nums: Vec<i32> = content
    //     .split(',')
    //     .map(|s| s.trim().parse().expect("Invalid number"))
    //     .collect();

    // let mut i: i32 = 0;
    // svg("results/block_1.svg", || {
    //     i = repeated_n_times_block(nums);
    // })
    // .expect("failed saving log");
    // assert_eq!(i, 100);

    Ok(())
}
