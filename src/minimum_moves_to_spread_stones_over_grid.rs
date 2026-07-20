fn dfs(
    idx: usize,
    grid: &mut Vec<Vec<i32>>,
    exceed: &Vec<(usize, usize)>,
    lack: &Vec<(usize, usize)>,
    visited: &mut i32,
    total: i32,
) -> i32 {
    if idx == exceed.len() { return total };

    let mut res = 100;
    for k in 0..lack.len() {
        if *visited & 1 << k == 0 {
            let (i, j) = exceed[idx];
            *visited ^= 1 << k;
            grid[i][j] -= 1;
            let steps = (lack[k].0 as i32 - i as i32).abs() + (lack[k].1 as i32 - j as i32).abs();
            res = res.min(dfs(if grid[i][j] == 1 { idx+1 } else { idx }, grid, exceed, lack, visited, total+steps));
            grid[i][j] += 1;
            *visited ^= 1 << k;

        }
    }

    res
}

fn minimum_moves(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let mut exceed = Vec::new();
    let mut lack = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            if grid[i][j] > 1 {
                exceed.push((i, j));
            } else if grid[i][j] == 0 {
                lack.push((i, j));
            }
        }
    }

    dfs(0, &mut grid, &exceed, &lack, &mut 0, 0)
}

pub fn main() {
    let grid = [
        [1,2,2],
        [1,1,0],
        [0,1,1]
    ].into_iter().map(Vec::from).collect();
    // let grid = [[3,2,0],[0,1,0],[0,3,0]].into_iter().map(Vec::from).collect();
    println!("{}", minimum_moves(grid));
}
