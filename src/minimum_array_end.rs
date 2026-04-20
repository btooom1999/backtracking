fn min_end(n: i32, x: i32) -> i64 {
    let mut x = x as i64;
    let mut n = (n-1) as i64;
    let mut pos = 0;

    while n > 0 {
        if (1 << pos) & x == 0 {
            x |= (n & 1) << pos;
            n >>= 1;
        }
        pos += 1;
    }

    x
}

pub fn main() {
    let n = 6715154;
    let x = 7193485;
    println!("{}", min_end(n, x));
}
