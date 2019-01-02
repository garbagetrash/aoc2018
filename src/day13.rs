pub mod day13 {
    extern crate ncurses;
    extern crate regex;

    use ncurses::*;
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;

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
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        buffer.trim().to_string()
    }

    pub enum Direction {
        Left,
        Straight,
        Right,
    }

    pub struct Cart {
        pos: (usize, usize),
        last_dir: Direction,
    }

    impl Cart {
        pub fn new(pos: (usize, usize), last_dir: Direction) -> Cart {
            Cart {
                pos: pos,
                last_dir: last_dir,
            }
        }

        pub fn tick(&mut self, track_map: HashMap<(usize, usize), TrackType>) {

        }
    }

    pub fn parse_input(input: &str) -> HashMap<(usize, usize), TrackType> {
        let mut track_map: HashMap<(usize, usize), TrackType> = HashMap::new();
        for (row, l) in input.lines().enumerate() {
            for (col, c) in l.chars().enumerate() {
                match c {
                    '|' => track_map.insert((row, col), TrackType::Vertical),
                    '-' => track_map.insert((row, col), TrackType::Horizontal),
                    '/' => track_map.insert((row, col), TrackType::ForwardSlash),
                    '\\' => track_map.insert((row, col), TrackType::BackSlash),
                    '+' => track_map.insert((row, col), TrackType::Intersection),
                    '<' => carts.push(Cart::new((row, col), Direction::Left)),
                    '^' => None,
                    '>' => None,
                    'v' => None,
                    _ => None,
                };
            }
        }
        track_map
    }

    pub fn print_board(pts: &Vec<(i32, i32)>, t: i32) {

        let minx = pts.iter().map(|tup| tup.0).min().unwrap();
        let miny = pts.iter().map(|tup| tup.1).min().unwrap();

        initscr();
        noecho();
        let tmsg = format!("Time: {:?}", t);
        mvprintw(0, 0, &tmsg);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        for pt in pts {
            mvprintw(pt.1 - miny + 1, pt.0 - minx, "#");
        }
        refresh();
        getch();
        clear();
        mv(0, 0);
        endwin();
    }

    pub fn part1(input: &str) -> String {
        let track_map = parse_input(input);
        String::new()
    }

    pub fn part2(input: &str) -> String {
        let track_map = parse_input(input);
        String::new()
    }
}
