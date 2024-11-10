// use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

use diam::join;
//use diam::svg;
// use itertools::Itertools;
// use rayon::ThreadPoolBuilder;

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
    if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
        return nums[0];
    } else if nums[1] == nums[3] {
        return nums[1];
    }

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            return nums[i];
        }
    }
    0
}

// pub fn repeated_n_times_best(nums: Vec<i32>) -> i32 {
//     let mut iter = nums.windows(2);
//     while let Some(x) = iter.next() {
//         if x[0] == x[1] {
//             return x[0];
//         }
//     }
//     if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
//         return nums[0];
//     } else if nums[1] == nums[3] {
//         return nums[1];
//     } else { // if nums[2] ^ nums[3] == 0
//         return nums[2];
//     }
// }

// // Stack overflow:
// pub fn repeated_2_times(nums: Vec<i32>) -> i32 {
//     if nums.len() > 2 {
//         let mut iter = nums.into_iter();
//         let rx = iter.next().unwrap();
//         let ry = iter.next().unwrap();
//         let (ra, rb) = rayon::join(
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

// v1.0
pub fn repeated_2_times_no_optim(nums: &[i32]) -> i32 {
    if nums.len() >= 2 {
        let mut iter = nums.into_iter();
        let rx = *iter.next().unwrap();
        let ry = *iter.next().unwrap();
        let (ra, rb) = join(
            || {
                if rx == ry {
                    rx
                } else {
                    0
                }
            },
            || repeated_2_times_no_optim(iter.as_slice()),
        );
        return ra | rb;
    }
    return 0;
}

pub fn repeated_n_times_par_no_optim(nums: Vec<i32>) -> i32 {
    if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
        return nums[0];
    } else if nums[1] == nums[3] {
        return nums[1];
    }

    let mut wanted_size = 2;
    let mut remaining_nums: &[i32] = nums.as_slice();
    let mut blocks = std::iter::from_fn(|| {
        if remaining_nums.is_empty() {
            None
        } else {
            let block_size = remaining_nums.len().min(wanted_size);
            let (block, remains) = remaining_nums.split_at(block_size);
            remaining_nums = remains;
            wanted_size = wanted_size * 2;
            Some(block)
        }
    });
    while let Some(chunk) = blocks.next() {
        let r: i32 = repeated_2_times_no_optim(chunk);
        if r != 0 {
            return r;
        }
    }
    0
}

pub fn repeated_2_times(nums: &[i32]) -> i32 {
    // At least >= 4
    let mut iter = nums.into_iter();
    let rx = *iter.next().unwrap_or(&0);
    let ry = *iter.next().unwrap_or(&0);
    let (ra, rb) = join(
        || {
            if rx == ry {
                rx
            } else {
                0
            }
        },
        || {
            if nums.len() > 2 {
                repeated_2_times(iter.as_slice())
            } else {
                0
            }
        },
    );
    return ra | rb;
}

pub fn repeated_n_times_par(nums: Vec<i32>) -> i32 {
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
            wanted_size = wanted_size * 2;
            Some(block)
        }
    });
    // let mut contador = 0;
    while let Some(chunk) = blocks.next() {
        // contador = contador + 1;
        let r: i32 = repeated_2_times(chunk);
        if r != 0 {
            // println!("Contador da gurizada: {}", contador);
            return r;
        }
    }
    // println!("Contador da gurizada: {}", contador);
    0
}

pub fn repeated_2_times_option(nums: &[i32]) -> Option<i32> {
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
                repeated_2_times_option(iter.as_slice())
            } else {
                None
            }
        },
    );
    if ra.is_some() {
        ra
    } else if rb.is_some() {
        rb
    } else {
        None
    }
}

pub fn repeated_n_times_par_option(nums: Vec<i32>) -> i32 {
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
            wanted_size = wanted_size * 2;
            Some(block)
        }
    });
    // let mut contador = 0;
    while let Some(chunk) = blocks.next() {
        // contador = contador + 1;
        if let Some(r) = repeated_2_times_option(chunk) {
            return r;
        }
    }
    // println!("Contador da gurizada: {}", contador);
    0
}

fn main() -> io::Result<()> {
    // Test cases
    println!("Prices 0");
    let prices0 = vec![1,2,3,3];
    let start = Instant::now();
    let result00 = repeated_n_times_hash_seq(prices0.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result01 = repeated_n_times_best(prices0.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result02 = repeated_n_times_par(prices0.clone());
    let duration = start.elapsed();
    println!("Function Parallel, Time elapsed: {:?}", duration);
    // println!("{}", result0);  // Expected output: 3
    assert_eq!(result00, 3);
    assert_eq!(result00, result01);
    assert_eq!(result01, result02);

    println!("Prices 1");
    let prices1 = vec![1,3,2,3];
    let start = Instant::now();
    let result10 = repeated_n_times_hash_seq(prices1.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result11 = repeated_n_times_best(prices1.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result12 = repeated_n_times_par(prices1.clone());
    let duration = start.elapsed();
    println!("Function Parallel, Time elapsed: {:?}", duration);
    // println!("{}", result0);  // Expected output: 3
    assert_eq!(result10, 3);
    assert_eq!(result10, result11);
    assert_eq!(result11, result12);

    println!("Prices 2");
    let prices2 = vec![2,1,2,5,3,2];
    let start = Instant::now();
    let result20 = repeated_n_times_hash_seq(prices2.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result21 = repeated_n_times_best(prices2.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result22 = repeated_n_times_par(prices2.clone());
    let duration = start.elapsed();
    println!("Function Parallel, Time elapsed: {:?}", duration);
    assert_eq!(result20, 2);
    assert_eq!(result20, result21);
    assert_eq!(result21, result22);

    println!("Prices 3");
    let prices3 = vec![5,1,5,2,5,3,5,4];
    let start = Instant::now();
    let result30 = repeated_n_times_hash_seq(prices3.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result31 = repeated_n_times_best(prices3.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result32 = repeated_n_times_par(prices3.clone());
    let duration = start.elapsed();
    println!("Function Parallel, Time elapsed: {:?}", duration);
    assert_eq!(result30, 5);
    assert_eq!(result30, result31);
    assert_eq!(result31, result32);

    println!("Prices 4");
    let prices4 = vec![1,2,3,5,5,5,5,4];
    let start = Instant::now();
    let result40 = repeated_n_times_hash_seq(prices4.clone());
    let duration = start.elapsed();
    println!("Function Hash seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result41 = repeated_n_times_best(prices4.clone());
    let duration = start.elapsed();
    println!("Function Best seq, Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result42 = repeated_n_times_par(prices4.clone());
    let duration = start.elapsed();
    println!("Function Parallel, Time elapsed: {:?}", duration);
    assert_eq!(result40, 5);
    assert_eq!(result40, result41);
    assert_eq!(result41, result42);

    println!("Prices Nums 1");
    // Read the file content into a string
    let mut file = File::open("input1-20_000.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Remove the square brackets
    content = content.trim().trim_start_matches('[').trim_end_matches(']').to_string();

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
    let resultnums12 = repeated_n_times_par(nums1.clone());
    let duration = start.elapsed();
    println!("Function Parallel, Time elapsed: {:?}", duration);
    assert_eq!(resultnums10, 10_000);
    assert_eq!(resultnums10, resultnums11);
    assert_eq!(resultnums11, resultnums12);

    println!("Prices Nums 2");
    // Read the file content into a string
    let mut file = File::open("input2-20_000.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Remove the square brackets
    content = content.trim().trim_start_matches('[').trim_end_matches(']').to_string();

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
    let resultnums22 = repeated_n_times_par(nums2.clone());
    let duration = start.elapsed();
    println!("Function Parallel, Time elapsed: {:?}", duration);
    assert_eq!(resultnums20, 10_000);
    assert_eq!(resultnums20, resultnums21);
    assert_eq!(resultnums21, resultnums22);

    // let nums = vec![5,1,5,2,5,3,5,4];

    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(4)
    //     .build_global()
    //     .unwrap();

    // let mut file = File::open("input1-10_000.txt")?;
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

    // // let mut i: i32=0;
    // // svg("join.svg", || {
    // //     i = repeated_n_times_par(nums);
    // // }).expect("failed saving log");

    // let start = Instant::now();
    // let b = repeated_n_times_par(nums.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel, Time elapsed: {:?}", duration);
    // assert_eq!(b, 5000);

    // let start = Instant::now();
    // let c = repeated_n_times_par_option(nums.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel Option, Time elapsed: {:?}", duration);
    // assert_eq!(c, 5000);

    // let start = Instant::now();
    // let a = repeated_n_times_par_no_optim(nums.clone());
    // let duration = start.elapsed();
    // println!("Function Parallel No Optim, Time elapsed: {:?}", duration);
    // assert_eq!(a, 5000);

    // print!("{}{}{}", a, b, c);

    // println!("{}", rayon::current_num_threads());

    Ok(())
}
