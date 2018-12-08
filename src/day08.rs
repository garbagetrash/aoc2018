pub mod day08 {
    extern crate regex;

    use regex::Regex;
    use std::fs::File;
    use std::io::Read;

    pub struct Header {
        pub n_children: u8,
        pub n_metadata: u8,
    }

    impl Header {
        pub fn new(n_children: u8, n_metadata: u8) -> Header {
            Header {
                n_children: n_children,
                n_metadata: n_metadata,
            }
        }
    }

    pub struct Node {
        pub header: Header,
        pub children: Vec<Node>,
        pub metadata: Vec<u8>,
        pub size: usize,
    }

    impl Node {
        pub fn new(slice: &[u8]) -> Node {
            let n_children = slice[0];
            let n_metadata = slice[1];
            let header = Header::new(n_children, n_metadata);

            let mut children = Vec::new();
            let mut idx = 2;
            for _ in 0..n_children {
                let child = Node::new(&slice[idx..]);
                idx += child.size;
                children.push(child);
            }

            let mut size = 2 + n_metadata as usize;
            for child in &children {
                size += child.size;
            }

            let mut metadata = Vec::new();
            let m_off = size - n_metadata as usize;
            for byte in &slice[m_off..m_off + n_metadata as usize] {
                metadata.push(*byte);
            }

            Node {
                header: header,
                children: children,
                metadata: metadata,
                size: size,
            }
        }

        pub fn metadata_sum(&self) -> u32 {
            let mut output = 0;
            for m in &self.metadata {
                output += *m as u32;
            }
            for child in &self.children {
                output += child.metadata_sum();
            }
            output
        }

        pub fn get_value(&self) -> u32 {
            let n_children = self.header.n_children;
            if n_children == 0 {
                return self.metadata_sum();
            } else {
                let mut value = 0;
                for i in &self.metadata {
                    if *i <= n_children && *i != 0 {
                        value += self.children[(*i - 1) as usize].get_value();
                    }
                }
                return value;
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

    pub fn input_to_bytes(input: &str) -> Vec<u8> {
        let mut nums = Vec::new();
        let re = Regex::new(r"(\d+)").unwrap();
        for cap in re.captures_iter(input) {
            nums.push(cap[1].parse::<u8>().unwrap());
        }
        nums
    }

    pub fn part1(input: &str) -> u32 {
        let nums = input_to_bytes(input);
        let root_node = Node::new(&nums[..]);
        root_node.metadata_sum()
    }

    pub fn part2(input: &str) -> u32 {
        let nums = input_to_bytes(input);
        let root_node = Node::new(&nums[..]);
        root_node.get_value()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            let input = String::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
            assert_eq!(part1(&input), 138);
        }

        #[test]
        fn part2examples() {
            let input = String::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
            assert_eq!(part2(&input), 66);
        }
    }
}
