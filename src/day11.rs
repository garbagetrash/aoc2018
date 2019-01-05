pub mod day11 {
    extern crate rayon;

    use rayon::prelude::*;

    pub fn power_level(x: i32, y: i32, serial_num: i32) -> i32 {
        let rack_id = x + 10;
        let mut temp = rack_id * y;
        temp += serial_num;
        temp *= rack_id;
        let str_arr = temp.to_string();
        let mut hundreds: i32 = 0;
        if str_arr.len() > 2 {
            hundreds = (str_arr.as_bytes()[str_arr.len() - 3] - 48) as i32;
        }
        hundreds - 5
    }

    pub fn construct_grid(serial_num: i32) -> [[i32; 300]; 300] {
        let mut grid = [[0i32; 300]; 300];
        for x in 0..300 {
            for y in 0..300 {
                grid[x][y] = power_level(x as i32, y as i32, serial_num);
            }
        }
        grid
    }

    pub fn construct_nxn_grid(grid: &[[i32; 300]; 300], n: usize) -> (i32, usize, usize) {
        let max_range = 300 - n + 1;
        let mut max = std::i32::MIN;
        let mut xmax = 0;
        let mut ymax = 0;
        for x in 0..max_range {
            for y in 0..max_range {
                let mut temp = 0i32;
                for j in 0..n {
                    for k in 0..n {
                        temp += grid[x + j][y + k];
                    }
                }
                if temp > max {
                    max = temp;
                    xmax = x;
                    ymax = y;
                }
            }
        }
        (max, xmax, ymax)
    }

    pub fn part1(input: i32) -> (usize, usize) {
        let grid = construct_grid(input);
        let (_, xmax, ymax) = construct_nxn_grid(&grid, 3);
        (xmax, ymax)
    }

    pub fn part2(input: i32) -> (usize, usize, usize) {
        let grid = construct_grid(input);

        let mut max = std::i32::MIN;
        let mut xmax = 0;
        let mut ymax = 0;
        let mut nmax = 1;
        let n: Vec<usize> = (1..300).collect();

        let tupvec = n
            .par_iter()
            .map(|&n| {
                let (m, xm, ym) = construct_nxn_grid(&grid, n);
                (m, xm, ym, n)
            })
            .collect::<Vec<(i32, usize, usize, usize)>>();

        for tup in tupvec {
            if tup.0 > max {
                max = tup.0;
                xmax = tup.1;
                ymax = tup.2;
                nmax = tup.3;
            }
        }

        (xmax, ymax, nmax as usize)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn calc_power_level1() {
            let serial_num = 8;
            let x = 3;
            let y = 5;
            assert_eq!(power_level(x, y, serial_num), 4);
        }

        #[test]
        fn calc_power_level2() {
            let serial_num = 57;
            let x = 122;
            let y = 79;
            assert_eq!(power_level(x, y, serial_num), -5);
        }

        #[test]
        fn calc_power_level3() {
            let serial_num = 39;
            let x = 217;
            let y = 196;
            assert_eq!(power_level(x, y, serial_num), 0);
        }

        #[test]
        fn calc_power_level4() {
            let serial_num = 71;
            let x = 101;
            let y = 153;
            assert_eq!(power_level(x, y, serial_num), 4);
        }

        #[test]
        fn part1example1() {
            let input = 18;
            assert_eq!(part1(input), (33, 45));
        }

        #[test]
        fn part1example2() {
            let input = 42;
            assert_eq!(part1(input), (21, 61));
        }

        /*
        #[test]
        fn part2example1() {
            let input = 18;
            assert_eq!(part2(input), (90, 269, 16));
        }

        #[test]
        fn part2example2() {
            let input = 42;
            assert_eq!(part2(input), (232, 251, 12));
        }
        */
    }
}
