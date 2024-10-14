use std::collections::VecDeque;

pub fn final_prices_unoptim(prices: Vec<i32>) -> Vec<i32> {
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

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; prices.len()];  // Pre-allocate result vector
    let mut stack: Vec<usize> = Vec::new();  // Stack to store indices of prices

    for i in (0..prices.len()).rev() {
        // Remove elements from the stack if they are greater than the current price
        while let Some(&last) = stack.last() {
            if prices[last] > prices[i] {
                stack.pop();
            } else {
                break;
            }
        }

        // Apply the discount if there is a valid element in the stack
        result[i] = if let Some(&last) = stack.last() {
            prices[i] - prices[last]
        } else {
            prices[i]
        };

        // Push the current index onto the stack
        stack.push(i);
    }

    result
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
