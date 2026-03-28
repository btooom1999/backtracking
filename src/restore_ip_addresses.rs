fn backtracking(
    x: usize,
    len: usize,
    bytes: &[u8],
    ip: &mut Vec<String>,
    result: &mut Vec<String>,
) {
    if x > bytes.len() {
        return;
    }

    if ip.len() == 4 && len == bytes.len() {
        result.push(ip.clone().join("."));
        return;
    }

    let mut num = 0;
    for i in x..bytes.len() {
        num = num * 10 + (bytes[i]-b'0') as i32;

        if num > 255 {
            return;
        }

        let str = num.to_string();
        ip.push(str.clone());
        backtracking(i+1, len+str.len(), bytes, ip, result);
        ip.pop();
    }

}

fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut result = Vec::new();
    backtracking(0, 0, s.as_bytes(), &mut vec![], &mut result);
    result
}

pub fn main() {
    let s = "101023".to_string();
    println!("{:?}", restore_ip_addresses(s));
}
