use std::collections::{HashMap, HashSet};

fn backtracking(
    i: usize,
    word: &[u8],
    hashmap: &mut HashMap<char, Vec<(i32, i32)>>,
    used: &mut HashSet<(i32, i32)>,
    condition: Option<(i32, i32)>,
) -> bool {
    if i >= word.len() {
        return true;
    }

    let directions = match hashmap.get(&(word[i] as char)) {
        Some(v) => v.clone(),
        _ => return false,
    };

    for direction in directions {
        if let Some(condition) = condition
        && (![(direction.0-1, direction.1),
            (direction.0+1, direction.1),
            (direction.0, direction.1-1),
            (direction.0, direction.1+1),
        ].contains(&condition) || used.contains(&direction)) {
            continue;
        }

        used.insert(direction);
        if backtracking(i+1, word, hashmap, used, Some(direction)) {
            return true;
        }
        used.remove(&direction);
    }

    false
}

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut hashmap = HashMap::<char, Vec<(i32, i32)>>::new();
    for (i, chars) in board.iter().enumerate() {
        for (j, char) in chars.iter().enumerate() {
            hashmap.entry(*char).or_default().push((i as i32, j as i32));
        }
    }

    backtracking(0, word.as_bytes(), &mut hashmap, &mut HashSet::new(), None)
}

pub fn main() {
    let board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]
        .into_iter()
        .map(|row| row.map(|v| v.chars().next().unwrap()).to_vec())
        .collect();
    let word = "ABCCED".to_string();
    println!("{}", exist(board, word));
}
