pub mod day12 {
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

    #[derive(Debug)]
    pub struct State {
        pub pots: Vec<u8>,
        pub start_idx: i32,
    }

    impl State {
        pub fn new(pots: Vec<u8>, start_idx: i32) -> State {
            State {
                pots: pots,
                start_idx: start_idx,
            }
        }

        pub fn next_iter(&mut self, rule_set: &RuleSet) {
            // Expand limits 4 more pots
            &self.pots.insert(0, 0);
            &self.pots.insert(0, 0);
            &self.pots.insert(0, 0);
            &self.pots.insert(0, 0);
            &self.pots.push(0);
            &self.pots.push(0);
            &self.pots.push(0);
            &self.pots.push(0);
            let mut new_pots: Vec<u8> = Vec::new();

            // Apply rules to build new pots vec
            for (idx, pot) in self.pots.iter().enumerate() {
                if idx > 1 && idx < self.pots.len() - 2 {
                    let mut key = [0u8; 5];
                    key.copy_from_slice(&self.pots[idx - 2..idx + 3]);
                    if let Some(new_pot) = rule_set.rules.get(&key) {
                        new_pots.push(*new_pot);
                    } else {
                        new_pots.push(0);
                    }
                }
            }

            // Clean up unnecessary limit pots
            let mut cnt = 0;
            loop {
                let mut key = [0u8; 5];
                key.copy_from_slice(&new_pots[..5]);
                if key == [0u8; 5] {
                    new_pots.remove(0);
                    cnt += 1;
                } else {
                    break;
                }
            }

            // Set self.pots to new pots vec
            self.pots = new_pots;

            // Adjust start_idx as necessary
            self.start_idx -= 2 - cnt;
        }
    }

    pub struct RuleSet {
        pub rules: HashMap<[u8; 5], u8>,
    }

    impl RuleSet {
        pub fn new(rules: HashMap<[u8; 5], u8>) -> RuleSet {
            RuleSet { rules: rules }
        }
    }

    pub fn parse_input(input: &str) -> (State, RuleSet) {
        let re1 = Regex::new(r"initial state: ([\.\#]+)").unwrap();
        let re2 = Regex::new(r"([\.\#]{5}) => ([\.\#])").unwrap();
        let mut pots = Vec::new();
        for cap in re1.captures_iter(input) {
            for c in cap[1].chars() {
                match c {
                    '.' => pots.push(0),
                    '#' => pots.push(1),
                    _ => panic!("WTFBBQ1"),
                }
            }
        }
        let mut rules = RuleSet::new(HashMap::new());
        for cap in re2.captures_iter(input) {
            let mut pattern = Vec::new();
            for c in cap[1].chars() {
                match c {
                    '.' => pattern.push(0),
                    '#' => pattern.push(1),
                    _ => panic!("WTFBBQ2"),
                }
            }
            let mut output = 0;
            for c in cap[2].chars() {
                match c {
                    '.' => output = 0,
                    '#' => output = 1,
                    _ => panic!("WTFBBQ3"),
                }
            }
            let mut arr = [0; 5];
            arr.copy_from_slice(pattern.as_slice());
            rules.rules.insert(arr, output);
        }
        (State::new(pots, 0), rules)
    }

    pub fn solver(input: &str, n_iter: usize) -> i32 {
        let mut prev_res: HashMap<Vec<u8>, u8> = HashMap::new();
        let mut srtup = parse_input(input);
        for i in 0..n_iter {
            if let Some(val) = prev_res.insert(srtup.0.pots.clone(), 1) {
                println!("Cycle detected!: {}", i);
                break;
            }
            srtup.0.next_iter(&srtup.1);
        }

        let mut output = 0;
        for (idx, i) in
            (srtup.0.start_idx..srtup.0.start_idx + srtup.0.pots.len() as i32).enumerate()
        {
            if srtup.0.pots[idx] == 1 {
                output += i;
            }
        }
        output
    }

    pub fn part1(input: &str) -> i32 {
        solver(input, 20)
    }

    pub fn part2(input: &str) -> i32 {
        solver(input, 50000000000)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            let input = String::from(
                "initial state: #..#.#..##......###...###

                ...## => #
                ..#.. => #
                .#... => #
                .#.#. => #
                .#.## => #
                .##.. => #
                .#### => #
                #.#.# => #
                #.### => #
                ##.#. => #
                ##.## => #
                ###.. => #
                ###.# => #
                ####. => #",
            );
            assert_eq!(part1(&input), 325);
        }

        #[test]
        fn part2examples() {
            let input = String::from(
                "initial state: #..#.#..##......###...###

                ...## => #
                ..#.. => #
                .#... => #
                .#.#. => #
                .#.## => #
                .##.. => #
                .#### => #
                #.#.# => #
                #.### => #
                ##.#. => #
                ##.## => #
                ###.. => #
                ###.# => #
                ####. => #",
            );
            assert_eq!(part2(&input), 1);
        }
    }
}
