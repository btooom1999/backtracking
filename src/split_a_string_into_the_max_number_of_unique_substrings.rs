use std::collections::HashSet;

fn backtracking(
    from: usize,
    s: &[u8],
    hashset: &mut HashSet<String>,
) -> i32 {
    if from == s.len() {
        return hashset.len() as i32;
    }

    let mut max = 0;
    let mut str = String::new();
    for i in from..s.len() {
        str.push(s[i] as char);

        if hashset.contains(&str) {
            continue;
        }

        hashset.insert(str.clone());
        max = max.max(backtracking(i+1, s, hashset));
        hashset.remove(&str);
    }

    max
}

fn max_unique_split(s: String) -> i32 {
    let mut hashset = HashSet::new();

    backtracking(0, s.as_bytes(), &mut hashset)
}

pub fn main() {
    let s = "ababccc".to_string();
    println!("{}", max_unique_split(s));
}
