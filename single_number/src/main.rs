// Runtime: 0ms Beats 100.00%
// Memory: 2.26MB Beats 45.71%
pub fn single_number_fold(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, e| acc ^ e)
}

// Runtime: 0ms Beats 100.00%
// Memory: 2.32MB Beats 13.64%
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|acc, e| acc ^ e).unwrap()
}

// Runtime: 0ms Beats 100.00%
// Memory: 2.30MB Beats 45.71%
pub fn single_number_for(nums: Vec<i32>) -> i32 {
    let mut acc = 0;
    for &x in nums.iter() {
        acc  ^= x;
    }
    acc
}


fn main() {
    // Test cases
    let nums1 = vec![2,2,1];
    let result1 = single_number(nums1);
    println!("{}", result1);  // Expected output: 1

    let nums2 = vec![4,1,2,1,2];
    let result2 = single_number(nums2);
    println!("{}", result2);  // Expected output: 4

    let nums3 = vec![1];
    let result3 = single_number(nums3);
    println!("{}", result3);  // Expected output: 1
}
