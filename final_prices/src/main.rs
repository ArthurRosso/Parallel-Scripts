use std::collections::VecDeque;

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut deque: VecDeque<i32> = VecDeque::new();  // To store the result
    let mut stack: Vec<i32> = Vec::new();  // To act as a stack for the discounts

    prices.iter().rev().for_each(|&x| {
        // Find the next smaller or equal element
        while let Some(&last) = stack.last() {
            if last > x {
                stack.pop();
            } else {
                break;
            }
        }
        
        // If there's a valid discount, apply it, otherwise no discount (x stays the same)
        if let Some(&discount) = stack.last() {
            deque.push_front(x - discount);
        } else {
            deque.push_front(x);
        }
        
        // Push the current price onto the stack
        stack.push(x);
    });
    
    Vec::from(deque)
}

fn main() {
    // Test cases
    let prices1 = vec![5, 4, 10, 2, 6, 1, 1, 1, 9, 1];
    let result1 = final_prices(prices1);
    println!("{:?}", result1);  // Expected output: [1, 2, 9, 1, 5, 1, 0, 0, 9, 1]

    let prices2 = vec![8, 7, 4, 2, 8, 1, 7, 7, 10, 1];
    let result2 = final_prices(prices2);
    println!("{:?}", result2);  // Expected output: [6, 5, 2, 2, 7, 1, 0, 0, 9, 1]
}
