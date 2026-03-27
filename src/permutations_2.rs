use std::collections::HashSet;

fn backtracking(
    hashset: &mut HashSet<usize>,
    nums: &Vec<i32>,
    vec: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if vec.len() == nums.len() {
        result.push(vec.clone());
        return;
    }

    for i in 0..nums.len() {
        if hashset.contains(&i) {
            continue;
        }

        if i > 0 && nums[i] == nums[i-1] && !hashset.contains(&(i-1)) {
            continue;
        }

        vec.push(nums[i]);
        hashset.insert(i);
        backtracking(hashset, nums, vec, result);
        hashset.remove(&i);
        vec.pop();
    }
}

fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut result = Vec::new();

    backtracking(&mut HashSet::new(), &nums, &mut vec![], &mut result);

    result
}

pub fn main() {
    let nums = [1,1,2];
    println!("{:?}", permute_unique(nums.into()));
}
