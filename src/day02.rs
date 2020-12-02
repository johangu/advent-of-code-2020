use lazy_static::lazy_static;
use regex::Regex;

fn is_valid_password(min: usize, max: usize, character: char, pwd: &str) -> bool {
    let n = pwd.matches(character).count();
    return n >= min && n <= max;
}

pub fn part1(input: String) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<character>[A-Za-z]): (?P<pwd>.*)").unwrap();
    }
    // let lines: Vec<(i32, i32, char, String)> =
    //     input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut valid_passwords = Vec::new();
    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let min = caps["min"].parse::<usize>().unwrap();
        let max = caps["max"].parse::<usize>().unwrap();
        let character = caps["character"].parse::<char>().unwrap();
        let pwd = caps["pwd"].parse::<String>().unwrap();

        if is_valid_password(min, max, character, &pwd) {
            valid_passwords.push(pwd);
        }
    }

    println!("Number of valid passwords: {}", valid_passwords.len());
}

pub fn part2(input: String) {}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_pwd1() {
        let min = 1;
        let max = 3;
        let character: char = 'a';
        let pwd = "abcde";

        assert_eq!(is_valid_password(min, max, character, pwd), true);
    }

    #[test]
    fn test_part2() {}
}
