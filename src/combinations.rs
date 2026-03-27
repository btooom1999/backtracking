fn backtracking(
    x: i32,
    n: i32,
    k: i32,
    vec: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if vec.len() as i32 == k {
        result.push(vec.clone());
        return;
    }

    for i in x..=n {
        vec.push(i);
        backtracking(i+1, n, k, vec, result);
        vec.pop();
    }
}

fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    backtracking(1, n, k, &mut vec![], &mut result);

    result
}

pub fn main() {
    let n = 4;
    let k = 2;
    println!("{:?}", combine(n, k));
}
