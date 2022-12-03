pub mod day13 {
    extern crate ncurses;
    extern crate regex;

    use ncurses::*;
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use std::{thread, time};

    #[derive(Debug)]
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
        buffer.to_string()
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Direction {
        Left,
        Straight,
        Right,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Cart {
        pos: (usize, usize),
        next_dir: Direction,
        orientation: char,
    }

    impl PartialOrd for Cart {
        fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
            let self_tup = (self.pos.1, self.pos.0);
            let other_tup = (other.pos.1, other.pos.0);
            Some(self_tup.cmp(&other_tup))
        }
    }

    impl Ord for Cart {
        fn cmp(&self, other: &Cart) -> Ordering {
            let self_tup = (self.pos.1, self.pos.0);
            let other_tup = (other.pos.1, other.pos.0);
            self_tup.cmp(&other_tup)
        }
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
                _ => panic!("Invalid orientation: {}", self.orientation),
            }

            // Turn
            match track_map
                .get(&self.pos)
                .expect(&format!("No track at {:?}", &self.pos))
            {
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
                }
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
                }
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
                }
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
                    }
                    '^' => {
                        carts.push(Cart::new((col, row), Direction::Left, '^'));
                        track_map.insert((col, row), TrackType::Vertical)
                    }
                    '>' => {
                        carts.push(Cart::new((col, row), Direction::Left, '>'));
                        track_map.insert((col, row), TrackType::Horizontal)
                    }
                    'v' => {
                        carts.push(Cart::new((col, row), Direction::Left, 'v'));
                        track_map.insert((col, row), TrackType::Vertical)
                    }
                    _ => None,
                };
            }
        }
        (track_map, carts)
    }

    pub fn print_board(
        carts: &Vec<Cart>,
        track_map: &HashMap<(usize, usize), TrackType>,
        t: i32,
        sleep_time: u64,
    ) {
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
            if cart.orientation != 'X' {
                mvprintw(
                    (cart.pos.1 + 2) as i32,
                    cart.pos.0 as i32,
                    &(String::from_utf8_lossy(&[cart.orientation as u8])),
                );
            }
        }
        refresh();
        let sleep_time = time::Duration::from_millis(sleep_time);
        thread::sleep(sleep_time);
        mv(0, 0);
    }

    // Set sleep_time to 0 to skip the animation
    pub fn part1(input: &str, sleep_time: u64) -> (usize, usize) {
        let (track_map, mut carts) = parse_input(input);

        if sleep_time > 0 {
            initscr();
            noecho();
        }
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
                            if sleep_time > 0 {
                                endwin();
                            }
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

        if sleep_time > 0 {
            initscr();
            noecho();
        }
        let mut tick_num = 0;
        loop {
            if sleep_time > 0 {
                print_board(&carts, &track_map, tick_num, sleep_time);
            }

            // Iterate through carts
            carts.sort();
            for i in 0..carts.len() {
                if carts[i].orientation != 'X' {
                    carts[i].tick(&track_map);

                    // Collision detection
                    for j in 0..carts.len() {
                        if carts[j].orientation != 'X' && j != i {
                            if carts[j].pos == carts[i].pos {
                                carts[j].orientation = 'X';
                                carts[i].orientation = 'X';
                            }
                        }
                    }
                }
            }

            // Count remaining carts
            let mut active_cart_cnt = 0;
            for cart in &carts {
                if cart.orientation != 'X' {
                    active_cart_cnt += 1;
                }
            }

            // If only 1 cart left, do a final tick and return
            if active_cart_cnt == 1 {
                for cart in &mut carts {
                    if cart.orientation != 'X' {
                        if sleep_time > 0 {
                            endwin();
                        }
                        return cart.pos;
                    }
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
            assert_eq!(part2(&input, 0), (6, 4));
        }
    }
}
