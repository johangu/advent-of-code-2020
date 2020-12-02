use advent_of_code::fmt_dur;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::time::Instant;

fn is_valid_password(min: usize, max: usize, c: char, pwd: &str) -> bool {
    let n = pwd.matches(c).count();
    n >= min && n <= max
}

fn is_valid_toboggan_password(pos1: usize, pos2: usize, c: char, pwd: &str) -> bool {
    let char1: char = pwd.chars().nth(pos1 - 1).unwrap();
    let char2: char = pwd.chars().nth(pos2 - 1).unwrap();

    (char1 == c) ^ (char2 == c)
}

fn main() {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<pos1>\d+)-(?P<pos2>\d+) (?P<c>[A-Za-z]): (?P<pwd>.*)").unwrap();
    }
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs/day02.txt");
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");

    let mut valid_passwords = Vec::new();
    let mut valid_toboggan_passwords = Vec::new();

    println!("Checking passwords");
    let start = Instant::now();

    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let min = caps["pos1"].parse::<usize>().unwrap();
        let max = caps["pos2"].parse::<usize>().unwrap();
        let c = caps["c"].parse::<char>().unwrap();
        let pwd = caps["pwd"].parse::<String>().unwrap();

        if is_valid_password(min, max, c, &pwd) {
            valid_passwords.push(pwd.clone());
        }

        if is_valid_toboggan_password(min, max, c, &pwd) {
            valid_toboggan_passwords.push(pwd.clone());
        }
    }
    let dur = start.elapsed();

    println!("Number of valid passwords: {}", valid_passwords.len());
    println!(
        "Number of valid toboggan passwords: {}",
        valid_toboggan_passwords.len()
    );
    println!("Took {}", fmt_dur(dur));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_pwd1() {
        let min = 1;
        let max = 3;
        let c: char = 'a';
        let pwd = "abcde";

        assert_eq!(is_valid_password(min, max, c, pwd), true);
    }

    #[test]
    fn test_part1_pwd2() {
        let min = 1;
        let max = 3;
        let c: char = 'b';
        let pwd = "cdefg";

        assert_eq!(is_valid_password(min, max, c, pwd), false);
    }

    #[test]
    fn test_part1_pwd3() {
        let min = 2;
        let max = 9;
        let c: char = 'c';
        let pwd = "ccccccccc";

        assert_eq!(is_valid_password(min, max, c, pwd), true);
    }

    #[test]
    fn test_part2_pwd1() {
        let min = 1;
        let max = 3;
        let c: char = 'a';
        let pwd = "abcde";

        assert_eq!(is_valid_toboggan_password(min, max, c, pwd), true);
    }

    #[test]
    fn test_part2_pwd2() {
        let min = 1;
        let max = 3;
        let c: char = 'b';
        let pwd = "cdefg";

        assert_eq!(is_valid_toboggan_password(min, max, c, pwd), false);
    }

    #[test]
    fn test_part2_pwd3() {
        let min = 2;
        let max = 9;
        let c: char = 'c';
        let pwd = "ccccccccc";

        assert_eq!(is_valid_toboggan_password(min, max, c, pwd), false);
    }
}
