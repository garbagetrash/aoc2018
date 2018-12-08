pub mod day06 {
    extern crate regex;

    use regex::Regex;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Read;

    pub fn load_input(filename: &str) -> String {
        let mut buffer = String::new();
        File::open(filename)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        buffer.trim().to_string()
    }

    pub fn point_vec(in_str: &str) -> Vec<(i32, i32)> {
        let mut vec: Vec<(i32, i32)> = Vec::new();
        let re = Regex::new(r"(\d*), (\d*)").unwrap();
        for cap in re.captures_iter(in_str) {
            let x = &cap[1].parse::<i32>().unwrap();
            let y = &cap[2].parse::<i32>().unwrap();
            vec.push((*x, *y));
        }
        vec
    }

    pub fn man_dist(p1: (i32, i32), p2: (i32, i32)) -> u32 {
        ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as u32
    }

    pub fn closest_point(pt: &(i32, i32), pts: &Vec<(i32, i32)>) -> (i32, i32) {
        let mut min_dist = std::u32::MAX;
        let mut min_p = (0, 0);
        for p in pts {
            let dist = man_dist(*p, *pt);
            if dist < min_dist {
                min_dist = dist;
                min_p = *p;
            }
        }
        min_p
    }

    pub fn make_map(pts: &Vec<(i32, i32)>) -> HashMap<(i32, i32), u32> {
        // Returns hashmap with grid of points and value index of pt closest
        let mut output = HashMap::new();
        let r = 500;
        for x in 0..r {
            for y in 0..r {
                let c = closest_point(&(x, y), pts);
                let mut c_idx = 0;
                for idx in 0..pts.len() {
                    if c == pts[idx] {
                        c_idx = idx;
                        break;
                    }
                }
                output.insert((x, y), c_idx as u32);
            }
        }
        output
    }

    pub fn part2_map(pts: &Vec<(i32, i32)>) -> HashMap<(i32, i32), u32> {
        let mut output = HashMap::new();
        let r = 500;
        for x in 0..r {
            for y in 0..r {
                let sum_dist: u32 = pts.iter().map(|xt| man_dist(*xt, (x, y))).sum();
                output.insert((x, y), sum_dist);
            }
        }
        output
    }

    pub fn part1(input: &str) -> usize {
        let pt_vec = point_vec(input);
        let pts = pt_vec.iter().map(|x| *x).collect();

        let map = make_map(&pts);
        let mut c_idx = HashSet::new();
        for x in 0..500 {
            c_idx.insert(map.get(&(x, 0)).unwrap());
            c_idx.insert(map.get(&(x, 499)).unwrap());
            c_idx.insert(map.get(&(0, x)).unwrap());
            c_idx.insert(map.get(&(499, x)).unwrap());
        }

        let mut sizes = Vec::new();
        for i in 0..pts.len() {
            let mut cnt = 0;
            if !c_idx.contains(&(i as u32)) {
                for val in map.values() {
                    if i == *val as usize {
                        cnt += 1;
                    }
                }
            }
            sizes.push(cnt);
        }
        *sizes.iter().max().unwrap() as usize
    }

    pub fn part2(input: &str, lim: u32) -> usize {
        let pt_vec = point_vec(input);
        let map = part2_map(&pt_vec);
        let mut cnt = 0;
        for val in map.values() {
            if val < &lim {
                cnt += 1;
            }
        }
        cnt
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            let mut input = String::new();
            input.push_str(
                "1, 1
                1, 6
                8, 3
                3, 4
                5, 5
                8, 9",
            );
            assert_eq!(part1(&input), 17);
        }

        #[test]
        fn part2examples() {
            let mut input = String::new();
            input.push_str(
                "1, 1
                1, 6
                8, 3
                3, 4
                5, 5
                8, 9",
            );
            assert_eq!(part2(&input, 32), 16);
        }
    }
}
