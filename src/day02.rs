pub mod day02 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    pub fn load_input() -> Vec<String> {
        let f = BufReader::new(File::open("inputs/02.txt").unwrap());
        f.lines().map(|x| x.unwrap()).collect()
    }

    pub fn counter(input: &str) -> (bool, bool) {
        // HashMap with letter counts for input ID
        let mut lettermap = HashMap::new();
        for c in input.chars() {
            if let Some(x) = lettermap.get_mut(&c) {
                *x += 1;
            } else {
                lettermap.insert(c, 1);
            }
        }

        // A count for each occurrance of exactly 2 or 3 identical letters
        let count2 = lettermap.values().fold(false, |acc, x| acc || *x == 2);
        let count3 = lettermap.values().fold(false, |acc, x| acc || *x == 3);
        (count2, count3)
    }

    pub fn part1(input: &Vec<String>) -> u32 {
        let mut count2sum = 0;
        let mut count3sum = 0;
        for line in input {
            let (count2, count3) = counter(&line);
            if count2 {
                count2sum += 1;
            }
            if count3 {
                count3sum += 1;
            }
        }
        count2sum * count3sum
    }

    pub fn diff(id1: &str, id2: &str) -> u32 {
        let mut output = 0;
        for (c1, c2) in id1.chars().zip(id2.chars()) {
            if c1 != c2 {
                output += 1;
            }
        }
        output
    }

    pub fn part2(input: &Vec<String>) -> String {
        let mut output = String::new();
        for line1 in input {
            for line2 in input {
                if diff(&line1, &line2) == 1 {
                    // found our boxes
                    for (c1, c2) in line1.chars().zip(line2.chars()) {
                        if c1 == c2 {
                            output.push(c1);
                        }
                    }
                    return output;
                }
            }
        }
        panic!("Should never get here!");
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            assert_eq!(counter("abcdef"), (false, false));
            assert_eq!(counter("bababc"), (true, true));
            assert_eq!(counter("abbcde"), (true, false));
            assert_eq!(counter("abcccd"), (false, true));
            assert_eq!(counter("aabcdd"), (true, false));
            assert_eq!(counter("abcdee"), (true, false));
            assert_eq!(counter("ababab"), (false, true));
            assert_eq!(part1(&load_input()), 7872);
        }

        #[test]
        fn part2examples() {
            let mut test = Vec::new();
            let literals = [
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
            ];
            for lit in literals.iter() {
                test.push(String::from(*lit));
            }
            assert_eq!(part2(&test), "fgij");
            assert_eq!(part2(&load_input()), "tjxmoewpdkyaihvrndfluwbzc")
        }
    }
}
