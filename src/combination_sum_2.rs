fn backtracking(
    mut i: usize,
    target: i32,
    candidates: &Vec<i32>,
    vec: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        result.push(vec.clone());
        return;
    }

    if target < 0 || i == candidates.len() {
        return;
    }

    vec.push(candidates[i]);
    backtracking(i+1, target-candidates[i], candidates, vec, result);
    vec.pop();

    while i+1 < candidates.len() && candidates[i] == candidates[i+1] {
        i += 1;
    }
    backtracking(i+1, target, candidates, vec, result);
}

fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();

    let mut result = Vec::new();

    backtracking(0, target, &candidates, &mut vec![], &mut result);

    result
}

pub fn main() {
    let candidates = [10,1,2,7,6,1,5];
    let target = 8;
    println!("{:?}", combination_sum2(candidates.into(), target));
}

