use advent_of_code::fmt_dur;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Passport {
    byr: u16,
    eyr: u16,
    iyr: u16,
    cid: Option<u8>,
    pid: String,
    ecl: String,
    hcl: String,
    hgt: String,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.has_valid_ecl()
            && self.has_valid_hcl()
            && self.has_valid_pid()
            && self.has_valid_byr()
            && self.has_valid_eyr()
            && self.has_valid_iyr()
            && self.has_valid_hgt()
    }

    fn has_valid_hgt(&self) -> bool {
        const MIN_CM: u8 = 150;
        const MAX_CM: u8 = 193;
        const MIN_IN: u8 = 59;
        const MAX_IN: u8 = 76;

        let hgt_re = Regex::new(r"(?P<measure>\d{2,3})(?P<unit>in|cm)").unwrap();
        match hgt_re.captures(&self.hgt) {
            Some(caps) => {
                let measure = caps["measure"].parse::<u8>().unwrap();
                let unit = &caps["unit"];

                match unit {
                    "cm" => measure >= MIN_CM && measure <= MAX_CM,
                    "in" => measure >= MIN_IN && measure <= MAX_IN,
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn has_valid_ecl(&self) -> bool {
        let allowed_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        allowed_ecl.iter().any(|color| self.ecl == *color)
    }

    fn has_valid_hcl(&self) -> bool {
        let hcl_re: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
        hcl_re.is_match(&self.hcl)
    }

    fn has_valid_pid(&self) -> bool {
        let pid_re: Regex = Regex::new(r"\d{9}").unwrap();
        pid_re.is_match(&self.pid)
    }

    fn has_valid_byr(&self) -> bool {
        const MIN_BYR: u16 = 1920;
        const MAX_BYR: u16 = 2002;
        self.byr >= MIN_BYR && self.byr <= MAX_BYR
    }

    fn has_valid_eyr(&self) -> bool {
        const MIN_EYR: u16 = 2020;
        const MAX_EYR: u16 = 2030;
        self.eyr >= MIN_EYR && self.eyr <= MAX_EYR
    }

    fn has_valid_iyr(&self) -> bool {
        const MIN_IYR: u16 = 2010;
        const MAX_IYR: u16 = 2020;
        self.iyr >= MIN_IYR && self.iyr <= MAX_IYR
    }
}

#[derive(Debug, PartialEq)]
enum PassportFromStrError {
    Int(ParseIntError),
    MissingKey { missing_key: String },
}

impl From<ParseIntError> for PassportFromStrError {
    fn from(cause: ParseIntError) -> PassportFromStrError {
        PassportFromStrError::Int(cause)
    }
}

impl FromStr for Passport {
    type Err = PassportFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport: HashMap<&str, Option<&str>> = HashMap::new();
        let mandatory_keys = ["byr", "eyr", "iyr", "pid", "ecl", "hcl", "hgt"];

        for attr in s.split_whitespace() {
            let spl_str = attr.split(':').collect::<Vec<&str>>();
            passport.insert(spl_str[0], Some(spl_str[1]));
        }

        if let Some(k) = mandatory_keys.iter().find(|k| !passport.contains_key(*k)) {
            return Err(PassportFromStrError::MissingKey {
                missing_key: k.to_string(),
            });
        }

        let mut cid_fromstr = None;
        if let Some(cid) = passport.get("cid") {
            cid_fromstr = cid.unwrap().parse::<u8>().ok();
        }
        let byr_fromstr = passport["byr"].unwrap().parse::<u16>()?;
        let eyr_fromstr = passport["eyr"].unwrap().parse::<u16>()?;
        let iyr_fromstr = passport["iyr"].unwrap().parse::<u16>()?;
        let pid_fromstr = passport["pid"].unwrap();
        let ecl_fromstr = passport["ecl"].unwrap();
        let hcl_fromstr = passport["hcl"].unwrap();
        let hgt_fromstr = passport["hgt"].unwrap();

        Ok(Passport {
            byr: byr_fromstr,
            eyr: eyr_fromstr,
            iyr: iyr_fromstr,
            cid: cid_fromstr,
            pid: pid_fromstr.to_string(),
            ecl: ecl_fromstr.to_string(),
            hcl: hcl_fromstr.to_string(),
            hgt: hgt_fromstr.to_string(),
        })
    }
}

fn main() {
    // // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs/day04.txt");
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");

    println!("Running Part 1");
    let part1_start = Instant::now();

    let passports: Vec<Passport> = input
        .split("\n\n")
        .filter_map(|p| Passport::from_str(p).ok())
        .collect();

    let part1_dur = part1_start.elapsed();
    println!("Valid passports: {}", passports.len());
    println!("Took {}", fmt_dur(part1_dur));

    println!("Running Part 2");
    let part2_start = Instant::now();

    let validated_passports: Vec<Passport> =
        passports.into_iter().filter(|p| p.is_valid()).collect();

    let part2_dur = part2_start.elapsed();
    println!("Valid passports: {}", validated_passports.len());
    println!("Took {}", fmt_dur(part2_dur));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str1() {
        let string = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let expected = Passport {
            ecl: "gry".to_string(),
            pid: "860033327".to_string(),
            eyr: 2020,
            hcl: "#fffffd".to_string(),
            byr: 1937,
            iyr: 2017,
            cid: Some(147),
            hgt: "183cm".to_string(),
        };
        let result = Passport::from_str(string);

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_from_without_hgt() {
        let string = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929";
        let result = Passport::from_str(string);
        let expected = Err(PassportFromStrError::MissingKey {
            missing_key: "hgt".to_string(),
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_str_without_cid() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm";
        let expected = Passport {
            ecl: "brn".to_string(),
            pid: "760753108".to_string(),
            eyr: 2024,
            hcl: "#ae17e1".to_string(),
            byr: 1931,
            iyr: 2013,
            hgt: "179cm".to_string(),
            cid: None,
        };
        let result = Passport::from_str(string);

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_from_str_without_cid_and_byr() {
        let string = "hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in";
        let result = Passport::from_str(string);
        let expected = Err(PassportFromStrError::MissingKey {
            missing_key: "byr".to_string(),
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_valid_ecl() {
        let string = "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_ecl() {
        let string = "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:red pid:760753108 byr:1931
        hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_valid_hcl() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_hcl() {
        let string = "hcl:#ze17e1 iyr:2013
    eyr:2024
    ecl:gry pid:760753108 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }
    #[test]
    fn test_hcl_without_hash() {
        let string = "hcl:ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:760753108 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_valid_pid() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_valid_pid_leading_zero() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:003456789 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_pid() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:red pid:12345678 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_valid_byr() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_byr_low() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1919
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invalid_byr_high() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:2003
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_valid_eyr() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_eyr_low() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2014
    ecl:gry pid:123456789 byr:1920
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invalid_eyr_high() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2034
    ecl:gry pid:123456789 byr:1920
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_valid_iyr() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_iyr_low() {
        let string = "hcl:#ae17e1 iyr:2009
    eyr:2024
    ecl:gry pid:123456789 byr:1920
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invalid_iyr_high() {
        let string = "hcl:#ae17e1 iyr:2023
    eyr:2024
    ecl:gry pid:123456789 byr:1920
    hgt:179cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_valid_hgt_in() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:60in";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_hgt_in() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:190in";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_valid_hgt_cm() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:190cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), true);
    }

    #[test]
    fn test_invalid_hgt_cm() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:200cm";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invalid_hgt_no_unit() {
        let string = "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:gry pid:123456789 byr:1931
    hgt:190";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invlid_passport1() {
        let string = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invlid_passport2() {
        let string = "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invlid_passport3() {
        let string = "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }

    #[test]
    fn test_invlid_passport4() {
        let string = "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let result = Passport::from_str(string).unwrap();

        assert_eq!(result.is_valid(), false);
    }
}
