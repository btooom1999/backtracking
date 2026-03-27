fn backtracking(
    exceed: (i32, i32, i32),
    n: i32,
    vec: &mut Vec<char>,
    result: &mut Vec<String>,
) {
    if exceed.0 > n || exceed.1 > n || exceed.2 > n || exceed.2 < 0 {
        return;
    }

    if vec.len() as i32 == n*2 {
        result.push(vec.clone().into_iter().collect::<String>());
        return;
    }

    vec.push('(');
    backtracking((exceed.0+1, exceed.1, exceed.2+1), n, vec, result);
    vec.pop();

    vec.push(')');
    backtracking((exceed.0, exceed.1+1, exceed.2-1), n, vec, result);

    vec.pop();
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = Vec::new();
    backtracking((0, 0, 0), n, &mut vec![], &mut result);

    result
}

pub fn main() {
    let n = 3;
    println!("{:?}", generate_parenthesis(n));
}
