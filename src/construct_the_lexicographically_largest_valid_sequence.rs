fn backtracking(
    i: usize,
    n: usize,
    vec: &mut Vec<i32>,
    hashset: &mut Vec<bool>
) -> Vec<i32> {
    for num in (1..=n).rev() {
        if hashset[num] || ((i+num >= vec.len() || vec[i+num] != -1) && num != 1) {
            continue;
        }

        hashset[num] = true;
        vec[i] = num as i32;
        if num != 1 {
            vec[i+num] = num as i32;
        }

        if let Some(i) = vec.iter().position(|v| *v == -1) {
            let res = backtracking(i, n, vec, hashset);
            if !res.is_empty() {
                return res;
            }
        } else {
            return vec.clone();
        }

        hashset[num] = false;
        vec[i] = -1;
        if num != 1 {
            vec[i+num] = -1;
        }
    }

    vec![]
}

fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    backtracking(0, n as usize, &mut vec![-1; ((n-1)*2+1) as usize], &mut vec![false; n as usize + 1])
}

pub fn main() {
    let n = 9;
    println!("{:?}", construct_distanced_sequence(n));
}
