pub mod day07 {
    extern crate regex;

    use regex::Regex;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Read;

    pub struct Step {
        pub name: char,
        pub children: HashSet<char>,
        pub parents: HashSet<char>,
    }

    impl Step {
        pub fn new(name: char) -> Step {
            Step {
                name: name,
                children: HashSet::new(),
                parents: HashSet::new(),
            }
        }
    }

    pub fn load_input(filename: &str) -> String {
        let mut buffer = String::new();
        File::open(filename)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        buffer.trim().to_string()
    }

    pub fn parse_input(input: &str) -> HashMap<char, Step> {
        let mut graph: HashMap<char, Step> = HashMap::new();
        let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
        for cap in re.captures_iter(input) {
            let c1 = cap[1].chars().next().unwrap();
            let c2 = cap[2].chars().next().unwrap();
            if !graph.contains_key(&c1) {
                graph.insert(c1, Step::new(c1));
            }
            if !graph.contains_key(&c2) {
                graph.insert(c2, Step::new(c2));
            }
            graph.get_mut(&c1).unwrap().children.insert(c2);
            graph.get_mut(&c2).unwrap().parents.insert(c1);
        }
        graph
    }

    pub fn remove_step(map: &mut HashMap<char, Step>, target: char) {
        let children = map.get(&target).unwrap().children.clone();
        for c in children.iter() {
            map.get_mut(c).unwrap().parents.remove(&target);
        }
        map.remove(&target);
    }

    pub fn part1(input: &str) -> String {
        let mut map = parse_input(input);
        let mut output = String::new();

        while map.len() > 0 {
            let mut options: Vec<char> = Vec::new();
            for (k, v) in map.iter() {
                if v.parents.len() == 0 {
                    options.push(*k);
                }
            }
            options.sort();

            let target = options.remove(0);
            remove_step(&mut map, target);
            output.push(target);
        }

        output
    }

    pub fn part2(input: &str, n_workers: u32, sec_offset: u32) -> u32 {
        let mut map = parse_input(input);
        let mut output = String::new();

        let mut options: Vec<(char, u32)> = Vec::new();
        let mut time = 0;
        while map.len() > 0 {
            // TODO: Don't want to rebuild options every second...
            for (k, v) in map.iter() {
                if v.parents.len() == 0 {
                    options.push((*k, sec_offset + (*k as u32) - 64));
                }
            }
            ////////////////////////////////////////////////////
            options.sort();
            for o in &options {
                println!("{:?}", *o);
            }
            if options.len() > n_workers as usize {
                options = options[..n_workers as usize].to_vec();
            }
            println!("Set ({}):", time);
            for o in &options {
                println!("{:?}", *o);
            }

            for idx in 0..options.len() {
                options[idx].1 -= 1;
                if options[idx].1 == 0 {
                    let target = options.remove(idx);
                    remove_step(&mut map, target.0);
                }
            }
            time += 1;
        }
        time
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            let input = String::from(
               "Step C must be finished before step A can begin.
                Step C must be finished before step F can begin.
                Step A must be finished before step B can begin.
                Step A must be finished before step D can begin.
                Step B must be finished before step E can begin.
                Step D must be finished before step E can begin.
                Step F must be finished before step E can begin.");
            assert_eq!(part1(&input), "CABDFE");
        }

        #[test]
        fn part2examples() {
            let input = String::from(
               "Step C must be finished before step A can begin.
                Step C must be finished before step F can begin.
                Step A must be finished before step B can begin.
                Step A must be finished before step D can begin.
                Step B must be finished before step E can begin.
                Step D must be finished before step E can begin.
                Step F must be finished before step E can begin.");
            assert_eq!(part2(&input, 2, 0), 15);
        }
    }
}
