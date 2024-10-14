use std::collections::HashMap;

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut info: HashMap<i32, i32> = HashMap::new();

    // for x in nums {
    //     if info.contains_key(&x) {
    //         return x
    //     } else {
    //         info.insert(x, 1);
    //     }
    // }
    for &x in nums.iter() {
        if let Some(_) = info.get(&x) {
            return x;
        } else {
            info.insert(x, 1);
        }
    }
    0
}

fn main() {
    // Test cases
    let prices1 = vec![1,2,3,3];
    let result1 = repeated_n_times(prices1);
    println!("{}", result1);  // Expected output: 3

    let prices2 = vec![2,1,2,5,3,2];
    let result2 = repeated_n_times(prices2);
    println!("{}", result2);  // Expected output: 2

    let prices3 = vec![5,1,5,2,5,3,5,4];
    let result3 = repeated_n_times(prices3);
    println!("{}", result3);  // Expected output: 5
}
