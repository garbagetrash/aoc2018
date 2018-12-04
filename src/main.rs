extern crate clap;

use clap::{App, Arg};

mod day01;
mod day02;
mod day03;
mod day04;

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
            let input = day01::day01::load_input();
            println!("Part 1 Solution: {}", day01::day01::part1(&input));
            println!("Part 2 Solution: {}", day01::day01::part2(&input));
        }
        2 => {
            let input = day02::day02::load_input();
            println!("Part 1 Solution: {}", day02::day02::part1(&input));
            println!("Part 2 Solution: {}", day02::day02::part2(&input));
        }
        3 => {
            let input = day03::day03::load_input();
            println!("Part 1 Solution: {}", day03::day03::part1(&input));
            println!("Part 2 Solution: {}", day03::day03::part2(&input).unwrap());
        }
        4 => {
            let input = day04::day04::load_input("inputs/04.txt");
            println!("Part 1 Solution: {}", day04::day04::part1(&input));
            println!("Part 2 Solution: {}", day04::day04::part2(&input));
        }
        _ => println!("Day {} not yet implemented", day),
    }
}
