pub mod day10 {
    extern crate regex;

    use regex::Regex;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    pub fn load_input(filename: &str) -> String {
        let mut buffer = String::new();
        File::open(filename)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        buffer.trim().to_string()
    }

    pub fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
        let re = Regex::new(r"^position=<\s*([\-\d]+),\s*([\-\d]+)> velocity=<\s*([\-\d]+),\s*([\-\d]+)>").unwrap();
        let mut pos = Vec::new();
        let mut vel = Vec::new();
        for cap in re.captures_iter(input) {
            let xpos = cap[1].parse::<i32>().unwrap();
            let ypos = cap[2].parse::<i32>().unwrap();
            pos.push((xpos, ypos));
            println!("Positions: {:?}", pos);
            let xvel = cap[3].parse::<i32>().unwrap();
            let yvel = cap[4].parse::<i32>().unwrap();
            vel.push((xvel, yvel));
            println!("Velocities: {:?}", vel);
        }
        println!("Positions: {:?}", pos);
        println!("Velocities: {:?}", vel);
        (pos, vel)
    }

    pub fn print_board() {

    }

    pub fn part1(input: &str) -> String {
        let (pos, vel) = parse_input(input);
        String::new()
    }

    pub fn part2(input: &str) -> usize {
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples1() {
            let input = String::from("position=< 9,  1> velocity=< 0,  2>
                                    position=< 7,  0> velocity=<-1,  0>
                                    position=< 3, -2> velocity=<-1,  1>
                                    position=< 6, 10> velocity=<-2, -1>
                                    position=< 2, -4> velocity=< 2,  2>
                                    position=<-6, 10> velocity=< 2, -2>
                                    position=< 1,  8> velocity=< 1, -1>
                                    position=< 1,  7> velocity=< 1,  0>
                                    position=<-3, 11> velocity=< 1, -2>
                                    position=< 7,  6> velocity=<-1, -1>
                                    position=<-2,  3> velocity=< 1,  0>
                                    position=<-4,  3> velocity=< 2,  0>
                                    position=<10, -3> velocity=<-1,  1>
                                    position=< 5, 11> velocity=< 1, -2>
                                    position=< 4,  7> velocity=< 0, -1>
                                    position=< 8, -2> velocity=< 0,  1>
                                    position=<15,  0> velocity=<-2,  0>
                                    position=< 1,  6> velocity=< 1,  0>
                                    position=< 8,  9> velocity=< 0, -1>
                                    position=< 3,  3> velocity=<-1,  1>
                                    position=< 0,  5> velocity=< 0, -1>
                                    position=<-2,  2> velocity=< 2,  0>
                                    position=< 5, -2> velocity=< 1,  2>
                                    position=< 1,  4> velocity=< 2,  1>
                                    position=<-2,  7> velocity=< 2, -2>
                                    position=< 3,  6> velocity=<-1, -1>
                                    position=< 5,  0> velocity=< 1,  0>
                                    position=<-6,  0> velocity=< 2,  0>
                                    position=< 5,  9> velocity=< 1, -2>
                                    position=<14,  7> velocity=<-2,  0>
                                    position=<-3,  6> velocity=< 2, -1>");
            assert_eq!(0, 0);
        }

        #[test]
        fn part2examples() {
            println!("Not yet implemented!");
            assert_eq!(0, 0);
        }
    }
}
