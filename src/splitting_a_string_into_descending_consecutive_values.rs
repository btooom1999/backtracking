fn backtracking(
    x: usize,
    bytes: &[u8],
    condition: Option<i128>,
) -> bool {
    let mut num = 0;
    for i in x..bytes.len() {
        num = num * 10 + (bytes[i] - b'0') as i128;
        if let Some(condition) = condition {
            if num < condition {
                continue;
            } else if num > condition {
                return false;
            }
        }

        if condition.is_some() && i+1 == bytes.len() {
            return true;
        }

        if backtracking(i+1, bytes, Some(num-1)) {
            return true;
        }
    }

    false
}

fn split_string(s: String) -> bool {
    backtracking(0, s.as_bytes(), None)
}

pub fn main() {
    let s = "10009998".to_string();
    println!("{}", split_string(s));
}
