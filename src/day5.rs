extern crate regex;

use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn load_input(input: &str) -> String {
    String::from(input)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut last_str: Vec<char> = input.chars().collect();
    let mut last_len = last_str.len();
    loop {
        let mut i = 1;
        while i < last_str.len() {
            if last_str[i].is_uppercase()
                && last_str[i - 1].is_lowercase()
                && last_str[i - 1] == last_str[i].to_lowercase().next().unwrap()
            {
                last_str.remove(i);
                last_str.remove(i - 1);
            } else if last_str[i].is_lowercase()
                && last_str[i - 1].is_uppercase()
                && last_str[i - 1] == last_str[i].to_uppercase().next().unwrap()
            {
                last_str.remove(i);
                last_str.remove(i - 1);
            }
            i += 1;
        }
        if last_str.len() == last_len {
            return last_str.len();
        }
        last_len = last_str.len();
    }
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut best_size = input.len();
    let vecchar: Vec<char> = input.chars().collect();

    let mut set = HashSet::new();
    for c in input.chars() {
        set.insert(c.to_lowercase().next().unwrap());
    }

    for c in set {
        let mut new_poly = String::new();
        for n in vecchar.clone() {
            if n.to_lowercase().next().unwrap() != c {
                new_poly.push(n);
            }
        }

        let size = part1(&new_poly);
        if size < best_size {
            best_size = size;
        }
    }
    best_size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1examples() {
        assert_eq!(part1("aA"), 0);
        assert_eq!(part1("abBA"), 0);
        assert_eq!(part1("abAB"), 4);
        assert_eq!(part1("aabAAB"), 6);
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part2examples() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), 4);
    }
}
