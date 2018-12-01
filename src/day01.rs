pub mod day01 {
    use std::collections::HashSet;

    pub fn part1(input: &Vec<i64>) -> i64 {
        let mut sum = 0 as i64;
        for num in input {
            sum += num;
        }
        sum
    }

    pub fn part2(input: &Vec<i64>) -> i64 {
        let mut sum = 0 as i64;
        let mut set = HashSet::new();

        set.insert(sum);
        loop {
            for num in input {
                sum += num;
                let not_done = set.insert(sum);
                if !not_done {
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
        }

        #[test]
        fn part2examples() {
            assert_eq!(part2(&vec![1, -2, 3, 1]), 2);
            assert_eq!(part2(&vec![1, -1]), 0);
            assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
            assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
            assert_eq!(part2(&vec![7, 7, -2, -7, -4]), 14);
        }
    }
}
