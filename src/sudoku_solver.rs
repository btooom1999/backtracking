fn backtracking(
    rows: &mut [[bool; 9]; 9],
    cols: &mut [[bool; 9]; 9],
    squares: &mut [[[bool; 9]; 3]; 3],
    board: &mut Vec<Vec<char>>,
    indexes: &[(usize, usize)],
    idx: usize,
) -> bool {
    if idx == indexes.len() {
         return true;
    }

    for k in 0..9 {
        let (i, j) = indexes[idx];
        if !rows[i][k] && !cols[j][k] && !squares[i/3][j/3][k] {
            rows[i][k] = true;
            cols[j][k] = true;
            squares[i/3][j/3][k] = true;
            board[i][j] = ((k+1) as u8 + b'0') as char;
            if backtracking(rows, cols, squares, board, indexes, idx+1) {
                return true;
            }
            squares[i/3][j/3][k] = false;
            rows[i][k] = false;
            cols[j][k] = false;
            board[i][j] = '.';
        }
    }

    false
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut rows = [[false; 9];9];
    let mut cols = [[false; 9];9];
    let mut squares = [[[false; 9]; 3]; 3];
    let mut indexes = Vec::new();
    let (m, n) = (board.len(), board[0].len());
    for i in 0..m {
        for j in 0..n {
            if board[i][j].is_ascii_digit() {
                let k = (board[i][j] as u8 - b'1') as usize;
                rows[i][k] = true;
                cols[j][k] = true;
                squares[i/3][j/3][k] = true;
            } else {
                indexes.push((i, j));
            }
        }
    }

    backtracking(&mut rows, &mut cols, &mut squares, board, &indexes, 0);
}

pub fn main() {
    let mut board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    solve_sudoku(&mut board);
    println!("{:?}", board);
}
