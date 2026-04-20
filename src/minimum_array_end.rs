fn min_end(n: i32, x: i32) -> i64 {
    let mut n = n-1;
    let mut bits = [0;64];
    for i in 0..32 {
        bits[i] = (x >> i) & 1;
    }

    let mut stack = Vec::new();
    while n > 0 {
        stack.push(n & 1);
        n >>= 1;
    }

    stack.reverse();
    for i in 0..64 {
        if bits[i] == 0 {
            if let Some(bit) = stack.pop() {
                bits[i] = bit;
            } else {
                break;
            }
        }
    }

    let mut res = 0;
    for i in 0..64 {
        if bits[i] == 1 {
            res |= (1 << i);
        }
    }

    res
}

pub fn main() {
    let n = 6715154;
    let x = 7193485;
    println!("{}", min_end(n, x));
}
