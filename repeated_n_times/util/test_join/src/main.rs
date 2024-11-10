use diam::join;
use diam::svg;
use itertools::Itertools;
// use rayon::join;

fn slice_sum(s: &[u64]) -> u64 {
    if s.len() < 10_000{
        s.iter().sum()
    } else {
        let mid = s.len()/2;
        let (left, right) =s.split_at(mid);
        let (left_sum, right_sum) = join(|| slice_sum(left),
                                         || slice_sum(right));
        left_sum+right_sum
    }
}

fn main() {
    let nums: Vec<u64> = std::iter::repeat_with(rand::random).take(1_000_000).collect();
    let mut i: u64=0;
    svg("test1.svg", || {
        i = slice_sum(nums.as_slice());
    }).expect("failed saving log");

    println!("{}",i);
}
