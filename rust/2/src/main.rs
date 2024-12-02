use regex::Regex;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

fn main() {
    let args: Vec<String> = args().collect();
    let path: &Path = Path::new(args.get(1).expect("Please specify the input path"));

    let reports = read_input(&path).unwrap();

    println!(
        "Safe reports: {}",
        reports.iter().filter(|r| is_safe(r)).count()
    );
    println!(
        "Safe reports with damper: {}",
        reports.iter().filter(|r| is_safe_ignoring_one(r)).count()
    );
}

fn read_input(path: &Path) -> Result<Vec<Vec<i32>>, Error> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let whitespace = Regex::new(r"\s+").unwrap();

    for line in reader.lines().flatten() {
        let x: Vec<i32> = whitespace
            .split(&line)
            .map(|s| s.parse().unwrap())
            .collect();
        reports.push(x);
    }

    Ok(reports)
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let direction = report[0] < report[1];
    let mut prev = report[0];

    for level in report[1..].iter() {
        let level = *level;
        if direction != (prev < level) {
            return false;
        }

        if prev == level || (prev - level).abs() > 3 {
            return false;
        }

        prev = level;
    }

    return true;
}

fn is_safe_ignoring_one(report: &Vec<i32>) -> bool {
    (0..report.len()).any(|i| is_safe_ignoring(report, i))
}

fn is_safe_ignoring(report: &Vec<i32>, ignore: usize) -> bool {
    let mut direction: Option<bool> = None;
    let mut prev: Option<&i32> = None;

    for (i, level) in report.iter().enumerate() {
        if i == ignore {
            continue;
        }

        if let Some(prev_level) = prev {
            if direction.is_none() {
                direction = Some(prev_level < level);
            }

            let dir = direction.unwrap();
            if dir != (prev_level < level) {
                return false;
            }

            if prev_level == level || (prev_level - level).abs() > 3 {
                return false;
            }
        }

        prev = Some(level);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_safe_sample() {
        let report = vec![7, 6, 4, 2, 1];

        let is_safe = is_safe(&report);

        assert!(is_safe);
    }

    #[test]
    fn is_unsafe_direction() {
        assert!(!is_safe(&vec![1, 2, 3, 2]))
    }

    #[test]
    fn is_unsafe_jump() {
        assert!(!is_safe(&vec![1, 2, 3, 7]))
    }
}
