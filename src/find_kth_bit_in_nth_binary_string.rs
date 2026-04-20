fn dfs(len: i32, k: i32) -> char {
    if len == 1 {
        return '0';
    }

    let half = len / 2;
    if k <= half {
       dfs(half, k)
    } else if k > half+1 {
        let res = dfs(half, 1+len-k);
        if res == '0' {
            '1'
        } else {
            '0'
        }
    } else {
        '1'
    }
}

fn find_kth_bit(n: i32, k: i32) -> char {
    dfs(2i32.pow(n as u32)-1, k)
}

pub fn main() {
    let n = 3;
    let k = 5;
    println!("{}", find_kth_bit(n, k));
}
