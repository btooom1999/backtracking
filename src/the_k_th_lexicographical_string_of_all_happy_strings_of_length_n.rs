use std::{cmp::Reverse, collections::BinaryHeap};

fn backtracking(
    i: i32,
    n: i32,
    chars: &[char],
    map: &mut Vec<char>,
    min_heap: &mut BinaryHeap<Reverse<String>>,
) {
    if i == n {
        min_heap.push(Reverse(map.clone().into_iter().collect()));
        return;
    }

    for c in chars {
        if map.last().is_some_and(|v| v == c) {
            continue;
        }

        map.push(*c);
        backtracking(i+1, n, chars, map, min_heap);
        map.pop();
    }
}

fn get_happy_string(n: i32, mut k: i32) -> String {
    let chars = ['a', 'b', 'c'];
    let mut min_heap = BinaryHeap::new();

    backtracking(0, n, &chars, &mut vec![], &mut min_heap);

    while k > 1 && min_heap.pop().is_some() {
        k -= 1;
    }

    min_heap.pop().unwrap_or(Reverse(String::new())).0
}

pub fn main() {
    let n = 3;
    let k = 9;
    println!("{}", get_happy_string(n, k));
}
