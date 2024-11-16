# 961. N-Repeated Element in Size 2N Array
The challenge, involves finding an element that repeats exactly N times in an array of size 2N. Given the constraints, the repeated element will appear frequently, making it feasible to optimize for early detection.
Challenge available at: https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/

## Initial Intuition and Implementation
The first approach used a HashMap to track element occurrences:
```rust []
use std::collections::HashMap;
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
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
}
```
This solution is functionally correct but not optimal, with a runtime complexity of O(N) and additional space complexity of O(N) due to the HashMap. On LeetCode, this implementation only performs the best 20% of submissions.

Upon analyzing the problem, it becomes clear that:

1. If an element appears N times in an array of size 2N, then:
- The repeated element will either appear among the first four elements.
- Or, there will be two adjacent identical elements somewhere in the array.
From this insight, a more efficient approach was implemented:
```rust []
impl Solution {
    pub fn repeated_n_times_best(nums: Vec<i32>) -> i32 {
        if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
            return nums[0];
        } else if nums[1] == nums[3] {
            return nums[1];
        }

        let mut iter = nums.windows(2);
        while let Some(x) = iter.next() {
            if x[0] == x[1] {
                return x[0];
            }
        }

        0
    }
}
```
This solution is O(k) where k is the first point where two equals elements are together, it avoids extra space and beats 100% of submissions in runtime on LeetCode.

## Parallel Approaches
<!-- Describe your approach to solving the problem. -->
An initial parallel implementation used recursion with rayon::join:
```rust []
impl Solution {
    pub fn repeated_2_times(nums: Vec<i32>) -> i32 {
        if nums.len() > 2 {
            let mut iter = nums.into_iter();
            let rx = iter.next().unwrap();
            let ry = iter.next().unwrap();
            let (ra, rb) = rayon::join(
                || {
                    if rx == ry { rx } else { 0 }
                },
                || {
                    repeated_2_times(iter.collect())
                }
            );
            return ra | rb;
        }
        return 0;
    }
    pub fn repeated_n_times_par(nums: Vec<i32>) -> i32 {
        let r = repeated_2_times(nums.clone());
        if r == 0 {
            if (nums[0] == nums[2]) || (nums[0] == nums[3]) {
                return nums[0];
            } else if nums[1] == nums[3] {
                return nums[1];
            }
        }
        return r;
    }
}
```
However, this approach does not benefit from interruption if the two desired numbers are found.

To fix this lack, a block-based approach combined was implemented:
```rust []
impl Solution {
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
}
```
But, this approach suffers from stack overflow for input sizes around 10,000 elements. The recursion depth is too high, and one thread handles an excessive stack load, causing the overflow.

To address the stack overflow and improve parallelism, a block-based approach combined with parallel folds was implemented:
```rust []
impl Solution {
    pub fn repeated_2_times(nums: &[i32]) -> Option<i32> {
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

    pub fn repeated_n_times_block(nums: Vec<i32>) -> i32 {
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
            if let Some(r) = repeated_2_times(chunk) {
                return r;
            }
        }
        0
    }
}
```

## Complexity
### Join
The repeated_n_times_join function uses a divide-and-conquer approach with the help of the join combinator to split tasks and solve them concurrently. The function processes blocks of increasing size to find the repeated element. To analyze its complexity, we will apply the Master Theorem.
#### Function Analysis:
1. Initial setup:
    - If we have a repetition on the first 4 elements, the function immediately returns, which is O(1).
2. Splits the input recursively using join into two tasks (repeated_2_times):
    - Task 1: Checks if the first two elements are the same (O(1)).
    - Task 2: Recursively processes the rest of the slice.
    - The results are combined using or.

![join with 100 elements](results/join.svg)


#### Problems
This approach has a stack overflow issue in one of the threads when the input size reaches around 10,000 elements. The issue arises because the repeated_2_times_join function uses recursion to process the input slice. 

For each recursive call:
- A new stack frame is created.
- In this recursive structure, each call adds to the stack depth, leading to excessive memory usage on the stack.

The join combinator spawns two tasks:
- The first task (Task 1) checks two elements and returns immediately.
- The second task (Task 2) performs the recursive calls. 

Since the recursion is concentrated in Task 2, one of the threads ends up handling a deep recursive chain, consuming a large amount of stack space.


### Blocks
The repeated_n_times_block function divides the problem into smaller subproblems and uses a growing block size to find the repeated element. To analyze its complexity, we will apply the Master Theorem.
#### Function Analysis:
1. Initial setup:
    - If we have a repetition on the first 4 elements, the function immediately returns, which is O(1).
2. Iterative Block Splitting:
    - The function splits the input into blocks of increasing size (4, 8, 16, ...) until it processes the entire array. This happens using blocks.next() in the iterator.
3. Subproblem Work (repeated_2_times):
    - Each block is processed by repeated_2_times.
    - repeated_2_times checks for repeated adjacent elements using par_windows(2) with a cost proportional to the block size.

#### Recurrence Relation:
For a total input size n:
- The first block size is 4.
- The seconde block size is 8, then 16, and so on.

If n is the size of the array, and we double the block size each interation:
- Number of blocks is at most log(n).
- The cost of processing each block is proportional to its size.

![block with 100 elements](results/block.svg)

## Results
Nums 0 = vec![1, 2, 3, 3]
Function Hash seq, Time elapsed: 19.334µs
Function Best seq, Time elapsed: 167ns
Function Parallel Block, Time elapsed: 249.459µs
Function Parallel Joins, Time elapsed: 60.792µs

Nums 1 = vec![1, 3, 2, 3]
Function Hash seq, Time elapsed: 667ns
Function Best seq, Time elapsed: 42ns
Function Parallel Block, Time elapsed: 125ns
Function Parallel Joins, Time elapsed: 125ns

Nums 2 = vec![2, 1, 2, 5, 3, 2]
Function Hash seq, Time elapsed: 250ns
Function Best seq, Time elapsed: 84ns
Function Parallel Block, Time elapsed: 83ns
Function Parallel Joins, Time elapsed: 42ns

Nums 3 = vec![5, 1, 5, 2, 5, 3, 5, 4]
Function Hash seq, Time elapsed: 292ns
Function Best seq, Time elapsed: 84ns
Function Parallel Block, Time elapsed: 83ns
Function Parallel Joins, Time elapsed: 84ns

Nums 4 = vec![1, 2, 3, 5, 5, 5, 5, 4]
Function Hash seq, Time elapsed: 2.542µs
Function Best seq, Time elapsed: 167ns
Function Parallel Block, Time elapsed: 68.125µs
Function Parallel Joins, Time elapsed: 69.625µs

Input Nums 1 = Vector with increasing numbers 20_000 elements from a file (the repetion from the 10_000 element)
Function Hash seq, Time elapsed: 465.875µs
Function Best seq, Time elapsed: 10.625µs
Function Parallel Block, Time elapsed: 668.167µs
Function Parallel Joins, Time elapsed: 1.364292ms

Input Nums 1 = Vector with increasing numbers 2_000_000 elements from a file (the repetion from the 1_000_000th element)
Function Hash seq, Time elapsed: 45.05ms
Function Best seq, Time elapsed: 921.125µs
Function Parallel Block, Time elapsed: 1.728417ms

Input Nums 2 = Vector with alternating numbers 20_000 elements from a file (the repetion from the 2nd element)
Function Hash seq, Time elapsed: 2.916µs
Function Best seq, Time elapsed: 2µs
Function Parallel Block, Time elapsed: 1.667µs
Function Parallel Joins, Time elapsed: 1.625µs
