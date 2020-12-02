use lazy_static::lazy_static;
use regex::Regex;

fn is_valid_password1(min: usize, max: usize, character: char, pwd: &str) -> bool {
    let n = pwd.matches(character).count();
    return n >= min && n <= max;
}

fn is_valid_password2(pos1: usize, pos2: usize, character: char, pwd: &str) -> bool {
    let char1: char = pwd.chars().nth(pos1 - 1).unwrap();
    let char2: char = pwd.chars().nth(pos2 - 1).unwrap();

    return (char1 == character) ^ (char2 == character);
}

pub fn part1(input: String) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<character>[A-Za-z]): (?P<pwd>.*)").unwrap();
    }

    let mut valid_passwords = Vec::new();
    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let min = caps["min"].parse::<usize>().unwrap();
        let max = caps["max"].parse::<usize>().unwrap();
        let character = caps["character"].parse::<char>().unwrap();
        let pwd = caps["pwd"].parse::<String>().unwrap();

        if is_valid_password1(min, max, character, &pwd) {
            valid_passwords.push(pwd);
        }
    }

    println!("Number of valid passwords: {}", valid_passwords.len());
}

pub fn part2(input: String) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<pos1>\d+)-(?P<pos2>\d+) (?P<character>[A-Za-z]): (?P<pwd>.*)")
                .unwrap();
    }

    let mut valid_passwords = Vec::new();
    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let min = caps["pos1"].parse::<usize>().unwrap();
        let max = caps["pos2"].parse::<usize>().unwrap();
        let character = caps["character"].parse::<char>().unwrap();
        let pwd = caps["pwd"].parse::<String>().unwrap();

        if is_valid_password2(min, max, character, &pwd) {
            valid_passwords.push(pwd);
        }
    }

    println!("Number of valid passwords: {}", valid_passwords.len());
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1_pwd1() {
        let min = 1;
        let max = 3;
        let character: char = 'a';
        let pwd = "abcde";

        assert_eq!(is_valid_password1(min, max, character, pwd), true);
    }

    #[test]
    fn test_part1_pwd2() {
        let min = 1;
        let max = 3;
        let character: char = 'b';
        let pwd = "cdefg";

        assert_eq!(is_valid_password1(min, max, character, pwd), false);
    }

    #[test]
    fn test_part1_pwd3() {
        let min = 2;
        let max = 9;
        let character: char = 'c';
        let pwd = "ccccccccc";

        assert_eq!(is_valid_password1(min, max, character, pwd), true);
    }

    #[test]
    fn test_part2_pwd1() {
        let min = 1;
        let max = 3;
        let character: char = 'a';
        let pwd = "abcde";

        assert_eq!(is_valid_password2(min, max, character, pwd), true);
    }

    #[test]
    fn test_part2_pwd2() {
        let min = 1;
        let max = 3;
        let character: char = 'b';
        let pwd = "cdefg";

        assert_eq!(is_valid_password2(min, max, character, pwd), false);
    }

    #[test]
    fn test_part2_pwd3() {
        let min = 2;
        let max = 9;
        let character: char = 'c';
        let pwd = "ccccccccc";

        assert_eq!(is_valid_password2(min, max, character, pwd), false);
    }
}
