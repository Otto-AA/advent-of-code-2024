use regex::Regex;
use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::zip,
    path::Path,
};

fn main() {
    let args: Vec<String> = args().collect();
    let path: &Path = Path::new(args.get(1).expect("Please specify the input path"));

    let (left, right) = read_input(path).unwrap();

    println!("Distance: {}", distance(&left, &right));
    println!("Similarity: {}", similarity(&left, &right));
}

fn read_input(path: &Path) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let x: Vec<&str> = Regex::new(r"\s+").unwrap().split(&line).collect();
        if x.len() != 2 {
            println!("{:?}", x);
            panic!("Unexpected length: {}", x.len());
        }
        left.push(x[0].parse().unwrap());
        right.push(x[1].parse().unwrap());
    }

    Ok((left, right))
}

fn distance(left: &[i32], right: &[i32]) -> i32 {
    let mut left: Vec<i32> = left.to_vec();
    let mut right: Vec<i32> = right.to_vec();

    left.sort();
    right.sort();

    zip(left, right).map(|(a, b)| (a - b).abs()).sum()
}

fn similarity(left: &[i32], right: &[i32]) -> i32 {
    let mut right_counts: HashMap<i32, i32> = HashMap::new();

    for id in right {
        *right_counts.entry(*id).or_default() += 1;
    }

    left.iter()
        .filter(|n| right_counts.contains_key(n))
        .map(|n| n * right_counts.get(n).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_sample_input() {
        let path = Path::new("sample.txt");

        let (left, right) = read_input(path).unwrap();

        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn distance_sample() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let distance = distance(&left, &right);

        assert_eq!(distance, 11);
    }

    #[test]
    fn similarity_sample() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let similarity = similarity(&left, &right);

        assert_eq!(similarity, 31);
    }
}
