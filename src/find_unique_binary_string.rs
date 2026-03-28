use std::collections::HashSet;

fn backtracking(
    i: usize,
    n: usize,
    str: &mut String,
    hashset: &HashSet<String>
) -> String {
    if i == n {
        if !hashset.contains(str) {
            return str.clone();
        }

        return String::new();
    }

    for bit in ['0', '1'] {
        str.push(bit);
        let res = backtracking(i+1, n, str, hashset);
        if !res.is_empty() {
            return res;
        }
        str.pop();
    }

    String::new()
}

fn find_different_binary_string(nums: Vec<String>) -> String {
    backtracking(0, nums.len(), &mut String::new(), &nums.into_iter().collect())
}

pub fn main() {
    let nums = ["00","01"].into_iter().map(String::from).collect();
    println!("{}", find_different_binary_string(nums));
}

