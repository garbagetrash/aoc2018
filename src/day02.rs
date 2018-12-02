pub mod day02 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    pub fn load_input() -> Vec<String> {
        let f = File::open("inputs/02.txt").unwrap();
        let f = BufReader::new(f);

        //f.lines()
        //    .map(|x| x.unwrap().parse::<i64>().unwrap())
        //    .collect()
        let mut input = Vec::new();
        for line in f.lines() {
            let x = line.unwrap();
            input.push(x);
        }
        input
    }

    pub fn counter(input: &str) -> (u32, u32) {
        let mut count2 = 0;
        let mut count3 = 0;
        let mut lettermap = HashMap::new();
        for c in input.chars() {
            if lettermap.contains_key(&c) {
                if let Some(x) = lettermap.get_mut(&c) {
                    *x += 1;
                }
            } else {
                lettermap.insert(c, 1);
            }
        }
        let mut count2_inc = false;
        let mut count3_inc = false;
        for (key, val) in lettermap.iter() {
            if *val == 2 {
                count2_inc = true;
            } else if *val == 3 {
                count3_inc = true;
            }
        }
        if count2_inc {
            count2 += 1;
        }
        if count3_inc {
            count3 += 1;
        }

        (count2, count3)
    }

    pub fn part1(input: &Vec<String>) -> u32 {
        let mut count2sum = 0;
        let mut count3sum = 0;
        for line in input {
            let (count2, count3) = counter(&line);
            count2sum += count2;
            count3sum += count3;
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
        println!("Should never get here");
        String::new()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            assert_eq!(counter("abcdef"), (0, 0));
            assert_eq!(counter("bababc"), (1, 1));
            assert_eq!(counter("abbcde"), (1, 0));
            assert_eq!(counter("abcccd"), (0, 1));
            assert_eq!(counter("aabcdd"), (1, 0));
            assert_eq!(counter("abcdee"), (1, 0));
            assert_eq!(counter("ababab"), (0, 1));
        }

        #[test]
        fn part2examples() {
            let mut test = Vec::new();
            test.push(String::from("abcde"));
            test.push(String::from("fghij"));
            test.push(String::from("klmno"));
            test.push(String::from("pqrst"));
            test.push(String::from("fguij"));
            test.push(String::from("axcye"));
            test.push(String::from("wvxyz"));
            assert_eq!(part2(&test), String::from("fgij"));
        }
    }
}
