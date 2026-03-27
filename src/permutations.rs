use std::collections::HashSet;

fn backtracking(
    nums: &Vec<i32>,
    vec: &mut Vec<i32>,
    hashset: &mut HashSet<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if vec.len() == nums.len() {
        result.push(vec.clone());
        return;
    }

    for i in 0..nums.len() {
        vec.push(nums[i]);

        if !hashset.contains(&nums[i]) {
            hashset.insert(nums[i]);
            backtracking(nums, vec, hashset, result);
            hashset.remove(&nums[i]);
        }

        vec.pop();
    }
}

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    backtracking(&nums, &mut vec![], &mut HashSet::new(), &mut result);

    result
}

pub fn main() {
    println!("{:?}", permute([1,2,3].into()));
}
