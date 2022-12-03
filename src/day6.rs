use std::collections::HashMap;
use std::collections::HashSet;
use scan_fmt::scan_fmt;

#[aoc_generator(day6)]
pub fn load_input(input: &str) -> String {
    input.to_string()
}

pub fn point_vec(in_str: &str) -> Vec<(i32, i32)> {
    let mut vec = vec![];
    for line in in_str.lines() {
        vec.push(scan_fmt!(line, "{}, {}", i32, i32).unwrap());
    }
    vec
}

fn grid_limits(pts: &[(i32, i32)]) -> (i32, i32, i32, i32) {
    // return grid (xmin, xmax, ymin, ymax)
    let mut xmin = pts[0].0;
    let mut xmax = pts[0].0;
    let mut ymin = pts[0].1;
    let mut ymax = pts[0].1;
    for pt in pts {
        if pt.0 < xmin {
            xmin = pt.0;
        } else if pt.0 > xmax {
            xmax = pt.0;
        }
        if pt.1 < ymin {
            ymin = pt.1;
        } else if pt.1 > ymax {
            ymax = pt.1;
        }
    }
    (xmin, xmax, ymin, ymax)
}

pub fn man_dist(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as u32
}

pub fn closest_point(pt: &(i32, i32), pts: &[(i32, i32)]) -> usize {
    // TODO: Doesn't detect if 2+ points have equal min. distance
    let mut min_dist = man_dist(pts[0], *pt);
    let mut min_idx = 0;
    for (i, p) in pts.iter().enumerate() {
        let dist = man_dist(*p, *pt);
        if dist < min_dist {
            min_dist = dist;
            min_idx = i;
        }
    }
    min_idx
}

pub fn make_map(pts: &[(i32, i32)]) -> HashMap<(i32, i32), u32> {
    // Returns hashmap with grid of points and value index of pt closest
    let mut output = HashMap::new();
    let (xmin, xmax, ymin, ymax) = grid_limits(pts);
    for x in xmin..xmax+1 {
        for y in ymin..ymax+1 {
            let c_idx = closest_point(&(x, y), pts);
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

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let pts = point_vec(input);

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

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let pt_vec = point_vec(input);
    let map = part2_map(&pt_vec);
    let mut cnt = 0;
    for val in map.values() {
        if val < &32 {
            cnt += 1;
        }
    }
    cnt
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2018/06.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 17);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2018/06.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 16);
    }
}
