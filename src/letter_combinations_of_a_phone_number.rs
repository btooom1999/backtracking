fn backtracking(
    i: usize,
    digits: &[u8],
    set: &Vec<Vec<char>>,
    map: &mut Vec<char>,
    result: &mut Vec<String>,
) {
    if i == digits.len() {
        result.push(map.clone().into_iter().collect());
        return;
    }

    for c in &set[(digits[i]-b'2') as usize] {
        map.push(*c);
        backtracking(i+1, digits, set, map, result);
        map.pop();
    }
}

fn letter_combinations(digits: String) -> Vec<String> {
    let set = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ];

    let mut result = Vec::new();
    backtracking(0, digits.as_bytes(), &set, &mut vec![], &mut result);
    result
}

pub fn main() {
    let digits = "23".to_string();
    println!("{:?}", letter_combinations(digits));
}
