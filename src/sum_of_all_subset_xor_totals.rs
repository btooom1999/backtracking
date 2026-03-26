fn backtracking(
    i: usize,
    sum: i32,
    nums: &Vec<i32>,
) -> i32 {
    if i == nums.len() {
        return sum;
    }

    backtracking(i+1, sum, nums) + backtracking(i+1, sum ^ nums[i], nums)
}

fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    backtracking(0, 0, &nums)
}

pub fn main() {
    println!("{}", subset_xor_sum([1,3].into()));
}
