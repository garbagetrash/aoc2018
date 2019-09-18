pub mod day15 {
    extern crate ncurses;
    extern crate regex;

    use ncurses::*;
    use std::cmp::Ordering;
    use std::fs::File;
    use std::io::Read;
    use std::{thread, time};

    #[derive(Debug, Clone, PartialEq)]
    pub enum TileType {
        Wall,
        Open,
        Goblin,
        Elf,
    }

    #[derive(Debug, Clone)]
    pub struct Tile {
        tile_type: TileType,
        pos: (usize, usize),
    }

    impl Tile {
        pub fn new(pos: (usize, usize), tile_type: TileType) -> Tile {
            Tile {
                pos,
                tile_type,
            }
        }
    }

    fn dist_to_tile(pos1: &(usize, usize), pos2: &(usize, usize)) -> u32 {
        let mut dist = (pos1.0 as i32 - pos2.0 as i32).abs() as u32;
        dist += (pos1.1 as i32 - pos2.1 as i32).abs() as u32;
        dist
    }

    #[derive(Debug)]
    pub struct Goblin {
        pos: (usize, usize),
        hp: i32,
    }

    impl Goblin {
        pub fn new(pos: (usize, usize)) -> Goblin {
            Goblin { pos: pos, hp: 200 }
        }

        pub fn attack(&self, elf: &mut Elf) {
            elf.hp -= 3;
        }
    }

    #[derive(Debug)]
    pub struct Elf {
        pos: (usize, usize),
        hp: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Side {
        Goblin,
        Elf,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Unit {
        pos: (usize, usize),
        side: Side,
        hp: i32,
    }

    impl Unit {
        fn new(pos: (usize, usize), side: Side, hp: i32) -> Unit {
            Unit {
                pos,
                side,
                hp,
            }
        }

        fn attack(&self, other: &mut Unit) {
            other.hp -= 3;
        }

        fn turn(&mut self, board: &Board, units: &Vec<Unit>) {

            // ID possible targets
            let mut targets = vec![];
            for unit in units {
                if unit.side != self.side {
                    targets.push(unit.clone());
                }
            }
            targets.sort();

            // ID Open Tiles in range of the target (adjacent)
            let mut open_tiles = Vec::new();
            for target in &targets {
                for candidate in board.adjacent_tiles(target.pos) {
                    if let Some(tile) = candidate {
                        if tile.tile_type == TileType::Open {
                            let tile_range = dist_to_tile(&self.pos, &tile.pos);
                            open_tiles.push((tile_range, tile));
                        }
                    }
                }
            }
            println!("Possible targets: {:?}", &targets);

            // Make a vector of the ranges to targets
            let mut target_ranges = vec![];
            for target in &targets {
                target_ranges.push(dist_to_tile(&self.pos, &target.pos));
            }

            // Iterate through targets and choose one somehow
            for (target, t_range) in targets.iter().zip(target_ranges.iter()) {
                if *t_range == 1 {
                    // If already in range, attack
                    // TODO: ATTACK
                    // Have to make it possible to mutate others hp, no idea how to do this
                    return
                } else if open_tiles.len() == 0 {
                    // If not in range of target, and no Open Tiles adjacent,
                    // end turn
                    return
                }
            }
            // If not in range, then move
            // TODO: MOVE
            // create (range, target) tuple vec, sort_by() range, take(1), move
            // towards him

            // If after move in range, attack

            // Make a vector of the ranges to targets
            let mut target_ranges = vec![];
            for target in &targets {
                target_ranges.push(dist_to_tile(&self.pos, &target.pos));
            }

            for (target, t_range) in targets.iter().zip(target_ranges.iter()) {
                if *t_range == 1{
                    // After attacking, end turn
                    // TODO: ATTACK
                    // Have to make it possible to mutate others hp, no idea how to do this
                    return
                }
            }
        }
    }

    impl Ord for Unit {
        fn cmp(&self, other: &Self) -> Ordering {
            self.pos.cmp(&other.pos)
        }
    }

    impl PartialOrd for Unit {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug)]
    pub struct Board {
        tiles: Vec<Vec<Tile>>,
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                tiles: Vec::new(),
            }
        }

        pub fn adjacent_tiles(&self, pos: (usize, usize)) -> Vec<Option<Tile>> {
            let mut output = Vec::new();

            // Up
            if pos.0 > 0 {
                output.push(Some(self.tiles[pos.0 - 1][pos.1].clone()));
            } else {
                output.push(None);
            }

            // Right
            if pos.1 < self.tiles[0].len() - 1 {
                output.push(Some(self.tiles[pos.0][pos.1 + 1].clone()));
            } else {
                output.push(None);
            }

            // Down
            if pos.0 < self.tiles.len() - 1 {
                output.push(Some(self.tiles[pos.0 + 1][pos.1].clone()));
            } else {
                output.push(None);
            }

            // Left
            if pos.1 > 0 {
                output.push(Some(self.tiles[pos.0][pos.1 - 1].clone()));
            } else {
                output.push(None);
            }

            output
        }
    }

    pub fn load_input(filename: &str) -> String {
        let mut buffer = String::new();
        File::open(filename)
            .expect(&format!("No file {}", filename))
            .read_to_string(&mut buffer)
            .unwrap();
        buffer.to_string()
    }

    pub fn parse_input(input: &str) -> (Board, Vec<Unit>) {
        let mut board = Board::new();
        let mut units = vec![];
        for (row, l) in input.lines().enumerate() {
            let mut board_row = Vec::new();
            for (col, c) in l.chars().enumerate() {
                match c {
                    '#' => board_row.push(Tile::new((row, col), TileType::Wall)),
                    '.' => board_row.push(Tile::new((row, col), TileType::Open)),
                    'G' => {
                        board_row.push(Tile::new((row, col), TileType::Goblin));
                        units.push(Unit::new((row, col), Side::Goblin, 200));
                    }
                    'E' => {
                        board_row.push(Tile::new((row, col), TileType::Elf));
                        units.push(Unit::new((row, col), Side::Elf, 200));
                    }
                    _ => (),
                };
            }
            board.tiles.push(board_row);
        }
        (board, units)
    }

    pub fn part1(input: &str) -> u32 {
        let (mut board, mut units) = parse_input(input);
        units.sort();
        println!("{:?}", units);

        // Simulate a round
        let units_copy = units.clone();
        for unit in &mut units {
            unit.turn(&board, &units_copy);
        }
        println!("{:?}", units);

        0
    }

    pub fn part2(input: &str) -> u32 {
        let (mut board, mut units) = parse_input(input);
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_part1_example1() {
            let input = load_input("inputs/15example1.txt");
            assert_eq!(part1(&input), 27730);
        }

        #[test]
        fn test_part1_example2() {
            let input = load_input("inputs/15example2.txt");
            assert_eq!(part1(&input), 36334);
        }

        #[test]
        fn test_part2() {
            let input = load_input("inputs/15.txt");
            assert_eq!(part2(&input), 0);
        }
    }
}
