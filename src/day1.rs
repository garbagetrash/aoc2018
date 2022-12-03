use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn load_input(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i64>) -> i64 {
    input.iter().fold(0, |acc, x| acc + x)
}

#[aoc(day1, part2)]
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
    fn test_part1() {
        assert_eq!(part1(&vec![1, -2, 3, 1]), 3);
        assert_eq!(part1(&vec![1, 1, 1]), 3);
        assert_eq!(part1(&vec![1, 1, -2]), 0);
        assert_eq!(part1(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![1, -2, 3, 1]), 2);
        assert_eq!(part2(&vec![1, -1]), 0);
        assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2(&vec![7, 7, -2, -7, -4]), 14);
    }
}
