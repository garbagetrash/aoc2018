pub mod day07 {
    extern crate regex;

    use regex::Regex;

    use std::cmp::Ordering;
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
        let re =
            Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
        for cap in re.captures_iter(input) {
            let c1 = cap[1].as_bytes()[0] as char;
            let c2 = cap[2].as_bytes()[0] as char;
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

    #[derive(PartialEq, Eq, Debug)]
    struct Job {
        id: char,
        time_remaining: u32,
        depends_on: HashSet<char>,
    }

    impl Job {
        pub fn new(id: char, time_remaining: u32, depends_on: HashSet<char>) -> Job {
            Job {
                id: id,
                time_remaining: time_remaining,
                depends_on: depends_on,
            }
        }

        pub fn ready(&self, done_vec: &HashSet<char>) -> bool {
            for dependency in &self.depends_on {
                if !done_vec.contains(&dependency) {
                    return false;
                }
            }
            true
        }
    }

    impl PartialOrd for Job {
        fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
            Some(self.id.cmp(&other.id))
        }
    }

    impl Ord for Job {
        fn cmp(&self, other: &Job) -> Ordering {
            self.id.cmp(&other.id)
        }
    }

    pub fn part2(input: &str, n_workers: usize, sec_offset: u32) -> u32 {
        let map = parse_input(input);
        let mut order = HashMap::new();
        for c in part1(input).chars() {
            let job = Job::new(
                c,
                sec_offset + (c as u32) - 64,
                map.get(&c).unwrap().parents.clone(),
            );
            order.insert(c, job);
        }

        let mut done = false;
        let mut active_jobs = HashMap::new();
        let mut complete_jobs = HashSet::new();
        let mut time = 0;
        while !done {
            // If work is available, distribute it
            let mut options: Vec<Job> = Vec::new();
            if active_jobs.len() < n_workers {
                for (id, v) in order.iter() {
                    if v.ready(&complete_jobs) {
                        options.push(Job::new(*id, v.time_remaining, v.depends_on.clone()));
                    }
                }
                if options.len() > 0 {
                    options.sort();
                    let slots = n_workers - active_jobs.len();
                    let work = options.len();
                    for _ in 0..std::cmp::min(slots, work) {
                        let c: &Job = &options.pop().unwrap();
                        active_jobs.insert(c.id, (c.time_remaining, c.depends_on.clone()));
                        order.remove(&c.id);
                    }
                }
            }

            // Simulate time step
            time += 1;
            for (id, job) in active_jobs.iter_mut() {
                job.0 -= 1;
                if job.0 == 0 {
                    complete_jobs.insert(*id);
                }
            }
            active_jobs.retain(|_, v| v.0 > 0);

            // Bail when done
            if active_jobs.is_empty() && order.is_empty() {
                done = true;
            }
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
                Step F must be finished before step E can begin.",
            );
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
                Step F must be finished before step E can begin.",
            );
            assert_eq!(part2(&input, 2, 0), 15);
        }
    }
}
