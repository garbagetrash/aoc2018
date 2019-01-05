pub mod day15 {
    extern crate ncurses;
    extern crate regex;

    use ncurses::*;
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
                pos: pos,
                tile_type: tile_type,
            }
        }
    }

    #[derive(Debug)]
    pub struct Goblin {
        pos: (usize, usize),
        hp: i32,
    }

    impl Goblin {
        pub fn new(pos: (usize, usize), hp: i32) -> Goblin {
            Goblin { pos: pos, hp: hp }
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

    impl Elf {
        pub fn new(pos: (usize, usize), hp: i32) -> Elf {
            Elf { pos: pos, hp: hp }
        }

        pub fn attack(&self, goblin: &mut Goblin) {
            goblin.hp -= 3;
        }

        pub fn turn(&mut self, board: &Board) {
            // ID possible targets
            let targets = &board.goblins;

            // ID Open Tiles in range of the target (adjacent)
            let mut in_range = Vec::new();
            for target in targets {
                for candidate in board.adjacent_tiles(target.pos) {
                    if let Some(tile) = candidate {
                        if tile.tile_type == TileType::Open {
                            in_range.push(tile);
                        }
                    }
                }
            }

            // If not in range of target, and no Open Tiles adjacent, end turn

            // If already in range, attack

            // If not in range, then move

            // If after move in range, attack

            // After attacking, end turn
        }
    }

    #[derive(Debug)]
    pub struct Board {
        board: Vec<Vec<Tile>>,
        goblins: Vec<Goblin>,
        elves: Vec<Elf>,
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                board: Vec::new(),
                goblins: Vec::new(),
                elves: Vec::new(),
            }
        }

        pub fn adjacent_tiles(&self, pos: (usize, usize)) -> Vec<Option<Tile>> {
            let mut output = Vec::new();

            // Up
            if pos.0 > 0 {
                output.push(Some(self.board[pos.0 - 1][pos.1].clone()));
            } else {
                output.push(None);
            }

            // Right
            if pos.1 < self.board[0].len() - 1 {
                output.push(Some(self.board[pos.0][pos.1 + 1].clone()));
            } else {
                output.push(None);
            }

            // Down
            if pos.0 < self.board.len() - 1 {
                output.push(Some(self.board[pos.0 + 1][pos.1].clone()));
            } else {
                output.push(None);
            }

            // Left
            if pos.1 > 0 {
                output.push(Some(self.board[pos.0][pos.1 - 1].clone()));
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

    pub fn parse_input(input: &str) -> Board {
        let mut board = Board::new();
        for (row, l) in input.lines().enumerate() {
            let mut board_row = Vec::new();
            for (col, c) in l.chars().enumerate() {
                match c {
                    '#' => board_row.push(Tile::new((row, col), TileType::Wall)),
                    '.' => board_row.push(Tile::new((row, col), TileType::Open)),
                    'G' => {
                        board_row.push(Tile::new((row, col), TileType::Goblin));
                        board.goblins.push(Goblin::new((row, col), 200));
                    }
                    'E' => {
                        board_row.push(Tile::new((row, col), TileType::Elf));
                        board.elves.push(Elf::new((row, col), 200));
                    }
                    _ => (),
                };
            }
            board.board.push(board_row);
        }
        board
    }

    pub fn part1(input: &str) -> u32 {
        let mut board = parse_input(input);
        0
    }

    pub fn part2(input: &str) -> u32 {
        let mut board = parse_input(input);
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
