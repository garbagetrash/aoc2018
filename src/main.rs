extern crate clap;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use clap::{App, Arg};

mod day01;

fn main() {
    let matches = App::new("AOC2018")
        .arg(
            Arg::with_name("DAY")
                .required(true)
                .index(1)
                .help("Day to run"),
        )
        .get_matches();

    let day = matches.value_of("DAY").unwrap().parse().unwrap();

    match day {
        1 => {
            let input: Vec<i64> = {
                let f = File::open("inputs/01.txt").unwrap();
                let f = BufReader::new(f);

                f.lines()
                    .map(|x| x.unwrap().parse::<i64>().unwrap())
                    .collect()
            };
            println!("Part 1 Solution: {}", day01::day01::part1(&input));
            println!("Part 2 Solution: {}", day01::day01::part2(&input));
        }
        _ => println!("Day {} not yet implemented", day),
    }
}
