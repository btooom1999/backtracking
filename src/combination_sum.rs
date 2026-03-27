use std::collections::HashSet;

fn backtracking(
    i: usize,
    mut target: i32,
    candidates: &Vec<i32>,
    result: &mut HashSet<Vec<i32>>,
    vec: &mut Vec<i32>,
) {
    if target == 0 {
        result.insert(vec.clone());
        return;
    }

    if target < 0 || i == candidates.len() {
        return;
    }

    vec.push(candidates[i]);
    backtracking(i, target-candidates[i], candidates, result, vec);

    vec.pop();
    backtracking(i+1, target, candidates, result, vec);
}

fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();

    let mut result = HashSet::new();
    backtracking(0, target, &candidates, &mut result, &mut vec![]);

    result.into_iter().collect::<Vec<_>>()
}

pub fn main() {
    let candidates = [2,3,6,7];
    let target = 7;
    println!("{:?}", combination_sum(candidates.into(), target));
}
