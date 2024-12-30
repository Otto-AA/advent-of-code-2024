use std::fs::read_to_string;

use anyhow::Result;

pub fn parse_input(path: &str) -> Result<Input> {
    let s = read_to_string(path)?;

    let mut locks = vec![];
    let mut keys = vec![];

    for block in s.split("\n\n") {
        let mut item: [u8; 5] = [0, 0, 0, 0, 0];
        let mut lines = block.lines();
        let is_lock = lines.next().unwrap().starts_with("#");
        for line in lines.take(5) {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '#' => item[i] += 1,
                    '.' => {}
                    _ => return Err(anyhow::Error::msg(format!("Invalid char: {c}"))),
                }
            }
        }
        if is_lock {
            locks.push(item);
        } else {
            keys.push(item);
        }
    }

    Ok(Input { locks, keys })
}

pub struct Input {
    pub keys: Vec<[u8; 5]>,
    pub locks: Vec<[u8; 5]>,
}
