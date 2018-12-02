pub mod day01 {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    pub fn load_input() -> Vec<i64> {
        let f = BufReader::new(File::open("inputs/01.txt").unwrap());
        f.lines()
            .map(|x| x.unwrap().parse::<i64>().unwrap())
            .collect()
    }

    pub fn part1(input: &Vec<i64>) -> i64 {
        input.iter().fold(0, |acc, x| acc + x)
    }

    pub fn part2(input: &Vec<i64>) -> i64 {
        let mut sum = 0 as i64;
        let mut set = HashSet::new();

        set.insert(sum);
        loop {
            for num in input {
                sum += num;
                if !set.insert(sum) {
                    return sum;
                };
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            assert_eq!(part1(&vec![1, -2, 3, 1]), 3);
            assert_eq!(part1(&vec![1, 1, 1]), 3);
            assert_eq!(part1(&vec![1, 1, -2]), 0);
            assert_eq!(part1(&vec![-1, -2, -3]), -6);
            assert_eq!(part1(&load_input()), 556);
        }

        #[test]
        fn part2examples() {
            assert_eq!(part2(&vec![1, -2, 3, 1]), 2);
            assert_eq!(part2(&vec![1, -1]), 0);
            assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
            assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
            assert_eq!(part2(&vec![7, 7, -2, -7, -4]), 14);
            assert_eq!(part2(&load_input()), 448);
        }
    }
}
