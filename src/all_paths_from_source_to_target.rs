fn dfs(
    graph: &Vec<Vec<i32>>,
    i: usize,
    visited: &mut i32,
    vec: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    *visited ^= 1 << i;
    vec.push(i as i32);
    for &k in &graph[i] {
        if *visited & 1 << k == 0 {
            dfs(graph, k as usize, visited, vec, result);
        }
    }

    if let Some(&last) = vec.last() && last == (graph.len()-1) as i32 { result.push(vec.clone()); }
    *visited ^= 1 << i;
    vec.pop();
}

fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    dfs(&graph, 0, &mut 0, &mut vec![], &mut result);
    result
}

pub fn main() {
    let graph = vec![vec![4,3,1],vec![3,2,4],vec![3],vec![4],vec![]];
    println!("{:?}", all_paths_source_target(graph));
}
