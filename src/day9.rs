use scan_fmt::scan_fmt;
use std::collections::HashMap;

#[aoc_generator(day9)]
fn load_input(input: &str) -> (usize, usize) {
    scan_fmt!(
        input,
        "{} players; last marble is worth {} points",
        usize,
        usize
    )
    .unwrap()
}

#[aoc(day9, part1)]
pub fn part1(input: &(usize, usize)) -> usize {
    let &(n_players, n_marbles) = input;
    let mut players: Vec<usize> = vec![0; n_players];
    let mut marbles: Vec<usize> = Vec::new();
    marbles.push(0);
    marbles.push(1);

    let mut idx = 1;
    for i in 2..n_marbles {
        if i % 23 == 0 {
            players[i % n_players] += i;
            if idx < 7 {
                idx += marbles.len();
            }
            idx = idx - 7;
            if idx == marbles.len() - 1 {
                println!("Are we ok here?");
            }
            players[i % n_players] += marbles.remove(idx);
        } else {
            if idx + 2 == marbles.len() {
                idx = marbles.len();
                marbles.push(i);
            } else {
                idx = (idx + 2) % marbles.len();
                marbles.insert(idx, i);
            }
        }
    }
    *players.iter().max().unwrap()
}

pub fn hash_insert(idx: usize, new_val: usize, map: &mut HashMap<usize, usize>) {
    if let Some(val) = map.insert(idx, new_val) {
        hash_insert(idx + 1, val, map);
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &(usize, usize)) -> usize {
    let &(n_players, mut n_marbles) = input;
    n_marbles *= 100;

    let mut players: Vec<usize> = vec![0; n_players];
    let mut marbles: HashMap<usize, usize> = HashMap::new();
    hash_insert(0, 0, &mut marbles);
    hash_insert(1, 1, &mut marbles);

    let mut idx = 1;
    for i in 2..n_marbles {
        if i % 23 == 0 {
            players[i % n_players] += i;
            if idx < 7 {
                idx += marbles.len();
            }
            idx = idx - 7;
            if idx == marbles.len() - 1 {
                println!("Are we ok here?");
            }
            players[i % n_players] += marbles.remove(&idx).unwrap();
        } else {
            if idx + 2 == marbles.len() {
                idx = marbles.len();
            } else {
                idx = (idx + 2) % marbles.len();
            }
            hash_insert(idx, i, &mut marbles);
        }
    }
    *players.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&load_input("10 players; last marble is worth 1618 points")),
            8317
        );
        assert_eq!(
            part1(&load_input("13 players; last marble is worth 7999 points")),
            146373
        );
        //assert_eq!(part1(&load_input("17 players; last marble is worth 1104 points")), 2764);
        assert_eq!(
            part1(&load_input("21 players; last marble is worth 6111 points")),
            54718
        );
        assert_eq!(
            part1(&load_input("30 players; last marble is worth 5807 points")),
            37305
        );
    }
}
