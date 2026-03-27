fn backtracking(
    amount: i32,
    tiles: &[u8],
    hashset: &mut Vec<bool>,
    result: &mut i32,
) {
    if amount > tiles.len() as i32 {
        return;
    }

    for i in 0..tiles.len() {
        if hashset[i] {
            continue;
        }

        if i > 0 && tiles[i] == tiles[i-1] && !hashset[i-1] {
            continue;
        }

        hashset[i] = true;
        backtracking(amount + 1, tiles, hashset, result);
        hashset[i] = false;
        *result += 1;
    }
}

fn num_tile_possibilities(tiles: String) -> i32 {
    let mut tiles = tiles.into_bytes();
    tiles.sort();

    let mut result = 0;

    backtracking(0, &tiles, &mut vec![false; tiles.len()], &mut result);

    result
}

pub fn main() {
    let tiles = "AAB";
    println!("{}", num_tile_possibilities(tiles.to_string()));
}
