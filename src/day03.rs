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

    impl Rect {
        pub fn new(id: u32, space_left: u32, space_top: u32, width: u32, height: u32) -> Rect {
            Rect {
                id: id,
                space_left: space_left,
                space_top: space_top,
                width: width,
                height: height,
            }
        }
    }

    pub fn load_input() -> Vec<Rect> {
        let mut f = BufReader::new(File::open("inputs/03.txt").unwrap());
        let mut output = Vec::new();
        for line in f.lines() {
            let mut line = line.unwrap().clone();
            let mut line_split: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
            let id = line_split[0].split('#').collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
            let space_left = line_split[2].split(',').collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let space_top = line_split[2].split(',').collect::<Vec<&str>>()[1]
                .split(':')
                .collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let width = line_split[3].split('x').collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let height = line_split[3].split('x').collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

            output.push(Rect::new(id, space_left, space_top, width, height));
        }
        output
    }

    pub fn intersection(r1: &Rect, r2: &Rect, mut set: HashSet<(u32, u32)>) -> HashSet<(u32, u32)> {
        let left = std::cmp::max(r1.space_left, r2.space_left);
        let right = std::cmp::min(r1.space_left + r1.width, r2.space_left + r2.width);

        if left >= right {
            return set;
        }

        let top = std::cmp::max(r1.space_top, r2.space_top);
        let bottom = std::cmp::min(r1.space_top + r1.height, r2.space_top + r2.height);

        if bottom <= top {
            return set;
        }

        for w in left..right {
            for h in top..bottom {
                println!("{}, {}", w, h);
                set.insert((w, h));
            }
        }
        set
    }

    pub fn part1(input: &Vec<Rect>) -> u32 {
        let mut set: HashSet<_> = HashSet::new();
        for i in 0..input.len() {
            for j in (i + 1)..input.len() {
                set = intersection(&input[i], &input[j], set);
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
        use std::collections::HashSet;

        #[test]
        fn test_intersection() {
            let mut set = HashSet::new();
            set = intersection(&Rect::new(0, 0, 0, 5, 5), &Rect::new(1, 1, 1, 3, 3), set);
            assert_eq!(set.len(), 9);
        }

        #[test]
        fn test_intersection2() {
            let mut set = HashSet::new();
            set = intersection(&Rect::new(0, 3, 1, 2, 5), &Rect::new(1, 1, 1, 3, 3), set);
            assert_eq!(set.len(), 3);
        }

        #[test]
        fn test_intersection3() {
            let mut set = HashSet::new();
            set = intersection(&Rect::new(0, 3, 1, 2, 5), &Rect::new(1, 0, 0, 5, 5), set);
            assert_eq!(set.len(), 8);
        }

        #[test]
        fn test_intersection4() {
            let mut set = HashSet::new();
            set = intersection(&Rect::new(0, 3, 1, 2, 5), &Rect::new(1, 7, 0, 5, 5), set);
            assert_eq!(set.len(), 0);
        }

        #[test]
        fn test_intersection5() {
            println!("Starting test 5");
            let r1 = Rect::new(0, 1, 3, 4, 4);
            let r2 = Rect::new(1, 3, 1, 4, 4);
            let r3 = Rect::new(2, 5, 5, 2, 2);
            let mut input = Vec::new();
            input.push(r1);
            input.push(r2);
            input.push(r3);
            assert_eq!(part1(&input), 4);
        }

        #[test]
        fn part1examples() {
            assert_eq!(0, 0);
        }

        #[test]
        fn part2examples() {
            assert_eq!(0, 0);
        }
    }
}
