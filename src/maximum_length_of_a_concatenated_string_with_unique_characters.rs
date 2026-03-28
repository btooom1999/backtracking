use std::collections::HashSet;

fn backtracking(
    from: usize,
    amount: i32,
    arr: &Vec<String>,
    hashset1: Vec<bool>,
    hashset2: &mut HashSet<String>,
) -> i32 {
    let mut max = amount;
    for i in from..arr.len() {
        let str = arr[i].clone();
        if hashset2.contains(&str) {
            continue;
        }

        let mut new_hashset = hashset1.clone();
        let mut has_duplication = false;
        for b in str.as_bytes() {
            let i = (b - b'a') as usize;
            if hashset1[i] || new_hashset[i] {
                has_duplication = true;
                break;
            }

            new_hashset[i] = true;
        }

        if has_duplication {
            continue;
        }

        hashset2.insert(str.clone());
        max = std::cmp::max(max, backtracking(i+1, amount+str.len() as i32, arr, new_hashset, hashset2));
        hashset2.remove(&str);
    }

    max
}

fn max_length(arr: Vec<String>) -> i32 {
    backtracking(0, 0, &arr, vec![false; 26], &mut HashSet::new())
}

pub fn main() {
    let arr = ["aa", "bb"].into_iter().map(String::from).collect();
    println!("{}", max_length(arr));
}
