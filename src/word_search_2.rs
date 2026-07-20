use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        const NONE_CHILD: Option<Box<TrieNode>> = None;
        const NONE_WORD: Option<String> = None;
        Self { children: [NONE_CHILD; 26], word: NONE_WORD }
    }
}

fn backtracking(
    trie: Option<&TrieNode>,
    board: &mut Vec<Vec<char>>,
    index: (usize, usize),
    len: (usize, usize),
    hashset: &mut HashSet<String>,
) {
    if let Some(trie) = trie {
        if let Some(str) = trie.word.as_ref() { hashset.insert(str.clone()); }
        let (i, j) = index;
        let (m, n) = len;
        let letter = board[i][j];
        board[i][j] = '#';

        for direct in DIRECTIONS {
            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if i < m && j < n && board[i][j] != '#' {
                backtracking(trie.children[(board[i][j] as u8 - b'a') as usize].as_deref(), board, (i, j), len, hashset);
            }
        }

        board[i][j] = letter;
    }
}

fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut trie = TrieNode::new();

    for word in words {
        let mut trie = &mut trie;
        for b in word.as_bytes() {
            trie = trie.children[(b - b'a') as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        trie.word = Some(word);
    }

    let mut hashset = HashSet::new();
    let (m, n) = (board.len(), board[0].len());
    for i in 0..m {
        for j in 0..n {
            backtracking(trie.children[(board[i][j] as u8 - b'a') as usize].as_deref(), &mut board, (i, j), (m, n), &mut hashset);
        }
    }

    hashset.into_iter().collect()
}

pub fn main() {
    let board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    let words = ["oath","pea","eat","rain"].into_iter().map(String::from).collect();
    println!("{:?}", find_words(board, words));
}
