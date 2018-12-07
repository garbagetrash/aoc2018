pub mod day06 {
    extern crate regex;

    use regex::Regex;
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
            println!("{}, {}", x, y);
            vec.push((*x, *y));
        }
        vec
    }

    pub fn check_hull(point_vec: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut hull_points = HashSet::new();
        for p1 in point_vec.iter() {
            for p2 in point_vec.iter() {
                if p1 != p2 {
                    let mut m: f64 = 0.0;
                    let mut is_hull = true;
                    if p2.0 - p1.0 != 0 {
                        m = (p2.1 as f64 - p1.1 as f64) / (p2.0 as f64 - p1.0 as f64);
                        let b = p1.1 as f64 - p1.0 as f64 * m;
                        let mut p_above = false;
                        let mut p_below = false;
                        for p3 in point_vec.iter() {
                            if p_above && p_below {
                                is_hull = false;
                                break;
                            }
                            if p3 != p1 && p3 != p2 {
                                if (p3.1 as f64) > m * p3.0 as f64 + b {
                                    p_above = true;
                                } else if (p3.1 as f64) < m * p3.0 as f64 + b {
                                    p_below = true;
                                }
                            }
                        }
                        if p_above && p_below {
                            is_hull = false;
                        }
                    } else {
                        let mut p_above = false;
                        let mut p_below = false;
                        for p3 in point_vec.iter() {
                            if p_above && p_below {
                                is_hull = false;
                                break;
                            }
                            if p3 != p1 && p3 != p2 {
                                if p3.0 > p2.0 {
                                    p_above = true;
                                } else if p3.0 < p2.0 {
                                    p_below = true;
                                }
                            }
                        }
                        if p_above && p_below {
                            is_hull = false;
                        }
                    }
                    if is_hull {
                        hull_points.insert(p1);
                        hull_points.insert(p2);
                    }
                }
            }
        }

        let mut interior_points: Vec<(i32, i32)> = Vec::new();
        for pt in point_vec.iter() {
            if !hull_points.contains(pt) {
                interior_points.push(*pt);
            }
        }
        interior_points
    }

    pub fn closest_point(pt: (i32, i32), pts: &Vec<(i32, i32)>) -> (i32, i32) {
        let mut min_dist = std::i32::MAX;
        let mut min_p = (0, 0);
        for p in pts {
            let mut dist = (pt.0 - p.0).abs();
            dist += (pt.1 - p.1).abs();

            if dist < min_dist {
                min_dist = dist;
                min_p = *p;
            }
        }
        println!("min_p: {:?}, pt: {:?}", min_p, pt);
        min_p
    }

    pub fn count_points(pt: (i32, i32), pts: &Vec<(i32, i32)>) -> usize {
        let mut count = 0;
        for x in 0..500 {
            for y in 0..500 {
                if closest_point((x, y), pts) == pt {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn part1(input: &str) -> usize {
        let mut map = point_vec(input);
        let pts = map.iter().map(|x| *x).collect();
        println!("pts: {:?}", pts);
        let interior_points = check_hull(map);
        println!("Int. Points: {:?}", interior_points);
        let mut cell_sizes = Vec::new();
        for pt in interior_points {
            println!("{:?}", pt);
            cell_sizes.push(count_points(pt, &pts));
        }
        println!("cell_sizes: {:?}", cell_sizes);
        *cell_sizes.iter().max().unwrap()
    }

    pub fn part2(input: &str) -> usize {
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            let mut input = String::new();
            input.push_str("1, 1
                1, 6
                8, 3
                3, 4
                5, 5
                8, 9");
            assert_eq!(part1(&input), 17);
        }

        #[test]
        fn test_part1() {
            assert_eq!(0, 0);
        }

        #[test]
        fn part2examples() {
            assert_eq!(0, 0);
        }

        #[test]
        fn test_part2() {
            assert_eq!(0, 0);
        }
    }
}
