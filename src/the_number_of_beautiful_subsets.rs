use std::collections::HashMap;

fn backtracking(
    from: usize,
    k: i32,
    nums: &Vec<i32>,
    hashmap: &mut HashMap<i32, i32>,
    count: &mut i32,
    vec: &mut Vec<i32>,
) {
    for i in from..nums.len() {
        if hashmap.get(&(nums[i]-k)).is_some_and(|v| *v > 0) {
            continue;
        }

        *count += 1;
        vec.push(nums[i]);
        *hashmap.entry(nums[i]).or_default() += 1;
        backtracking(i+1, k, nums, hashmap, count, vec);
        vec.pop();
        hashmap.entry(nums[i]).and_modify(|v| *v -= 1);
    }
}

fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    let mut count = 0;
    backtracking(0, k, &nums, &mut HashMap::new(), &mut count, &mut vec![]);

    count
}

pub fn main() {
    let nums = [1,1,2,3];
    let k = 1;
    println!("{:?}", beautiful_subsets(nums.into(), k));
}
