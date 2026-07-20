const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn backtracking(
    i: usize,
    j: usize,
    m: usize,
    n: usize,
    boxes: i32,
    grid: &mut Vec<Vec<i32>>,
) -> i32 {
    if grid[i][j] == 2 {
        return (boxes < 0) as i32;
    }

    let mut count = 0;
    for direct in DIRECTIONS {
        let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
        let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
        if i < m && j < n {
            let num = grid[i][j];
            if num == 0 || num == 2 {
                if num == 0 { grid[i][j] = 1; }
                count += backtracking(i, j, m, n, boxes-1, grid);
                if num == 0 { grid[i][j] = 0; }
            }
        }
    }

    count
}

fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut start = (0, 0);
    let mut boxes = 0;
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 { start = (i, j) }
            else if grid[i][j] == 0 { boxes += 1; }
        }
    }

    backtracking(start.0, start.1, m, n, boxes, &mut grid)
}

pub fn main() {
    let grid = [[1,0,0,0],[0,0,0,0],[0,0,2,-1]].into_iter().map(Vec::from).collect();
    println!("{}", unique_paths_iii(grid));
}
