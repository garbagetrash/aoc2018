extern crate regex;

use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn load_input(input: &str) -> Vec<char> {
    String::from(input).chars().collect()
}

fn iterate(seq: &[char]) -> Vec<char> {
    let start_len = seq.len();
    //println!("{}", start_len);
    if start_len < 2 {
        return seq.to_vec();
    }
    let siter = seq.chunks_exact(2);
    let end = siter.remainder();

    let mut next_seq = siter
        .filter(|letters| {
            if letters[0] == letters[1].to_lowercase().next().unwrap() && letters[1].is_uppercase()
            {
                false
            } else {
                !(letters[0].to_lowercase().next().unwrap() == letters[1]
                    && letters[0].is_uppercase())
            }
        })
        .collect::<Vec<_>>()
        .concat();
    if !end.is_empty() {
        next_seq.push(end[0]);
    }
    next_seq
}

#[aoc(day5, part1)]
pub fn part1(input: &[char]) -> usize {
    let mut seq = input.to_vec();

    loop {
        let start_len = seq.len();
        seq = iterate(&seq);
        if seq.len() < 2 {
            return seq.len();
        }
        let mut start = vec![seq[0]];
        seq = iterate(&seq[1..]);

        // This is still slow, is there a better way?
        start.append(&mut seq);
        seq = start;

        // Bail when done
        if seq.len() == start_len {
            break;
        }
    }

    seq.len()
}

#[aoc(day5, part2)]
pub fn part2(input: &[char]) -> usize {
    let mut best_size = input.len();
    let vecchar: Vec<char> = input.to_vec();

    let mut set = HashSet::new();
    for c in input {
        set.insert(c.to_lowercase().next().unwrap());
    }

    for c in set {
        let mut new_poly = vec![];
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
    fn test_part1() {
        assert_eq!(part1(&load_input("aA")), 0);
        assert_eq!(part1(&load_input("abBA")), 0);
        assert_eq!(part1(&load_input("abAB")), 4);
        assert_eq!(part1(&load_input("aabAAB")), 6);
        assert_eq!(part1(&load_input("dabAcCaCBAcCcaDA")), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&load_input("dabAcCaCBAcCcaDA")), 4);
    }
}
