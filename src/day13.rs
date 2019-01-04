pub mod day13 {
    extern crate ncurses;
    extern crate regex;

    use ncurses::*;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use std::{thread, time};

    pub enum TrackType {
        Vertical,
        Horizontal,
        ForwardSlash,
        BackSlash,
        Intersection,
    }

    pub fn load_input(filename: &str) -> String {
        let mut buffer = String::new();
        File::open(filename)
            .expect(&format!("No file {}", filename))
            .read_to_string(&mut buffer)
            .unwrap();
        buffer.trim().to_string()
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Clone)]
    pub enum Direction {
        Left,
        Straight,
        Right,
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Cart {
        pos: (usize, usize),
        next_dir: Direction,
        orientation: char,
    }

    impl Cart {
        pub fn new(pos: (usize, usize), next_dir: Direction, orientation: char) -> Cart {
            Cart {
                pos: pos,
                next_dir: next_dir,
                orientation: orientation,
            }
        }

        pub fn tick(&mut self, track_map: &HashMap<(usize, usize), TrackType>) {
            // Step
            match self.orientation {
                '>' => self.pos.0 += 1,
                'v' => self.pos.1 += 1,
                '<' => self.pos.0 -= 1,
                '^' => self.pos.1 -= 1,
                _ => panic!(format!("Invalid orientation: {}", self.orientation)),
            }

            // Turn
            match track_map.get(&self.pos).expect(&format!("No track at {:?}", &self.pos)) {
                TrackType::ForwardSlash => {
                    if self.orientation == '^' {
                        self.orientation = '>';
                    } else if self.orientation == '>' {
                        self.orientation = '^';
                    } else if self.orientation == 'v' {
                        self.orientation = '<';
                    } else if self.orientation == '<' {
                        self.orientation = 'v';
                    }
                },
                TrackType::BackSlash => {
                    if self.orientation == '^' {
                        self.orientation = '<';
                    } else if self.orientation == '>' {
                        self.orientation = 'v';
                    } else if self.orientation == 'v' {
                        self.orientation = '>';
                    } else if self.orientation == '<' {
                        self.orientation = '^';
                    }
                },
                TrackType::Intersection => {
                    if self.next_dir == Direction::Left {
                        self.next_dir = Direction::Straight;
                        if self.orientation == '>' {
                            self.orientation = '^';
                        } else if self.orientation == 'v' {
                            self.orientation = '>';
                        } else if self.orientation == '<' {
                            self.orientation = 'v';
                        } else {
                            self.orientation = '<';
                        }
                    } else if self.next_dir == Direction::Straight {
                        self.next_dir = Direction::Right;
                    } else {
                        self.next_dir = Direction::Left;
                        if self.orientation == '>' {
                            self.orientation = 'v';
                        } else if self.orientation == 'v' {
                            self.orientation = '<';
                        } else if self.orientation == '<' {
                            self.orientation = '^';
                        } else {
                            self.orientation = '>';
                        }
                    }
                },
                _ => (),
            }
        }
    }

    pub fn parse_input(input: &str) -> (HashMap<(usize, usize), TrackType>, Vec<Cart>) {
        let mut carts = Vec::new();
        let mut track_map: HashMap<(usize, usize), TrackType> = HashMap::new();
        for (row, l) in input.lines().enumerate() {
            for (col, c) in l.chars().enumerate() {
                match c {
                    '|' => track_map.insert((col, row), TrackType::Vertical),
                    '-' => track_map.insert((col, row), TrackType::Horizontal),
                    '/' => track_map.insert((col, row), TrackType::ForwardSlash),
                    '\\' => track_map.insert((col, row), TrackType::BackSlash),
                    '+' => track_map.insert((col, row), TrackType::Intersection),
                    '<' => {
                        carts.push(Cart::new((col, row), Direction::Left, '<'));
                        track_map.insert((col, row), TrackType::Horizontal)
                    },
                    '^' => {
                        carts.push(Cart::new((col, row), Direction::Left, '^'));
                        track_map.insert((col, row), TrackType::Vertical)
                    },
                    '>' => {
                        carts.push(Cart::new((col, row), Direction::Left, '>'));
                        track_map.insert((col, row), TrackType::Horizontal)
                    },
                    'v' => {
                        carts.push(Cart::new((col, row), Direction::Left, 'v'));
                        track_map.insert((col, row), TrackType::Vertical)
                    },
                    _ => None,
                };
            }
        }
        (track_map, carts)
    }

    pub fn print_board(carts: &Vec<Cart>, track_map: &HashMap<(usize, usize), TrackType>, t: i32, sleep_time: u64) {

        initscr();
        noecho();
        let tmsg = format!("Tick: {:?}", t);
        mvprintw(0, 0, &tmsg);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        for (pos, track_type) in track_map.iter() {
            let mut track = String::new();
            match track_type {
                TrackType::Vertical => track.push('|'),
                TrackType::Horizontal => track.push('-'),
                TrackType::ForwardSlash => track.push('/'),
                TrackType::BackSlash => track.push('\\'),
                TrackType::Intersection => track.push('+'),
            }
            mvprintw((pos.1 + 2) as i32, pos.0 as i32, &track);
        }
        for cart in carts {
            mvprintw((cart.pos.1 + 2) as i32, cart.pos.0 as i32, &(String::from_utf8_lossy(&[cart.orientation as u8])));
        }
        refresh();
        let sleep_time = time::Duration::from_millis(sleep_time);
        thread::sleep(sleep_time);
        clear();
        mv(0, 0);
        endwin();
    }

    // Set sleep_time to 0 to skip the animation
    pub fn part1(input: &str, sleep_time: u64) -> (usize, usize) {
        let (track_map, mut carts) = parse_input(input);

        let mut tick_num = 0;
        loop {
            if sleep_time > 0 {
                print_board(&carts, &track_map, tick_num, sleep_time);
            }
            for i in 0..carts.len() {
                carts[i].tick(&track_map);

                // Collision detection
                for j in 0..carts.len() {
                    if j != i {
                        if carts[j].pos == carts[i].pos {
                            return carts[i].pos;
                        }
                    }
                }
            }
            tick_num += 1;
        }
    }

    pub fn part2(input: &str, sleep_time: u64) -> (usize, usize) {
        let (track_map, mut carts) = parse_input(input);
        let mut tick_num = 0;
        loop {
            if sleep_time > 0 {
                print_board(&carts, &track_map, tick_num, sleep_time);
            }
            for i in 0..carts.len() {
                carts[i].tick(&track_map);

                if carts.len() == 1 {
                    return carts[0].pos;
                }

                // Collision detection
                let mut remove_vec = Vec::new();
                for j in 0..carts.len() {
                    if j != i {
                        if carts[j].pos == carts[i].pos {
                            remove_vec.push(i);
                            remove_vec.push(j);
                        }
                    }
                }
                remove_vec.sort_by(|a, b| b.cmp(a));
                for idx in remove_vec {
                    carts.remove(idx);
                }
            }
            tick_num += 1;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_part1example1() {
            let input = load_input("inputs/13example1.txt");
            assert_eq!(part1(&input, 0), (0, 3));
        }

        #[test]
        fn test_part1example2() {
            let input = load_input("inputs/13example2.txt");
            assert_eq!(part1(&input, 0), (7, 3));
        }

        #[test]
        fn test_part2() {
            let input = load_input("inputs/13example3.txt");
            assert_eq!(part2(&input, 250), (6, 4));
        }
    }
}
