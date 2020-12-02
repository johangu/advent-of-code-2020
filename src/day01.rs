use advent_of_code::fmt_dur;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn find_pair(input: Vec<i32>, n: usize, k: &i32) -> Option<(i32, i32)> {
    let mut map: HashMap<i32, bool> = HashMap::with_capacity(n);

    for i in input.iter() {
        let x = k - i;
        if map.contains_key(&x) {
            return Some((x, *i));
        } else {
            map.insert(*i, true);
        }
    }

    None
}

fn find_triplet(input: Vec<i32>, k: &i32) -> Option<(i32, i32, i32)> {
    for (pos, a) in input.iter().enumerate() {
        let new_k = k - a;
        let mut new_vec = vec![];
        new_vec.extend_from_slice(&input[pos + 1..]);
        let n = new_vec.len();
        if let Some((b, c)) = find_pair(new_vec, n, &new_k) {
            return Some((*a, b, c));
        }
    }

    None
}

fn part1(lines: Vec<i32>, n: usize, k: i32) {
    match find_pair(lines, n, &k) {
        Some((a, b)) => println!("{} * {} = {}", a, b, a * b),
        None => println!("Did not find a matching pair"),
    }
}

fn part2(lines: Vec<i32>, k: i32) {
    match find_triplet(lines, &k) {
        Some((a, b, c)) => println!("{} * {} * {} = {}", a, b, c, a * b * c),
        None => println!("Did not find a matching triplet"),
    }
}

fn main() {
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs/day01.txt");
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");
    let lines: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    println!("Running Part 1");
    let part1_start = Instant::now();
    part1(lines.clone(), lines.len(), 2020);
    let part1_dur = part1_start.elapsed();
    println!("Took {}", fmt_dur(part1_dur));

    println!("Running Part 2");
    let part2_start = Instant::now();
    part2(lines.clone(), 2020);
    let part2_dur = part2_start.elapsed();
    println!("Took {}", fmt_dur(part2_dur));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let k = 2020;
        let mut input = Vec::new();
        input.extend([1721, 979, 366, 299, 675, 1456].iter());
        let len = input.len();
        if let Some((a, b)) = find_pair(input, len, &k) {
            assert_eq!(a * b, 514579);
        }
    }

    #[test]
    fn test_part2() {
        let k = 2020;
        let mut input = Vec::new();
        input.extend([1721, 979, 366, 299, 675, 1456].iter());
        if let Some((a, b, c)) = find_triplet(input, &k) {
            assert_eq!(a * b * c, 241861950);
        }
    }
}
