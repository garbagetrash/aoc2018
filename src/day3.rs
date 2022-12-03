use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Rect {
    id: u32,
    space_left: u32,
    space_top: u32,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn new(id: u32, space_left: u32, space_top: u32, width: u32, height: u32) -> Rect {
        Rect {
            id: id,
            space_left: space_left,
            space_top: space_top,
            width: width,
            height: height,
        }
    }
}

#[aoc_generator(day3)]
pub fn load_input(input: &str) -> Vec<Rect> {
    let mut output = Vec::new();
    for line in input.lines() {
        let (id, space_left, space_top, width, height) =
            scan_fmt!(line, "#{} @ {},{}: {}x{}\n", u32, u32, u32, u32, u32).unwrap();
        output.push(Rect::new(id, space_left, space_top, width, height));
    }
    output
}

pub fn intersection(r1: &Rect, r2: &Rect) -> Option<Rect> {
    let left = std::cmp::max(r1.space_left, r2.space_left);
    let right = std::cmp::min(r1.space_left + r1.width, r2.space_left + r2.width);

    if left >= right {
        return None;
    }

    let top = std::cmp::max(r1.space_top, r2.space_top);
    let bottom = std::cmp::min(r1.space_top + r1.height, r2.space_top + r2.height);

    if bottom <= top {
        return None;
    }

    Some(Rect::new(0, left, top, right - left, bottom - top))
}

pub fn set_intersection(r1: &Rect, r2: &Rect, mut set: HashSet<(u32, u32)>) -> HashSet<(u32, u32)> {
    let left = std::cmp::max(r1.space_left, r2.space_left);
    let right = std::cmp::min(r1.space_left + r1.width, r2.space_left + r2.width);

    if left >= right {
        return set;
    }

    let top = std::cmp::max(r1.space_top, r2.space_top);
    let bottom = std::cmp::min(r1.space_top + r1.height, r2.space_top + r2.height);

    if bottom <= top {
        return set;
    }

    for w in left..right {
        for h in top..bottom {
            set.insert((w, h));
        }
    }
    set
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Rect>) -> u32 {
    let mut set: HashSet<_> = HashSet::new();
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            set = set_intersection(&input[i], &input[j], set);
        }
    }
    set.len() as u32
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Rect>) -> u32 {
    let mut claims = HashMap::new();
    for claim in input {
        claims.insert(claim.id, 0);
    }
    for r1 in input {
        for r2 in input {
            if r1.id == r2.id {
                continue;
            }
            if let Some(_) = intersection(&r1, &r2) {
                if let Some(x) = claims.get_mut(&r1.id) {
                    *x += 1;
                }
                if let Some(x) = claims.get_mut(&r2.id) {
                    *x += 1;
                }
            }
        }
    }

    for (id, val) in claims.iter() {
        if *val == 0 {
            return *id;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use std::fs::read_to_string;

    #[test]
    fn test_set_intersection() {
        let mut set = HashSet::new();
        set = set_intersection(&Rect::new(0, 0, 0, 5, 5), &Rect::new(1, 1, 1, 3, 3), set);
        assert_eq!(set.len(), 9);
    }

    #[test]
    fn test_set_intersection2() {
        let mut set = HashSet::new();
        set = set_intersection(&Rect::new(0, 3, 1, 2, 5), &Rect::new(1, 1, 1, 3, 3), set);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_set_intersection3() {
        let mut set = HashSet::new();
        set = set_intersection(&Rect::new(0, 3, 1, 2, 5), &Rect::new(1, 0, 0, 5, 5), set);
        assert_eq!(set.len(), 8);
    }

    #[test]
    fn test_set_intersection4() {
        let mut set = HashSet::new();
        set = set_intersection(&Rect::new(0, 3, 1, 2, 5), &Rect::new(1, 7, 0, 5, 5), set);
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_part1_example() {
        let r1 = Rect::new(1, 1, 3, 4, 4);
        let r2 = Rect::new(2, 3, 1, 4, 4);
        let r3 = Rect::new(3, 5, 5, 2, 2);
        let mut input = Vec::new();
        input.push(r1);
        input.push(r2);
        input.push(r3);
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2018/03.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2018/03.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 3);
    }
}
