use std::collections::HashMap;

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    end: bool,
}

impl TrieNode {
    fn new() -> Self {
        const NONE_CHILD: Option<Box<TrieNode>> = None;
        Self { children: [NONE_CHILD; 26], end: false }
    }
}

struct Encrypter {
    keys: Vec<char>,
    values: Vec<String>,
    trie: TrieNode,
    hashmap: (HashMap<char, usize>, HashMap<String, Vec<usize>>), // (keys, values)
}

impl Encrypter {

    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut hashmap_keys = HashMap::new();
        let mut hashmap_values = HashMap::<_, Vec<usize>>::new();
        for i in 0..keys.len() {
            hashmap_keys.insert(keys[i], i);
            hashmap_values.entry(values[i].clone()).or_default().push(i);
        }

        let mut trie = TrieNode::new();
        for word in dictionary {
            let mut trie = &mut trie;
            for b in word.as_bytes() {
                trie = trie.children[(b-b'a') as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
            }
            trie.end = true;
        }

        Self { keys, values, trie, hashmap: (hashmap_keys, hashmap_values) }
    }

    fn encrypt(&self, word1: String) -> String {
        word1.chars().fold(String::new(), |mut str, c| {
            str.push_str(&self.values[*self.hashmap.0.get(&c).unwrap()]);
            str
        })
    }

    fn count(
        trie: &TrieNode,
        keys: &Vec<char>,
        hashmap: &HashMap<String, Vec<usize>>,
        word: &[u8],
        start: usize,
        end: usize,
    ) -> i32 {
        if start == word.len() {
            return trie.end as i32;
        }

        let mut count = 0;
        for &i in hashmap.get(&String::from_utf8(word[start..end].to_vec()).unwrap()).unwrap_or(&vec![]) {
            if let Some(trie) = trie.children[(keys[i] as u8 - b'a') as usize].as_deref() {
                count += Self::count(trie, keys, hashmap, word, start+2, end+2);
            }
        }

        count
    }

    fn decrypt(&self, word2: String) -> i32 {
        Self::count(&self.trie, &self.keys, &self.hashmap.1, word2.as_bytes(), 0, 2)
    }
}

pub fn main() {
    let keys = ['a', 'b', 'c', 'd'].to_vec();
    let values = ["ei", "zf", "ei", "am"].into_iter().map(String::from).collect();
    let dictionary = ["abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad"].into_iter().map(String::from).collect();
    let mut encrypter = Encrypter::new(keys, values, dictionary);
    println!("{}", encrypter.encrypt("abcd".to_string()));
    println!("{}", encrypter.decrypt("eizfeiam".to_string()));
}
