use std::collections::HashMap;

fn find_pair(input: Vec<i32>, n: usize, k: &i32) -> (i32, i32) {
    let mut map: HashMap<i32, bool> = HashMap::with_capacity(n);

    for i in input.iter() {
        let x = k - i;
        if map.contains_key(&x) {
            return (x, *i);
        } else {
            map.insert(*i, true);
        }
    }
    return (0, 0);
}

fn find_triplet(input: Vec<i32>, k: &i32) -> (i32, i32, i32) {
    for (pos, a) in input.iter().enumerate() {
        let new_k = k - a;
        let mut new_vec = vec![];
        new_vec.extend_from_slice(&input[pos + 1..]);
        let n = new_vec.len();
        let (b, c) = find_pair(new_vec, n, &new_k);
        if b != 0 && c != 0 {
            return (*a, b, c);
        }
    }

    return (0, 0, 0);
}

pub fn part1(input: String) {
    let lines: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let k = 2020;
    let len = lines.len();

    let (a, b) = find_pair(lines, len, &k);

    println!("{} * {} = {}", a, b, a * b);
}

pub fn part2(input: String) {
    let lines: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let k = 2020;

    let (a, b, c) = find_triplet(lines, &k);

    println!("{} * {} * {} = {}", a, b, c, a * b * c);
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
        let (a, b) = find_pair(input, len, &k);
        assert_eq!(a * b, 514579);
    }

    #[test]
    fn test_part2() {
        let k = 2020;
        let mut input = Vec::new();
        input.extend([1721, 979, 366, 299, 675, 1456].iter());
        let (a, b, c) = find_triplet(input, &k);
        assert_eq!(a * b * c, 241861950);
    }
}
