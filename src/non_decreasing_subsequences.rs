use std::collections::HashSet;

fn dfs(
    i: usize,
    n: usize,
    nums: &Vec<i32>,
    dp: &mut Vec<Option<HashSet<Vec<i32>>>>,
) -> HashSet<Vec<i32>> {
    if i == n {
        return HashSet::new();
    }

    if dp[i].is_some() {
        return dp[i].clone().unwrap();
    }

    dfs(i+1, n, nums, dp);

    let num = nums[i];
    let mut vec = HashSet::from([vec![num]]);
    for x in i+1..n {
        if num > nums[x] {
            continue;
        }

        for new_set in dfs(x, n, nums, dp) {
            let mut set = vec![num];
            set.extend(new_set);
            vec.insert(set);
        }
    }

    dp[i] = Some(vec.clone());
    vec
}

fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut dp = vec![None; nums.len()];
    dfs(0, nums.len(), &nums, &mut dp);

    dp
        .into_iter()
        .flat_map(|v| v.unwrap().into_iter().collect::<Vec<_>>())
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|v| v.len() > 1)
        .collect::<Vec<_>>()
}

pub fn main() {
    let nums = [4,6,7,7].to_vec();
    println!("{:?}", find_subsequences(nums));
}
