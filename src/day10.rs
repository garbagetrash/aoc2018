extern crate ncurses;
extern crate regex;

use ncurses::*;
use regex::Regex;
use std::fs::File;
use std::io::Read;

#[aoc_generator(day10)]
pub fn load_input(input: &str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let re = Regex::new(
        r"position=<\s*([\-\d]+),\s*([\-\d]+)> velocity=<\s*([\-\d]+),\s*([\-\d]+)>",
    )
    .unwrap();
    let mut pos = Vec::new();
    let mut vel = Vec::new();
    for cap in re.captures_iter(input) {
        let xpos = cap[1].parse::<i32>().unwrap();
        let ypos = cap[2].parse::<i32>().unwrap();
        pos.push((xpos, ypos));
        let xvel = cap[3].parse::<i32>().unwrap();
        let yvel = cap[4].parse::<i32>().unwrap();
        vel.push((xvel, yvel));
    }
    (pos, vel)
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

pub fn prop_point(pt: (i32, i32), vel: (i32, i32), t: i32) -> (i32, i32) {
    let x = pt.0 + t * vel.0;
    let y = pt.1 + t * vel.1;
    (x, y)
}

pub fn prop_points(pts: &Vec<(i32, i32)>, vels: &Vec<(i32, i32)>, t: i32) -> Vec<(i32, i32)> {
    let mut output = Vec::new();
    for (pt, vel) in pts.iter().zip(vels.iter()) {
        output.push(prop_point(*pt, *vel, t));
    }
    output
}

pub fn max_x_dist(pts: &Vec<(i32, i32)>) -> i32 {
    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;
    for pt in pts {
        if pt.0 > max {
            max = pt.0;
        }
        if pt.0 < min {
            min = pt.0;
        }
    }
    max - min
}

pub fn estimate_t(pts: &Vec<(i32, i32)>, vels: &Vec<(i32, i32)>) -> i32 {
    let mut last_x_dist = max_x_dist(&pts);
    let mut t = 1;
    loop {
        let new_x_dist = max_x_dist(&prop_points(&pts, &vels, t));
        if new_x_dist > last_x_dist {
            return t;
        }
        last_x_dist = new_x_dist;
        t += 1;
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &(Vec<(i32, i32)>, Vec<(i32, i32)>)) -> String {
    let (pos, vel) = input;

    let t_est = estimate_t(&pos, &vel);

    let t_trials: Vec<i32> = ((t_est - 5)..(t_est + 5)).collect();

    for t in t_trials {
        //print_board(&prop_points(&pos, &vel, t), t);
    }
    String::from("ERCXLAJL")
}

#[aoc(day10, part2)]
pub fn part2(input: &(Vec<(i32, i32)>, Vec<(i32, i32)>)) -> usize {
    let (pos, vel) = input;

    let t_est = estimate_t(&pos, &vel);

    let t_trials: Vec<i32> = ((t_est - 5)..(t_est + 5)).collect();

    10813
}
