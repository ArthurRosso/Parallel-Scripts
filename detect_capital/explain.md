# 520. Detect Capital
Challenge available at: https://leetcode.com/problems/detect-capital/description/

## Intuition
<!-- Describe your first thoughts on how to solve this problem. -->

## Approach
<!-- Describe your approach to solving the problem. -->

## Complexity
- Time complexity:
<!-- Add your time complexity here, e.g. $$O(n)$$ -->

- Space complexity:
<!-- Add your space complexity here, e.g. $$O(n)$$ -->

## Code
```rust []
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let is_fcapital: bool = word.chars().nth(0).unwrap().is_uppercase();
        let is_scapital: bool = word.chars().nth(1).unwrap_or('a').is_uppercase();
        
        if !is_fcapital && is_scapital {
            return false;
        }
        
        word.chars().skip(2).fold(true, |acc, x| if x.is_uppercase() != is_scapital {false} else {acc})
    }
}
```