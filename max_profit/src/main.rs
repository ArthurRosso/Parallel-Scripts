// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/

use rayon::prelude::*;

// Brute force solution
// Runtime: 692 ms
fn max_profit_bf(prices: Vec<i32>) -> i32 {
    let mut total_profit: i32 = 0;

    // Iterate through the prices
    for (index, &x) in prices.iter().enumerate() {
        // Find the maximum profit we can get after day `index`
        let profit = prices
            .iter()
            .skip(index + 1)  // Skip the current day and look for future prices
            .map(|&y| y - x)  // Calculate profit by subtracting current day's price
            .max()            // Find the maximum profit
            .unwrap_or(0);    // Default to 0 if no profit is found
        
        if profit > total_profit {
            total_profit = profit;
        }
    }

    total_profit
}

// DP solution
// Runtime: 9 ms
fn max_profit_for(prices: Vec<i32>) -> i32 {
    let mut cheapest: i32 = prices[0];
    let mut profit: i32 = 0;

    prices.iter().for_each(|x| {
        if (*x) < cheapest {
            cheapest = *x;
        } else {
            if (*x)-cheapest > profit {
                profit = (*x)-cheapest;
            }
        }
    });
    profit
}

// Runtime: 7 ms
fn max_profit_fold(prices: Vec<i32>) -> i32 {
    // Use fold to track both the cheapest price and the max profit
    let (_, profit) = prices.iter().fold((prices[0], 0), |(cheapest, profit), &price| {
        // Update the cheapest price and max profit
        let new_cheapest = cheapest.min(price);
        let new_profit = profit.max(price - new_cheapest);
        (new_cheapest, new_profit)
    });

    profit
}

fn max_profit_par(prices: Vec<i32>) -> i32 {
    let (min_index, &min_val) = prices.par_iter().enumerate().min_by_key(|&(_, &v)| v).unwrap();

    if let Some(profit) = prices.into_par_iter().skip_any(min_index + 1).max() {
        return profit - min_val;
    }
    return 0;
}


fn main() {
    let prices: Vec<i32> = [7,1,5,3,6,4].to_vec();
    println!("{:?}", max_profit_par(prices.clone()));
    assert_eq!(max_profit_par(prices.clone()), max_profit_for(prices.clone()));
    assert_eq!(max_profit_bf(prices.clone()), max_profit_fold(prices.clone()));
}
