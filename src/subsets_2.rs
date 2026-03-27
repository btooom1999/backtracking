fn backtracking(
    mut i: usize,
    vec: &mut Vec<i32>,
    nums: &Vec<i32>,
    result: &mut Vec<Vec<i32>>
) {
    if i == nums.len() {
        return;
    }

    vec.push(nums[i]);
    result.push(vec.clone());
    backtracking(i+1, vec, nums, result);
    vec.pop();

    while i+1 < nums.len() && nums[i] == nums[i+1] {
        i += 1;
    }

    backtracking(i+1, vec, nums, result);
}

fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut result = Vec::new();
    result.push(vec![]);

    backtracking(0, &mut vec![], &nums, &mut result);
    result
}

pub fn main() {
    let nums = [1,2,2];
    println!("{:?}", subsets_with_dup(nums.into()));
}
