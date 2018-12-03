pub mod day03 {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    pub struct Rect {
        id: u32,
        space_left: u32,
        space_top: u32,
        width: u32,
        height: u32,
    }

    pub fn load_input() -> Vec<Rect> {
        let mut f = BufReader::new(File::open("inputs/03.txt").unwrap());
        let mut output = Vec::new();
        for line in f.lines() {
            let mut line = line.unwrap().clone();
            let mut line_split: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
            let id = line_split[0].split('#').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
            let space_left = line_split[2].split(',').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            let space_top = line_split[2].split(',').collect::<Vec<&str>>()[1].split(':').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            let width = line_split[3].split('x').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            let height = line_split[3].split('x').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

            output.push(Rect { id: id, space_left: space_left, space_top: space_top, width: width, height: height });
        }
        output
    }

    pub fn intersection(r1: &Rect, r2: &Rect) -> HashSet<(u32, u32)> {
        let mut set = HashSet::new();
        for w in r1.space_left..(r1.space_left + r1.width) {
            if w > r2.space_left && w < (r2.space_left + r2.width) {
                for h in r1.space_top..(r1.space_top + r1.height) {
                    if h > r2.space_top && h < (r2.space_top + r2.height) {
                        set.insert((w, h));
                    }
                }
            }
        }
        set.clone()
    }

    pub fn part1(input: &Vec<Rect>) -> u32 {
        let mut set = HashSet::new();
        for r1 in input {
            for r2 in input {
                let set: HashSet<_> = set.union(&intersection(r1, r2)).collect();
            }
        }
        println!("{}", set.len() as u32);

        set.len() as u32
    }

    pub fn part2(input: &Vec<Rect>) -> i32 {
        println!("Not yet implemented!");
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            assert_eq!(part1(), 0);
        }

        #[test]
        fn part2examples() {
            assert_eq!(part2(), 0);
        }
    }
}
