use advent_of_code::fmt_dur;
use std::env;
use std::fs;
use std::time::Instant;

const TREE_SYMBOL: char = '#';

fn traverse(slope: Vec<&str>, start_pos: (usize, usize), route: (usize, usize)) -> i32 {
    let mut tree_counter = 0;
    let (right, down) = route;
    let (mut line, mut tile) = start_pos;
    let slope_width = slope[line].chars().count();
    while line < slope.len() - 1 {
        line += down;
        tile += right;
        if tile >= slope_width {
            tile -= slope_width;
        }

        let cur_symbol = slope[line].chars().nth(tile).unwrap();
        if cur_symbol == TREE_SYMBOL {
            tree_counter += 1;
        }
    }

    tree_counter
}

fn main() {
    // // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs/day03.txt");
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");
    let lines: Vec<&str> = input.lines().collect();

    println!("Running Part 1");
    let part1_start = Instant::now();
    let start_pos = (0, 0);
    let route = (3, 1);

    let part1 = traverse(lines.clone(), start_pos, route);
    let part1_dur = part1_start.elapsed();
    println!("Number of trees: {}", part1);
    println!("Took {}", fmt_dur(part1_dur));

    println!("Running Part 2");
    let part2_start = Instant::now();
    let start_pos = (0, 0);
    let routes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut results = Vec::new();

    for route in routes.iter() {
        let res = traverse(lines.clone(), start_pos, *route);
        results.push(res);
    }

    let part2_dur = part2_start.elapsed();
    println!(
        "Number of trees: {}",
        results.iter().map(|x| *x as u32).product::<u32>()
    );
    println!("Took {}", fmt_dur(part2_dur));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3right_1down() {
        let slope = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        let start_pos = (0, 0);
        let route = (3, 1);

        let res = traverse(slope.clone(), start_pos, route);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_1right_1down() {
        let slope = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        let start_pos = (0, 0);
        let route = (1, 1);

        let res = traverse(slope.clone(), start_pos, route);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_5right_1down() {
        let slope = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        let start_pos = (0, 0);
        let route = (5, 1);

        let res = traverse(slope.clone(), start_pos, route);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_7right_1down() {
        let slope = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        let start_pos = (0, 0);
        let route = (7, 1);

        let res = traverse(slope.clone(), start_pos, route);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_1right_2down() {
        let slope = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        let start_pos = (0, 0);
        let route = (1, 2);

        let res = traverse(slope.clone(), start_pos, route);
        assert_eq!(res, 2);
    }
}
