pub mod day06 {
    extern crate regex;

    use std::collections::HashSet;
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
    }

    pub fn part2(input: &str) -> usize {
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            let mut input = String::new();
            assert_eq!(0, 0);
        }

        #[test]
        fn test_part1() {
            assert_eq!(0, 0);
        }

        #[test]
        fn part2examples() {
            assert_eq!(0, 0);
        }

        #[test]
        fn test_part2() {
            assert_eq!(0, 0);
        }
    }
}
