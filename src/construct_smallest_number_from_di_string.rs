fn backtracking(
    i: i32,
    prev_num: usize,
    pattern: &[u8],
    str: &mut Vec<u8>,
    used: &mut Vec<bool>,
) -> String {
    if i == pattern.len() as i32 {
        return String::from_utf8(str.clone()).unwrap();
    }

    let range = if i < 0 { (1..10) } else if pattern[i as usize] == b'I' { (prev_num+1..10) } else { (1..prev_num) };
    for num in range {
        if used[num] {
            continue;
        }

        used[num] = true;
        str.push(num as u8 + b'0');

        let res = backtracking(i+1, num, pattern, str, used);
        if !res.is_empty() {
            return res;
        }

        str.pop();
        used[num] = false;
    }

    String::new()
}

fn smallest_number(pattern: String) -> String {
    let mut used = vec![false; 10];

    backtracking(-1, 0, pattern.as_bytes(), &mut vec![], &mut used)
}

pub fn main() {
    let pattern = "I".to_string();
    println!("{:?}", smallest_number(pattern));
}
