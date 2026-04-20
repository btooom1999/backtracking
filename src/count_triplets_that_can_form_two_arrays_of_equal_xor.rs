fn count_triplets(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut res = 0;
    for i in 0..n {
        let mut xor = 0;
        for j in i..n {
            xor ^= arr[j];
            if xor == 0 {
                res += (j - i);
            }
        }
    }

    res as i32
}

pub fn main() {
    // let arr = [2,3,1,6,7].to_vec();
    let arr = [1; 300].to_vec();
    println!("{}", count_triplets(arr));
}
