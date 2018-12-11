pub mod day09 {
    extern crate regex;

    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;

    pub fn load_input(filename: &str) -> String {
        let mut buffer = String::new();
        File::open(filename)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        buffer.trim().to_string()
    }

    pub fn part1(input: &str) -> usize {

        let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
        let mut n_players = 0;
        let mut n_marbles = 0;
        for cap in re.captures_iter(input) {
            n_players = cap[1].parse::<usize>().unwrap();
            n_marbles = cap[2].parse::<usize>().unwrap();
        }
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

    pub fn part2(input: &str) -> usize {
        let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
        let mut n_players = 0;
        let mut n_marbles = 0;
        for cap in re.captures_iter(input) {
            n_players = cap[1].parse::<usize>().unwrap();
            n_marbles = 100 * cap[2].parse::<usize>().unwrap();
        }

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
        fn part1examples1() {
            let input = String::from("10 players; last marble is worth 1618 points");
            assert_eq!(part1(&input), 8317);
        }

        #[test]
        fn part1examples2() {
            let input = String::from("13 players; last marble is worth 7999 points");
            assert_eq!(part1(&input), 146373);
        }

        #[test]
        fn part1examples4() {
            let input = String::from("21 players; last marble is worth 6111 points");
            assert_eq!(part1(&input), 54718);
        }

        #[test]
        fn part1examples5() {
            let input = String::from("30 players; last marble is worth 5807 points");
            assert_eq!(part1(&input), 37305);
        }
    }
}
