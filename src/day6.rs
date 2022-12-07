use scan_fmt::scan_fmt;
use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc_generator(day6)]
fn load_input(input: &str) -> Vec<(i32, i32)> {
    let mut vec = vec![];
    for line in input.lines() {
        vec.push(scan_fmt!(line, "{}, {}", i32, i32).unwrap());
    }
    vec
}

fn grid_limits(pts: &[(i32, i32)]) -> (i32, i32, i32, i32) {
    // return grid (xmin, xmax, ymin, ymax)
    let xmin = pts.iter().map(|p| p.0).min().unwrap();
    let xmax = pts.iter().map(|p| p.0).max().unwrap();
    let ymin = pts.iter().map(|p| p.1).min().unwrap();
    let ymax = pts.iter().map(|p| p.1).max().unwrap();
    (xmin, xmax, ymin, ymax)
}

pub fn man_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

pub fn closest_points(pt: &(i32, i32), pts: &[(i32, i32)]) -> Vec<usize> {
    let mut min_dist = man_dist(pts[0], *pt);
    let mut min_idx_vec = vec![0];
    for (i, p) in pts.iter().enumerate() {
        let dist = man_dist(*p, *pt);
        match dist.cmp(&min_dist) {
            Ordering::Less => {
                min_dist = dist;
                min_idx_vec = vec![i];
            }
            Ordering::Equal => min_idx_vec.push(i),
            _ => (),
        }
    }
    min_idx_vec
}

pub fn make_map(pts: &[(i32, i32)]) -> HashMap<(i32, i32), Option<usize>> {
    // Returns hashmap with grid of points and value index of pt closest
    let mut output = HashMap::new();
    let (xmin, xmax, ymin, ymax) = grid_limits(pts);
    for x in xmin..xmax + 1 {
        for y in ymin..ymax + 1 {
            let c_idxs = closest_points(&(x, y), pts);
            if c_idxs.len() == 1 {
                output.insert((x, y), Some(c_idxs[0]));
            } else {
                output.insert((x, y), None);
            }
        }
    }
    output
}

fn part2_grid_limits(pts: &[(i32, i32)], limit: i32) -> (i32, i32, i32, i32) {
    // return grid (xmin, xmax, ymin, ymax)
    let mut xmin = pts.iter().map(|p| p.0).min().unwrap();
    let mut xmax = pts.iter().map(|p| p.0).max().unwrap();
    let mut ymin = pts.iter().map(|p| p.1).min().unwrap();
    let mut ymax = pts.iter().map(|p| p.1).max().unwrap();

    let mut xmin_d: i32 = pts.iter().map(|p| (p.0 - xmin).abs()).sum();
    while xmin_d < limit {
        xmin -= 1;
        xmin_d = pts.iter().map(|p| (p.0 - xmin).abs()).sum();
    }
    let mut xmax_d: i32 = pts.iter().map(|p| (p.0 - xmax).abs()).sum();
    while xmax_d < limit {
        xmax += 1;
        xmax_d = pts.iter().map(|p| (p.0 - xmax).abs()).sum();
    }
    let mut ymin_d: i32 = pts.iter().map(|p| (p.1 - ymin).abs()).sum();
    while ymin_d < limit {
        ymin -= 1;
        ymin_d = pts.iter().map(|p| (p.1 - ymin).abs()).sum();
    }
    let mut ymax_d: i32 = pts.iter().map(|p| (p.1 - ymax).abs()).sum();
    while ymax_d < limit {
        ymax += 1;
        ymax_d = pts.iter().map(|p| (p.1 - ymax).abs()).sum();
    }

    (xmin, xmax, ymin, ymax)
}

fn part2_map(pts: &[(i32, i32)], limit: i32) -> HashMap<(i32, i32), i32> {
    let (xmin, xmax, ymin, ymax) = part2_grid_limits(pts, limit);
    let mut output = HashMap::new();
    for x in xmin..xmax {
        for y in ymin..ymax {
            let sum_dist: i32 = pts.iter().map(|p| man_dist(*p, (x, y))).sum();
            output.insert((x, y), sum_dist);
        }
    }
    output
}

#[aoc(day6, part1)]
fn part1(input: &[(i32, i32)]) -> usize {
    let map = make_map(input);
    let (xmin, xmax, ymin, ymax) = grid_limits(input);

    let mut sizes = Vec::new();
    for i in 0..input.len() {
        let mut cnt = 0;
        for (key, val) in map.iter() {
            if let Some(value) = *val {
                if i == value {
                    if key.0 == xmin || key.0 == xmax || key.1 == ymin || key.1 == ymax {
                        // This point has a border and thus is infinite sized
                        cnt = 0;
                        break;
                    }
                    cnt += 1;
                }
            }
        }
        sizes.push(cnt);
    }
    *sizes.iter().max().unwrap() as usize
}

fn part2_dist(input: &[(i32, i32)], limit: i32) -> usize {
    let map = part2_map(input, limit);
    let mut cnt = 0;
    for &val in map.values() {
        if val < limit {
            cnt += 1;
        }
    }
    cnt
}

#[aoc(day6, part2)]
pub fn part2(input: &[(i32, i32)]) -> usize {
    part2_dist(input, 10000)
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
        assert_eq!(part2_dist(&input, 32), 16);
    }
}
